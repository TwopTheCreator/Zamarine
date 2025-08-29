#!/bin/bash
SOURCE_FILE="$1"
SERVICE_NAME="$2"
BIN_NAME="${SERVICE_NAME}"

if [[ -z "$SOURCE_FILE" || -z "$SERVICE_NAME" ]]; then
    echo "Usage: $0 <source_file.c> <service_name>"
    exit 1
fi

gcc "$SOURCE_FILE" -o "$BIN_NAME"
if [[ $? -ne 0 ]]; then
    echo "Compilation failed!"
    exit 1
fi
echo "Compilation successful: $BIN_NAME"

sudo mv "$BIN_NAME" /usr/local/bin/
sudo chmod +x /usr/local/bin/"$BIN_NAME"
echo "Binary moved to /usr/local/bin and made executable."

SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
sudo bash -c "cat > $SERVICE_FILE <<EOL
[Unit]
Description=$SERVICE_NAME Service
After=network.target

[Service]
ExecStart=/usr/local/bin/$BIN_NAME
Restart=always

[Install]
WantedBy=multi-user.target
EOL"

sudo systemctl daemon-reload
sudo systemctl enable "$SERVICE_NAME"
sudo systemctl start "$SERVICE_NAME"

echo "System service $SERVICE_NAME created and started."
