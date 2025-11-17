mod github;
mod telegram;

use chrono::{Local, Timelike};
use dotenvy::dotenv;
use std::env;
use tokio::time::{Duration, Instant, sleep_until};
use tracing::{error, info};

const NOTIF_MAX: u32 = 3; // jumlah maksimal notif
const NOTIF_INTERVAL: u64 = 600; // 10 menit (600 detik)

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let github_token = env::var("GITHUB_TOKEN").expect("Need GITHUB_TOKEN");
    let github_user = env::var("GITHUB_USERNAME").expect("Need GITHUB_USERNAME");

    let tg_token = env::var("TELEGRAM_BOT_TOKEN").expect("Need TELEGRAM_BOT_TOKEN");
    let tg_chat = env::var("TELEGRAM_CHAT_ID").expect("Need TELEGRAM_CHAT_ID");

    let client = reqwest::Client::new();

    loop {
        // Waktu target harian
        let now = Local::now();
        let target = now
            .with_hour(21)
            .unwrap()
            .with_minute(50)
            .unwrap()
            .with_second(0)
            .unwrap();

        let wait = (target - now).to_std().unwrap_or(Duration::from_secs(0));

        info!("⏳ Menunggu hingga {}", target);
        sleep_until(Instant::now() + wait).await;

        let today = Local::now().format("%Y-%m-%d").to_string();
        info!("🔍 Mulai cek kontribusi hari ini: {}", today);

        let mut attempt = 0;

        loop {
            let count = github::get_today_contribution(&client, &github_token, &github_user)
                .await
                .unwrap_or(0);

            if count > 0 {
                info!("✔ Kamu sudah commit hari ini, total kontribusi: {}", count);
                break; // stop repeat
            }

            attempt += 1;

            // kirim notif
            let msg = format!(
                "⚠️ Kamu belum commit hari ini ({}).\nPercobaan ke: {}/{}",
                today, attempt, NOTIF_MAX
            );

            match telegram::send_telegram(&client, &tg_token, &tg_chat, &msg).await {
                Ok(_) => info!("📨 Notifikasi Telegram #{} terkirim.", attempt),
                Err(e) => error!("Gagal kirim notif: {:?}", e),
            }

            if attempt >= NOTIF_MAX {
                info!("⛔ Batas notif hari ini tercapai ({})", NOTIF_MAX);
                break;
            }

            // tunggu 10 menit sebelum cek ulang
            info!("😴 Menunggu {} detik sebelum cek ulang...", NOTIF_INTERVAL);
            sleep_until(Instant::now() + Duration::from_secs(NOTIF_INTERVAL)).await;
        }

        // tidur 24 jam sebelum cek ulang besok
        info!("😴 Selesai hari ini. Tidur 24 jam...");
        sleep_until(Instant::now() + Duration::from_secs(24 * 3600)).await;
    }
}
