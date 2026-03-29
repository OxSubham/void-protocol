#!/bin/bash
set -e

# --- Universal Auto-Detection ---
# This part finds your username and repo name automatically
REMOTE_URL=$(git remote get-url origin 2>/dev/null || echo "")
if [[ $REMOTE_URL == *"github.com"* ]]; then
    REPO=$(echo $REMOTE_URL | sed -E 's/.*github.com[:\/](.*)\.git/\1/')
else
    # Fallback if git isn't init yet (Change this if you change your name)
    REPO="OxSubham/void-protocol"
fi

BINARY_NAME="void-core"
INSTALL_DIR="$HOME/.void"
BIN_DIR="$INSTALL_DIR/bin"

echo "🌌 V.O.I.D. Genesis: Initiating install for $REPO..."

# 1. Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux)  PLATFORM="linux" ;;
    Darwin) PLATFORM="macos" ;;
    *)      echo "❌ Error: V.O.I.D. requires Linux or macOS."; exit 1 ;;
esac

# 2. Setup Directories
mkdir -p "$BIN_DIR"
mkdir -p "$INSTALL_DIR/pulse"

# 3. Get the latest release file dynamically
echo "📡 Contacting the mesh for the latest shards..."
LATEST_TAG=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "tag_name" | cut -d '"' -f 4)

if [ -z "$LATEST_TAG" ]; then
    echo "❌ Error: No release found. Make sure you have published a 'Latest Release' on GitHub."
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$LATEST_TAG/${BINARY_NAME}-${PLATFORM}"

# 4. Download and Install
curl -L "$URL" -o "$BIN_DIR/$BINARY_NAME"
chmod +x "$BIN_DIR/$BINARY_NAME"

# 5. Add to PATH
SHELL_RC="$HOME/.bash_rc"
[[ "$SHELL" == *"zsh"* ]] && SHELL_RC="$HOME/.zshrc" || SHELL_RC="$HOME/.bashrc"

if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$SHELL_RC"
    echo "✅ Added to PATH. Please run: source $SHELL_RC"
fi

echo "🏁 V.O.I.D. Core is ready. Type 'void-core' to start."
