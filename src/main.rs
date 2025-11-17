mod github;
mod telegram;

use chrono::Local;
use dotenvy::dotenv;
use std::env;
use tokio::time::{Duration, sleep};
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let github_token = env::var("GITHUB_TOKEN").expect("Need GITHUB_TOKEN");
    let github_user = env::var("GITHUB_USERNAME").expect("Need GITHUB_USERNAME");

    let tg_token = env::var("TELEGRAM_BOT_TOKEN").expect("Need TELEGRAM_BOT_TOKEN");
    let tg_chat = env::var("TELEGRAM_CHAT_ID").expect("Need TELEGRAM_CHAT_ID");

    // interval minutes OR hours (fallback)
    let interval_minutes: u64 = env::var("REMINDER_INTERVAL_MINUTES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| {
            env::var("REMINDER_INTERVAL_HOURS")
                .unwrap_or("2".into())
                .parse::<u64>()
                .unwrap_or(2)
                * 60
        });

    let client = reqwest::Client::new();

    // TEST MODE
    if env::var("MODE").unwrap_or("NORMAL".into()) == "TEST" {
        telegram::send_telegram(
            &client,
            &tg_token,
            &tg_chat,
            "🧪 TEST MODE → Notifikasi dari GitHub Reminder!",
        )
        .await
        .unwrap();

        println!("📩 Test notif terkirim.");
        return;
    }

    info!("⏳ Interval reminder: {} menit", interval_minutes);

    loop {
        let today = Local::now().format("%Y-%m-%d").to_string();

        info!("🔍 Checking GitHub contribution for {}", today);

        let count = github::get_today_contribution(&client, &github_token, &github_user)
            .await
            .unwrap_or(0);

        if count == 0 {
            let msg = format!(
                "⚠️ Reminder: Kamu BELUM commit hari ini ({}). Ayo push 1x!",
                today
            );
            telegram::send_telegram(&client, &tg_token, &tg_chat, &msg)
                .await
                .unwrap();
        } else {
            info!("✔ Kamu sudah commit hari ini ({} kontribusi).", count);
        }

        // sleep agar CPU tetap 0%
        info!("😴 Sleep {} menit...", interval_minutes);
        sleep(Duration::from_secs(interval_minutes * 60)).await;
    }
}
