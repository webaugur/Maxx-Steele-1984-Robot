#!/bin/sh
# Register .532 files with Launch Services. Run again if the stick moves.
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
APP="$ROOT/Maxx.app"
if [ ! -d "$APP" ]; then
  echo "Missing $APP" >&2
  exit 1
fi
LSREGISTER="/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister"
if [ -x "$LSREGISTER" ]; then
  "$LSREGISTER" -f "$APP"
fi
echo "Registered .532 with $APP"
echo "If macOS asks, right-click Maxx.app and choose Open once."
echo "Run this again if the USB mount path changes."
