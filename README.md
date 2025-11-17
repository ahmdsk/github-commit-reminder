# 🚀 GitHub Daily Contribution Notifier (Rust)

Sebuah aplikasi ringan berbasis Rust yang berjalan 24/7 untuk **mengecek kontribusi GitHub harian**, dan mengirimkan **notifikasi Telegram** jika kamu **belum melakukan commit pada hari tersebut**.

Aplikasi ini ideal untuk membangun kebiasaan _"commit setiap hari"_ atau menjaga GitHub streak.

---

## ✨ Fitur Utama

- 🔍 **Cek kontribusi GitHub harian** (menggunakan GitHub GraphQL API)
- ⏰ **Jalan otomatis setiap hari jam 21:50**
- 📢 **Mengirim notifikasi Telegram** jika kontribusi hari ini = 0
- 🔁 **Repeat notification** hingga 3 kali  
  (interval 10 menit atau bisa diubah)
- 🌙 **CPU usage sangat rendah**  
  (menggunakan `tokio::sleep_until` → bukan busy loop)
- 🔒 Menggunakan token GitHub aman via `.env`
- ⚡ Written with Rust + Tokio async runtime

---

## 🧠 Cara Kerja

1. Aplikasi start → langsung tidur sampai jam **21:50**.
2. Jam 21:50 → bangun → panggil GitHub GraphQL API.
3. Jika **sudah commit** → selesai hari itu.
4. Jika **belum commit** → kirim notif Telegram #1.
5. Tunggu 10 menit → cek ulang.
6. Ulangi sampai **3 kali** (bisa diubah).
7. Setelah selesai → tidur 24 jam.
8. Besok ulangi lagi.

Semuanya dilakukan dengan event-loop async, sehingga CPU usage = **0% hampir sepanjang hari**.

---

## 🏗️ Teknologi yang Digunakan

- **Rust 2021 Edition**
- **Tokio** — async runtime
- **Reqwest** — HTTP client
- **GitHub GraphQL API**
- **Telegram Bot API**
- **Chrono** — waktu & tanggal
- **dotenvy** — konfigurasi environment
- **Tracing** — logging modern

---

## 📁 Struktur Project

```

src/
├── main.rs
├── github.rs        # GitHub GraphQL API logic
└── telegram.rs      # Telegram bot sender
.env                  # environment variables
Cargo.toml
README.md

```

---

## 🔧 Instalasi

Pastikan Rust sudah terinstall:

```

curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh

```

Clone repo:

```

git clone [https://github.com/your/repo.git](https://github.com/your/repo.git)
cd github-commit-reminder

```

---

## ⚙️ Setup Environment

Buat file `.env`:

```

GITHUB_TOKEN=ghp_xxxxxx
GITHUB_USERNAME=yourusername

TELEGRAM_BOT_TOKEN=xxxxxx
TELEGRAM_CHAT_ID=123456789

```

### Cara mendapatkan `GITHUB_TOKEN`

Masuk GitHub → Settings → Developer Settings → PAT:

- pilih "Fine-grained token"
- beri akses minimal:
  - `read:user`
  - `read:contributions`

### Cara mendapatkan Telegram Bot Token & Chat ID

1. Cari **@BotFather** → buat bot baru
2. Ambil bot token
3. Chat bot kamu
4. Buka:

```

[https://api.telegram.org/botTOKEN/getUpdates](https://api.telegram.org/botTOKEN/getUpdates)

```

5. Ambil `chat.id`

---

## ▶️ Menjalankan Aplikasi

```

cargo run --release

```

Aplikasi akan langsung menunggu sampai jam 21:50.
