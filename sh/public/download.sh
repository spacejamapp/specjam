#!/bin/bash
set -e

VERSION="0.6.7"

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$OS" in
    "darwin") OS="macos" ;;
    "linux")  OS="linux" ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

# Detect architecture
ARCH="$(uname -m)"
case "$ARCH" in
    "x86_64"|"amd64") ARCH="amd64" ;;
    "arm64"|"aarch64") ARCH="arm64" ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Compose download URL
DOWNLOAD_URL="https://github.com/spacejamapp/specjam/releases/download/${VERSION}/spacejam-${VERSION}-${OS}-${ARCH}.tar.gz"
TEMP_DIR=$(mktemp -d)
ARCHIVE="$TEMP_DIR/spacejam.tar.gz"

echo "Downloading spacejam at graypaper v${VERSION} for ${OS}-${ARCH}..."
echo "URL: $DOWNLOAD_URL"

# Download the archive
if command -v curl > /dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "$ARCHIVE"
elif command -v wget > /dev/null 2>&1; then
    wget -q "$DOWNLOAD_URL" -O "$ARCHIVE"
else
    echo "Error: neither curl nor wget found"
    exit 1
fi

# Extract the archive
tar -xzf "$ARCHIVE" -C "$TEMP_DIR"

# Find the binary and move it to the appropriate location
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

mv "$TEMP_DIR"/spacejam "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/spacejam"

# Cleanup
rm -rf "$TEMP_DIR"

echo "Successfully installed spacejam to $INSTALL_DIR/spacejam"
echo ""
$INSTALL_DIR/spacejam --version
$INSTALL_DIR/spacejam -g