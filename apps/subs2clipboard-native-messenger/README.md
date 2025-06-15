# Browser Native Messenger

This is a cross-platform native messaging host for browser extensions that allows copying text to the system clipboard.

## Features

- Receives messages from browser extensions (Chrome, Firefox, Edge)
- Copies received text to the system clipboard
- Cross-platform support (Windows, macOS, Linux)
- Error handling and logging

## Building

To build the application, run:

```bash
cargo build --release
```

## Installation

### Automatic Installation

#### Linux/macOS

1. Update the extension IDs in the `install_manifests.sh` script
2. Make the script executable (if not already): `chmod +x install_manifests.sh`
3. Run the script: `./install_manifests.sh`

#### Windows

1. Update the extension IDs in the `install_manifests.ps1` script
2. Run the script in PowerShell: `.\install_manifests.ps1`

### Manual Installation

You need to register the native messaging host with your browser:

1. Build the application with `cargo build --release`
2. Copy `manifest/manifest-chrome.json` (for Chrome/Edge) or `manifest/manifest-firefox.json` (for Firefox) to the appropriate location for your browser
3. Edit the manifest to include:
   - The correct path to the executable
   - Your browser extension ID

Manifest locations:

#### Chrome/Chromium:
- Windows: `%LOCALAPPDATA%\Google\Chrome\User Data\NativeMessagingHosts\`
- macOS: `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/`
- Linux: `~/.config/google-chrome/NativeMessagingHosts/`

#### Firefox:
- Windows: `%APPDATA%\Mozilla\NativeMessagingHosts\`
- macOS: `~/Library/Application Support/Mozilla/NativeMessagingHosts/`
- Linux: `~/.mozilla/native-messaging-hosts/`

## Usage in Extensions

Your browser extension needs to connect to the native application:

### Chrome/Edge

```javascript
const port = chrome.runtime.connectNative('subs2clipboard-native-messenger');

port.onMessage.addListener((message) => {
  console.log('Received:', message);
});

port.onDisconnect.addListener(() => {
  console.log('Disconnected');
});

// Send text to be copied to clipboard
port.postMessage({ text: 'Text to copy to clipboard' });
```

### Firefox

```javascript
const port = browser.runtime.connectNative('subs2clipboard-native-messenger');

port.onMessage.addListener((message) => {
  console.log('Received:', message);
});

port.onDisconnect.addListener(() => {
  console.log('Disconnected');
});

// Send text to be copied to clipboard
port.postMessage({ text: 'Text to copy to clipboard' });
```

## Troubleshooting

If you encounter issues:

1. Check the extension's console for error messages
2. Check the log file located at:
   - Linux/macOS: `/tmp/browser_native_messenger.log`
   - Windows: `%TEMP%\browser_native_messenger.log`
3. Run the host manually with `./subs2clipboard-native-messenger` for detailed output
4. Verify that the manifest paths are correct and the extension IDs match
4. Ensure the executable has proper permissions (especially on Linux/macOS)
