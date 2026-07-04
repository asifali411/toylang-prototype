#!/usr/bin/env bash
set -euo pipefail

# ============================================================
REPO="asifali411/toylang-prototype"
BIN_NAME="toylang"
INSTALL_DIR="${TOYLANG_INSTALL_DIR:-$HOME/.local/bin}"
# ============================================================

info()  { printf "\033[1;34m==>\033[0m %s\n" "$1"; }
error() { printf "\033[1;31merror:\033[0m %s\n" "$1" >&2; exit 1; }

# --- detect OS ---
OS="$(uname -s)"
case "$OS" in
  Linux)  ASSET_NAME="${BIN_NAME}-linux" ;;
  Darwin) ASSET_NAME="${BIN_NAME}-macos" ;;
  *) error "Unsupported OS: $OS. Use install.ps1 on Windows." ;;
esac

info "Detected platform: $OS"

# --- resolve latest release download URL ---
API_URL="https://api.github.com/repos/${REPO}/releases/latest"
info "Fetching latest release info..."

DOWNLOAD_URL=$(curl -fsSL "$API_URL" \
  | grep "browser_download_url" \
  | grep "$ASSET_NAME\"" \
  | cut -d '"' -f 4 || true)

if [ -z "${DOWNLOAD_URL:-}" ]; then
  error "Could not find asset '$ASSET_NAME' in the latest release.
Check https://github.com/${REPO}/releases for available downloads."
fi

# --- download ---
TMP_FILE="$(mktemp)"
trap 'rm -f "$TMP_FILE"' EXIT

info "Downloading $ASSET_NAME..."
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_FILE"

# --- install ---
mkdir -p "$INSTALL_DIR"
cp "$TMP_FILE" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

info "Installed $BIN_NAME to $INSTALL_DIR"

# --- PATH check ---
case ":$PATH:" in
  *":$INSTALL_DIR:"*)
    info "$INSTALL_DIR is already on your PATH."
    ;;
  *)
    SHELL_RC=""
    case "$(basename "${SHELL:-}")" in
      bash) SHELL_RC="$HOME/.bashrc" ;;
      zsh)  SHELL_RC="$HOME/.zshrc" ;;
      fish) SHELL_RC="$HOME/.config/fish/config.fish" ;;
    esac

    if [ -n "$SHELL_RC" ]; then
      echo "" >> "$SHELL_RC"
      echo "# Added by ${BIN_NAME} installer" >> "$SHELL_RC"
      echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
      info "Added $INSTALL_DIR to PATH in $SHELL_RC"
      info "Run 'source $SHELL_RC' or restart your terminal to use '$BIN_NAME'."
    else
      info "Add this to your shell profile to use '$BIN_NAME':"
      echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
    fi
    ;;
esac

info "Done! Run '$BIN_NAME --version' (after restarting your shell) to verify."
