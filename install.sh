#!/usr/bin/env bash

set -e

APP_NAME="csvlive"
REPO="imck037/csvlive"

# Detect OS

OS="$(uname -s)"
ARCH="$(uname -m)"

# Normalize architecture names

case "$ARCH" in
x86_64) ARCH="x86_64" ;;
aarch64) ARCH="arm64" ;;
armv7l) ARCH="armv7" ;;
*) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac


# Get latest version from GitHub

LATEST=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep tag_name | cut -d '"' -f 4)

# File name (match your release naming!)

FILE="${APP_NAME}-linux-${ARCH}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${LATEST}/${FILE}"

echo "📦 Installing $APP_NAME ($LATEST) for $ARCH..."

# Download

curl -L "$URL" -o "$FILE"

# Extract

tar -xzf "$FILE"

# Install

chmod +x "$APP_NAME"
sudo mv "$APP_NAME" /usr/local/bin/

# Cleanup

rm "$FILE"

echo "✅ Installed successfully!"
