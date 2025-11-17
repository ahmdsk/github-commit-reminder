use anyhow::Result;
use reqwest::Client;
use tracing::{error, info};

pub async fn send_telegram(client: &Client, token: &str, chat_id: &str, text: &str) -> Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    info!("📡 REQUEST → Telegram: {}", text);

    let raw = client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": text
        }))
        .send()
        .await;

    match raw {
        Ok(res) => {
            let body = res.text().await.unwrap_or_default();
            info!("📥 RESPONSE ← Telegram: {}", body);
        }
        Err(e) => {
            error!("❌ Telegram error: {:?}", e);
        }
    }

    Ok(())
}
