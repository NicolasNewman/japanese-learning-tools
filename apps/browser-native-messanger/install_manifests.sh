#!/bin/bash

# This script installs the native messaging host manifest for both Chrome/Chromium and Firefox

set -e

# Get the directory of the script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MANIFEST_DIR="$SCRIPT_DIR/manifest"

# Path to the executable - determine the release binary path
EXECUTABLE_PATH="$SCRIPT_DIR/../../target/release/browser-native-messenger"

# Chrome/Chromium extension ID
CHROME_EXTENSION_ID="YOUR_CHROME_EXTENSION_ID"

# Firefox extension ID
FIREFOX_EXTENSION_ID="YOUR_FIREFOX_EXTENSION_ID@mozilla.org"

# Detect operating system
OS="$(uname)"

# Set installation paths based on OS
if [ "$OS" == "Darwin" ]; then
    # macOS paths
    CHROME_TARGET_DIR="$HOME/Library/Application Support/Google/Chrome/NativeMessagingHosts"
    CHROMIUM_TARGET_DIR="$HOME/Library/Application Support/Chromium/NativeMessagingHosts"
    FIREFOX_TARGET_DIR="$HOME/Library/Application Support/Mozilla/NativeMessagingHosts"
    
    # Create directories if they don't exist
    mkdir -p "$CHROME_TARGET_DIR"
    mkdir -p "$CHROMIUM_TARGET_DIR"
    mkdir -p "$FIREFOX_TARGET_DIR"
elif [ "$OS" == "Linux" ]; then
    # Linux paths
    CHROME_TARGET_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"
    CHROMIUM_TARGET_DIR="$HOME/.config/chromium/NativeMessagingHosts"
    FIREFOX_TARGET_DIR="$HOME/.mozilla/native-messaging-hosts"
    
    # Create directories if they don't exist
    mkdir -p "$CHROME_TARGET_DIR"
    mkdir -p "$CHROMIUM_TARGET_DIR"
    mkdir -p "$FIREFOX_TARGET_DIR"
else
    echo "Unsupported operating system: $OS"
    echo "Please use the Windows PowerShell script for Windows."
    exit 1
fi

# Check if executable exists
if [ ! -f "$EXECUTABLE_PATH" ]; then
    echo "Executable not found at: $EXECUTABLE_PATH"
    echo "Please build the project first with 'cargo build --release'"
    exit 1
fi

# Make executable
chmod +x "$EXECUTABLE_PATH"

# Install Chrome/Chromium manifest
echo "Installing Chrome/Chromium manifest..."
TEMP_CHROME_MANIFEST=$(mktemp)
cat "$MANIFEST_DIR/manifest-chrome.json" | \
  sed "s|PATH_TO_EXECUTABLE|$EXECUTABLE_PATH|g" | \
  sed "s|YOUR_CHROME_EXTENSION_ID|$CHROME_EXTENSION_ID|g" | \
  sed "s|YOUR_FIREFOX_EXTENSION_ID|$FIREFOX_EXTENSION_ID|g" \
  > "$TEMP_CHROME_MANIFEST"

cp "$TEMP_CHROME_MANIFEST" "$CHROME_TARGET_DIR/browser-native-messanger.json"
cp "$TEMP_CHROME_MANIFEST" "$CHROMIUM_TARGET_DIR/browser-native-messanger.json"
rm "$TEMP_CHROME_MANIFEST"

# Install Firefox manifest
echo "Installing Firefox manifest..."
TEMP_FIREFOX_MANIFEST=$(mktemp)
cat "$MANIFEST_DIR/manifest-firefox.json" | \
  sed "s|PATH_TO_EXECUTABLE|$EXECUTABLE_PATH|g" | \
  sed "s|YOUR_FIREFOX_EXTENSION_ID|$FIREFOX_EXTENSION_ID|g" \
  > "$TEMP_FIREFOX_MANIFEST"

cp "$TEMP_FIREFOX_MANIFEST" "$FIREFOX_TARGET_DIR/browser-native-messanger.json"
rm "$TEMP_FIREFOX_MANIFEST"

echo "Installation complete."
echo ""
echo "Chrome/Chromium manifest installed at: $CHROME_TARGET_DIR/browser-native-messanger.json"
echo "Firefox manifest installed at: $FIREFOX_TARGET_DIR/browser-native-messanger.json"
echo ""
echo "Don't forget to update the extension IDs in this script before running it."
