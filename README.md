# GitHub Daily Commit Reminder 🚀

A lightweight Rust-based daemon that reminds you (via Telegram Bot) whether you have contributed to GitHub today.  
It checks your GitHub contributions every X minutes/hours and sends a notification:

- ⚠️ When you **haven’t committed anything today**
- ✔️ When you **already made contributions**
- 🧪 Includes TEST MODE for easy debugging
- 💤 Designed with zero CPU usage during idle state

Built using:

- Rust + Tokio (async executor)
- GitHub GraphQL API
- Telegram Bot API
- Reqwest HTTP client
- Tracing logger

---

## ✨ Features

- 🔃 **Interval-based checking** (minutes or hours)
- ☑️ **Daily contribution check using GitHub GraphQL**
- 📢 **Telegram notifications**
- ⚡ **Low CPU usage** (sleep-based async loop)
- 🧪 **Test mode** available
- 🛠️ Easy to extend & open for contribution

---

## 📦 Requirements

### 1. Create a Telegram Bot

- Open Telegram → search `@BotFather`
- Use command `/newbot`
- Save the given **BOT TOKEN**
- Use `https://api.telegram.org/botTOKEN/getUpdates`  
  to get your **chat ID**

### 2. Create GitHub Token

Go to: https://github.com/settings/tokens  
Create a PAT (Personal Access Token) with:

- `read:user`
- `read:org`

---

## 🔧 Installation

Clone the project:

```bash
git clone https://github.com/yourname/github-commit-reminder
cd github-commit-reminder
```

Install Rust (if needed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## ⚙️ Environment Variables

Create a `.env` file:

```env
GITHUB_TOKEN=ghp_xxxxxxxxxx
GITHUB_USERNAME=ahmdsk

TELEGRAM_BOT_TOKEN=123456:ABCDEF
TELEGRAM_CHAT_ID=123456789

# Remind every 60 minutes (default)
REMINDER_INTERVAL_MINUTES=60

# OR use hours instead (fallback)
# REMINDER_INTERVAL_HOURS=2

# Test mode (send 1 message then exit)
MODE=NORMAL
```

---

## ▶️ Running

### Development mode

```bash
cargo run
```

### Production mode

```bash
cargo build --release
./target/release/github-commit-reminder
```

---

## 🧪 Testing Notification

Before running as daemon, you can test:

```env
MODE=TEST
```

Run:

```bash
cargo run
```

It will send:

```
🧪 TEST MODE → Notifikasi dari GitHub Reminder!
```

Then exit automatically.

---

## 📘 How It Works (Flow Overview)

```
load .env → init logger → init HTTP client
                  ↓
if MODE=TEST → kirim notif → exit
                  ↓
loop:
    cek kontribusi GitHub hari ini
    |
    ├─ jika 0 → kirim notif "Belum ada kontribusi"
    └─ jika >0 → kirim notif "Sudah kontribusi"
    |
    sleep (interval) → repeat
```

---

## 🧩 Project Structure

```
src/
 ├── main.rs          # Program entry, loop utama
 ├── github.rs        # Github GraphQL API
 ├── telegram.rs      # Telegram Bot sender
 └── utils.rs         # (optional future utilities)
.env.example
README.md
Cargo.toml
```

---

## 🛠️ Contributing

Contributions are welcome!  
You can help with:

- improving error handling
- adding multi-user support
- adding daily summary mode
- adding streak tracking
- improving logging
- optimizing GraphQL queries

### Steps:

1. Fork project
2. Create new branch
3. Make changes
4. Submit pull request

Please keep contributions clean and idiomatic.


---

## 📜 License

MIT License  
Feel free to use and modify.
