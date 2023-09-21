use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use tokio;

pub async fn send_text(text: &str, conversation_id: &str, base_url: &str, session: &str, session_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let info = "core_message_send_messages_to_conversation";
    let url = format!("https://{}/lib/ajax/service.php?sesskey={}&info={}", base_url, session_token, info);

    let mut headers = HeaderMap::new();
    headers.insert("authority", HeaderValue::from_str(base_url)?);
    headers.insert("method", HeaderValue::from_static("POST"));
    headers.insert("path", HeaderValue::from_str(&format!("/lib/ajax/service.php?sesskey={}&info={}", session_token, info))?);
    headers.insert("scheme", HeaderValue::from_static("https"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Cookie", HeaderValue::from_str(&format!("MoodleSession={};", session))?);
    headers.insert("Origin", HeaderValue::from_str(&format!("https://{}", base_url))?);
    headers.insert("Referer", HeaderValue::from_str(&format!("https://{}/my/", base_url))?);

    let json = json!([{"index":0,"methodname":info,"args":{"conversationid":conversation_id,"messages":[{"text":text}]}}]);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client.post(&url)
        .json(&json)
        .send()
        .await?;

    println!("{}", res.status());
    println!("{}", res.text().await?);

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..10000 {
        send_text("Hallo Vincent :)", "12697", "moodle.jobelmannschule.de", "", "").await?;
    }
    Ok(())
}
