use anyhow::Result;
use reqwest::Client;

pub async fn send_telegram(client: &Client, token: &str, chat_id: &str, text: &str) -> Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    client
        .post(url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": text
        }))
        .send()
        .await?;

    Ok(())
}
