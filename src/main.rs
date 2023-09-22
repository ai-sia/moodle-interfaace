use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE}, Client};
use serde_json::json;
use tokio;
use dotenv;
use std::sync::Arc;
use std::env;

pub fn build_client(text: &str, conversation_id: &str, base_url: &str, moodle_session: &str, session_key: &str) -> Result<(Arc<Client>,String, serde_json::Value), Box<dyn std::error::Error>> {
    let info = "core_message_send_messages_to_conversation";
    let url = format!("https://{}/lib/ajax/service.php?sesskey={}&info={}", base_url, session_key, info);

    let mut headers = HeaderMap::new();
    headers.insert("authority", HeaderValue::from_str(base_url)?);
    headers.insert("method", HeaderValue::from_static("POST"));
    headers.insert("path", HeaderValue::from_str(&format!("/lib/ajax/service.php?sesskey={}&info={}", session_key, info))?);
    headers.insert("scheme", HeaderValue::from_static("https"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Cookie", HeaderValue::from_str(&format!("MoodleSession={};", moodle_session))?);
    headers.insert("Origin", HeaderValue::from_str(&format!("https://{}", base_url))?);
    headers.insert("Referer", HeaderValue::from_str(&format!("https://{}/my/", base_url))?);

    let json = json!([{"index":0,"methodname":info,"args":{"conversationid":conversation_id,"messages":[{"text":text}]}}]);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let client = Arc::new(client);
    return Ok((client, url, json));
}

pub async fn send_text(client: Arc<Client>, url: String, json: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    
    let res = client.as_ref().post(&url)
        .json(&json)
        .send()
        .await?;

    println!("{}", res.status());
    println!("{}", res.text().await?);

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Could not load .env");
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Not enough arguments. Usage: {} text conver_id base_url sess_key session_token", args[0]);
        return Ok(());
    }

    let text = &args[1];
    let conver_id = &args[2];
    let base_url = "moodle.jobelmannschule.de";
    let sess_key = &args[3];
    let session_token = &args[4];

    
    let (client, url, json) = build_client(text, conver_id, base_url, session_token, sess_key)?;
    let data = Arc::new((client, url, json));

    let mut handles = Vec::new();


    for _ in 0..10 {
        let data_clone = Arc::clone(&data);
        
        let handle = tokio::task::spawn(async move {
            for _ in 0..10 {
                let (client, url, json) = &*data_clone;
                send_text(client.clone(), url.clone(), json.clone()).await.unwrap();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}
