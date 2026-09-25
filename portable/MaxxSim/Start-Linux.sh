#!/bin/sh
# Open the live simulator. Internal ROM, or carts/default.532 when that file exists.
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
BIN="$ROOT/linux/maxx"
if [ ! -f "$BIN" ]; then
  echo "Missing $BIN" >&2
  echo "From the repo: sh portable/build_linux.sh" >&2
  exit 1
fi
chmod +x "$BIN" 2>/dev/null || true
if [ -f "$ROOT/carts/default.532" ]; then
  exec "$BIN" simulate --gui "$ROOT/carts/default.532"
else
  exec "$BIN" simulate --gui
fi
