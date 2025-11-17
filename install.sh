#!/usr/bin/env bash

set -e

APP_NAME="github-commit-reminder"
USER_NAME=$(whoami)
PROJECT_DIR=$(pwd)
ENV_FILE="$PROJECT_DIR/.env"
SERVICE_FILE="/etc/systemd/system/$APP_NAME.service"
BIN_PATH="/usr/local/bin/$APP_NAME"

echo ""
echo "============================================"
echo "   GitHub Commit Reminder - Installer 🚀"
echo "============================================"
echo ""

# ---------------------------------------------------
# 0. Check if Rust/Cargo is installed
# ---------------------------------------------------
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ Rust/Cargo is not installed."
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# ---------------------------------------------------
# 1. Build the project in release mode
# ---------------------------------------------------
echo "🔨 Building project in release mode..."
cargo build --release

# ---------------------------------------------------
# 2. Install binary to /usr/local/bin
# ---------------------------------------------------
echo "📦 Installing binary to /usr/local/bin..."
sudo cp "$PROJECT_DIR/target/release/$APP_NAME" "$BIN_PATH"
sudo chmod +x "$BIN_PATH"

# ---------------------------------------------------
# 3. Ensure .env exists
# ---------------------------------------------------
if [ ! -f "$ENV_FILE" ]; then
    echo "⚠️ .env file not found. Creating a template..."
    cat <<EOF > "$ENV_FILE"
GITHUB_TOKEN=
GITHUB_USERNAME=

TELEGRAM_BOT_TOKEN=
TELEGRAM_CHAT_ID=

REMINDER_INTERVAL_MINUTES=60
MODE=NORMAL
EOF
    echo "📄 A default .env file has been created. Please fill in the values!"
else
    echo "📄 .env found."
fi

# ---------------------------------------------------
# 4. Create systemd service
# ---------------------------------------------------
echo "📝 Creating systemd service at $SERVICE_FILE..."

sudo bash -c "cat > $SERVICE_FILE" <<EOF
[Unit]
Description=GitHub Commit Reminder (Rust Daemon)
After=network.target

[Service]
ExecStart=$BIN_PATH
WorkingDirectory=$PROJECT_DIR
EnvironmentFile=$ENV_FILE
Restart=always
RestartSec=5
User=$USER_NAME

[Install]
WantedBy=multi-user.target
EOF

# ---------------------------------------------------
# 5. Reload systemd, enable, and start service
# ---------------------------------------------------
echo "🔄 Reloading systemd..."
sudo systemctl daemon-reload

echo "🚀 Enabling service..."
sudo systemctl enable $APP_NAME

echo "▶️ Starting service..."
sudo systemctl start $APP_NAME

echo ""
echo "============================================"
echo "        🎉 INSTALLATION COMPLETE!"
echo "============================================"
echo ""
echo "Service Name : $APP_NAME"
echo "Binary Path  : $BIN_PATH"
echo "Project Dir  : $PROJECT_DIR"
echo "Env File     : $ENV_FILE"
echo ""
echo "Check status:   sudo systemctl status $APP_NAME"
echo "View logs:      journalctl -u $APP_NAME -f"
echo ""