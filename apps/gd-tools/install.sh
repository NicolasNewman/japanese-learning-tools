#!/bin/bash
set -e

# gd-tools installer script
echo "Installing gd-tools..."

# Build the project
echo "Building project..."
cargo build --release

# Create installation directories
INSTALL_DIR="$HOME/.local"
BIN_DIR="$INSTALL_DIR/bin"
RESOURCE_DIR="$INSTALL_DIR/share/gd-tools"

mkdir -p "$BIN_DIR"
mkdir -p "$RESOURCE_DIR"

# Copy the executable
echo "Installing executable to $BIN_DIR..."
cp target/release/gd-tools "$BIN_DIR/"

# Create symlinks for individual tools
cd "$BIN_DIR"
for tool in ankisearch massif images translate marisa mecab strokeorder handwritten; do
    ln -sf gd-tools "gd-$tool"
    echo "Created symlink for gd-$tool"
done

# Copy resource files if they exist
echo "Installing resource files..."
if [ -d "gd-tools/res" ]; then
    cp -r gd-tools/res/* "$RESOURCE_DIR/"
    echo "Installed resource files to $RESOURCE_DIR"
fi

# Create desktop file for integration
DESKTOP_DIR="$HOME/.local/share/applications"
mkdir -p "$DESKTOP_DIR"

cat > "$DESKTOP_DIR/gd-tools.desktop" << EOF
[Desktop Entry]
Name=GD Tools
GenericName=GoldenDict Enhancement Tools
Comment=Tools to enhance GoldenDict for immersion learning
Exec=$BIN_DIR/gd-tools
Icon=dictionary
Terminal=false
Type=Application
Categories=Education;Languages;
EOF

echo "Created desktop file at $DESKTOP_DIR/gd-tools.desktop"

echo "Installation complete!"
echo "Make sure $BIN_DIR is in your PATH."

if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo "Warning: $BIN_DIR is not in your PATH."
    echo "Add the following line to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"\$PATH:$BIN_DIR\""
fi

echo "You can now use gd-tools by running 'gd-tools' or any of the tool-specific commands like 'gd-ankisearch'."
