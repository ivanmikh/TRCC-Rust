#!/bin/bash
set -e

CONFIG_DIR="$HOME/.config/trcc"
SERVICE_DIR="$HOME/.config/systemd/user"

# Install binary
cargo install --path .

# Install config
mkdir -p "$CONFIG_DIR"
if [ ! -f "$CONFIG_DIR/config.toml" ]; then
    cp config.toml "$CONFIG_DIR/"
    echo "Installed config to $CONFIG_DIR/config.toml"
else
    echo "Config already exists, skipping"
fi

# Install systemd service
mkdir -p "$SERVICE_DIR"
cp trcc.service "$SERVICE_DIR/"
systemctl --user daemon-reload
systemctl --user enable trcc

echo "Done. Start with: systemctl --user start trcc"
