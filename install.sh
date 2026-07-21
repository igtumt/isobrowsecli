#!/bin/sh
set -e

REPO="igtumt/isobrowsecli"
BINARY_NAME="iso"

echo "🔍 System architecture is being detected..."

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64) ARTIFACT="iso-linux-amd64" ;;
      *) echo "❌ Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  Darwin)
    case "$ARCH" in
      arm64|aarch64) ARTIFACT="iso-macos-arm64" ;;
      x86_64) ARTIFACT="iso-macos-x86_64" ;;
      *) echo "❌ Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "❌ Unsupported OS: $OS"
    exit 1
    ;;
esac

DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ARTIFACT}"
INSTALL_DIR="/usr/local/bin"

echo "📥 Downloading ${ARTIFACT} from GitHub Releases..."
curl -fsSL "$DOWNLOAD_URL" -o "$BINARY_NAME"

chmod +x "$BINARY_NAME"

echo "⚙️ Installing $BINARY_NAME to $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
  mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
else
  sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
fi

echo "✅ Installation complete! Run 'iso' in your terminal."
