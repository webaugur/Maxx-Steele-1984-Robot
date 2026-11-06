#!/bin/sh
# Register .532 files for this user. Run again if the stick moves.
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
BIN="$ROOT/maxx/linux/maxx"
if [ ! -f "$BIN" ]; then
  echo "Missing $BIN" >&2
  exit 1
fi

mime_pkg="${XDG_DATA_HOME:-$HOME/.local/share}/mime/packages"
apps="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
icons="${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor/48x48/apps"
mkdir -p "$mime_pkg" "$apps" "$icons"

cp "$ROOT/mime/maxx-cartridge.xml" "$mime_pkg/maxx-cartridge.xml"
cp "$ROOT/icons/rom-chip.png" "$icons/maxx-cartridge.png"

desktop="$apps/maxx-cartridge.desktop"
cat > "$desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Maxx Steele simulator
Comment=Open a Maxx Steele .532 cartridge
Exec=$BIN simulate --gui %f
MimeType=application/x-maxx-cartridge;
Icon=maxx-cartridge
Terminal=false
NoDisplay=true
EOF

if command -v update-mime-database >/dev/null 2>&1; then
  update-mime-database "${XDG_DATA_HOME:-$HOME/.local/share}/mime"
fi
if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$apps" || true
fi
if command -v xdg-mime >/dev/null 2>&1; then
  xdg-mime default maxx-cartridge.desktop application/x-maxx-cartridge || true
fi

magic="$HOME/.magic"
mark="# Maxx Steele cartridge"
if ! grep -q "$mark" "$magic" 2>/dev/null; then
  cat "$ROOT/mime/maxx.magic" >> "$magic"
fi
if command -v file >/dev/null 2>&1; then
  file -C -m "$magic" >/dev/null 2>&1 || true
fi

echo "Registered .532 -> $BIN simulate --gui"
echo "Run this again if the USB mount path changes."
