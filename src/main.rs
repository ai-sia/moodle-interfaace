use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE}, Client};
use serde_json::json;
use tokio;

pub fn build_headers(base_url: &str, moodle_session: &str, session_key: &str) -> Result<HeaderMap, reqwest::Error> {
    let info = "core_message_send_messages_to_conversation";
    let mut headers = HeaderMap::new();
    headers.insert("authority", HeaderValue::from_str(base_url)?);
    headers.insert("method", HeaderValue::from_static("POST"));
    headers.insert("path", HeaderValue::from_str(&format!("/lib/ajax/service.php?sesskey={}&info={}", session_key, info))?);
    headers.insert("scheme", HeaderValue::from_static("https"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Cookie", HeaderValue::from_str(&format!("MoodleSession={};", moodle_session))?);
    headers.insert("Origin", HeaderValue::from_str(&format!("https://{}", base_url))?);
    headers.insert("Referer", HeaderValue::from_str(&format!("https://{}/my/", base_url))?);
    Ok(headers)
}

pub async fn send_text(client: &Client, url: &str, json: &serde_json::Value) -> Result<(), reqwest::Error> {
    let res = client.post(url)
        .json(json)
        .send()
        .await?;

    println!("{}", res.status());
    println!("{}", res.text().await?);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "";
    let conversation_id = "";
    let base_url = "moodle.jobelmannschule.de";
    let session_token = "";
    let sess_key = "";

    let info = "core_message_send_messages_to_conversation";
    let url = format!("https://{}/lib/ajax/service.php?sesskey={}&info={}", base_url, sess_key, info);
    let json = json!([{"index":0,"methodname":info,"args":{"conversationid":conversation_id,"messages":[{"text":text}]}}]);

    let headers = build_headers(base_url, session_token, sess_key)?;
    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let num_tasks = 1000;
    let messages_per_task = 10;

    let mut handles = Vec::new();

    for _ in 0..num_tasks {
        let handle = tokio::task::spawn(async move {
            for _ in 0..messages_per_task {
                send_text(&client, &url, &json).await.unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}
