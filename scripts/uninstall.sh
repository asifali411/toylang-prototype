#!/usr/bin/env bash
set -euo pipefail
# ============================================================
BIN_NAME="toylang"
INSTALL_DIR="${TOYLANG_INSTALL_DIR:-$HOME/.local/bin}"
# ============================================================
info()  { printf "\033[1;34m==>\033[0m %s\n" "$1"; }
error() { printf "\033[1;31merror:\033[0m %s\n" "$1" >&2; exit 1; }

BIN_PATH="$INSTALL_DIR/$BIN_NAME"

# --- remove installed binary ---
if [ -f "$BIN_PATH" ]; then
  rm -f "$BIN_PATH"
  info "Removed $BIN_PATH"
else
  info "$BIN_PATH does not exist; nothing to remove."
fi

# --- remove PATH entry added by installer, from known shell rc files ---
for SHELL_RC in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.config/fish/config.fish"; do
  if [ -f "$SHELL_RC" ] && grep -q "Added by ${BIN_NAME} installer" "$SHELL_RC" 2>/dev/null; then
    # remove the marker comment line and the line immediately after it (the export/path line)
    TMP_RC="$(mktemp)"
    awk -v marker="# Added by ${BIN_NAME} installer" '
      $0 == marker { skip=2; next }
      skip > 0 { skip--; next }
      { print }
    ' "$SHELL_RC" > "$TMP_RC"
    mv "$TMP_RC" "$SHELL_RC"
    info "Removed PATH entry from $SHELL_RC"
    info "Run 'source $SHELL_RC' or restart your terminal for the change to take effect."
  fi
done

info "Done! $BIN_NAME has been uninstalled."