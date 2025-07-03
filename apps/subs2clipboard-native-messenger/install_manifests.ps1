# This script installs the native messaging host manifest for both Chrome/Chromium and Firefox on Windows

# Get the directory of the script
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ManifestDir = "$ScriptDir\manifest"

# Path to the executable - determine the release binary path
$ExecutablePath = "$ScriptDir\..\..\target\release\subs2clipboard-native-messenger.exe"

# Replace backslashes with forward slashes (JSON format)
$ExecutablePath = $ExecutablePath -replace "\\", "/"

# Chrome/Chromium extension ID
$ChromeExtensionId = "YOUR_CHROME_EXTENSION_ID"

# Firefox extension ID
$FirefoxExtensionId = "YOUR_FIREFOX_EXTENSION_ID@mozilla.org"

# Set installation paths
$ChromeTargetDir = "$env:LOCALAPPDATA\Google\Chrome\User Data\NativeMessagingHosts"
$EdgeTargetDir = "$env:LOCALAPPDATA\Microsoft\Edge\User Data\NativeMessagingHosts"
$FirefoxTargetDir = "$env:APPDATA\Mozilla\NativeMessagingHosts"

# Create directories if they don't exist
if (-not (Test-Path $ChromeTargetDir)) {
    New-Item -ItemType Directory -Path $ChromeTargetDir -Force | Out-Null
}
if (-not (Test-Path $EdgeTargetDir)) {
    New-Item -ItemType Directory -Path $EdgeTargetDir -Force | Out-Null
}
if (-not (Test-Path $FirefoxTargetDir)) {
    New-Item -ItemType Directory -Path $FirefoxTargetDir -Force | Out-Null
}

# Check if executable exists
if (-not (Test-Path $ExecutablePath)) {
    Write-Host "Executable not found at: $ExecutablePath"
    Write-Host "Please build the project first with 'cargo build --release'"
    exit 1
}

# Install Chrome/Edge manifest
Write-Host "Installing Chrome/Edge manifest..."
$ChromeManifestContent = Get-Content -Raw "$ManifestDir\manifest-chrome.json" | 
    ForEach-Object { 
        $_ -replace "PATH_TO_EXECUTABLE", $ExecutablePath `
           -replace "YOUR_CHROME_EXTENSION_ID", $ChromeExtensionId `
           -replace "YOUR_FIREFOX_EXTENSION_ID", $FirefoxExtensionId
    }

$ChromeManifestContent | Set-Content -Path "$ChromeTargetDir\subs2clipboard-native-messenger.json" -Encoding UTF8
$ChromeManifestContent | Set-Content -Path "$EdgeTargetDir\subs2clipboard-native-messenger.json" -Encoding UTF8

# Install Firefox manifest
Write-Host "Installing Firefox manifest..."
$FirefoxManifestContent = Get-Content -Raw "$ManifestDir\manifest-firefox.json" | 
    ForEach-Object { 
        $_ -replace "PATH_TO_EXECUTABLE", $ExecutablePath `
           -replace "YOUR_FIREFOX_EXTENSION_ID", $FirefoxExtensionId
    }

$FirefoxManifestContent | Set-Content -Path "$FirefoxTargetDir\subs2clipboard-native-messenger.json" -Encoding UTF8

Write-Host "Installation complete."
Write-Host ""
Write-Host "Chrome manifest installed at: $ChromeTargetDir\subs2clipboard-native-messenger.json"
Write-Host "Edge manifest installed at: $EdgeTargetDir\subs2clipboard-native-messenger.json"
Write-Host "Firefox manifest installed at: $FirefoxTargetDir\subs2clipboard-native-messenger.json"
Write-Host ""
Write-Host "Don't forget to update the extension IDs in this script before running it."
