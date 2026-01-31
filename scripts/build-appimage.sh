#!/bin/bash
# SecureBeam AppImage Build Script
# This script ensures all dependencies are installed for AppImage bundling

set -e

echo "=== SecureBeam AppImage Builder ==="

# Check if running on Linux
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo "Error: This script must be run on Linux"
    exit 1
fi

# Install required dependencies
echo "Installing dependencies..."
if command -v apt-get &> /dev/null; then
    sudo apt-get update
    sudo apt-get install -y \
        libwebkit2gtk-4.1-dev \
        libappindicator3-dev \
        librsvg2-dev \
        patchelf \
        libfuse2 \
        file
elif command -v dnf &> /dev/null; then
    sudo dnf install -y \
        webkit2gtk4.1-devel \
        libappindicator-gtk3-devel \
        librsvg2-devel \
        patchelf \
        fuse-libs \
        file
elif command -v pacman &> /dev/null; then
    sudo pacman -Sy --noconfirm \
        webkit2gtk-4.1 \
        libappindicator-gtk3 \
        librsvg \
        patchelf \
        fuse2 \
        file
fi

# Download linuxdeploy if not present
LINUXDEPLOY_URL="https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage"
LINUXDEPLOY_PATH="$HOME/.local/bin/linuxdeploy"

if [ ! -f "$LINUXDEPLOY_PATH" ]; then
    echo "Downloading linuxdeploy..."
    mkdir -p "$HOME/.local/bin"
    curl -L "$LINUXDEPLOY_URL" -o "$LINUXDEPLOY_PATH"
    chmod +x "$LINUXDEPLOY_PATH"
fi

# Add to PATH if not already
export PATH="$HOME/.local/bin:$PATH"

# Navigate to client directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/../client"

# Install frontend dependencies
echo "Installing frontend dependencies..."
yarn install

# Build Tauri
echo "Building SecureBeam..."
yarn tauri build

echo ""
echo "=== Build Complete ==="
echo "Packages available at:"
echo "  - DEB: client/src-tauri/target/release/bundle/deb/"
echo "  - RPM: client/src-tauri/target/release/bundle/rpm/"
echo "  - AppImage: client/src-tauri/target/release/bundle/appimage/"
