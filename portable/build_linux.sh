#!/bin/sh
# Copy the Linux release binary into portable/MaxxSim/linux/.
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
python3 "$ROOT/tools/ensure_maxx_built"
mkdir -p "$ROOT/portable/MaxxSim/maxx/linux"
install -m 0755 "$ROOT/tools/maxxbas/target/release/maxx" "$ROOT/portable/MaxxSim/maxx/linux/maxx"
echo "ok: $ROOT/portable/MaxxSim/maxx/linux/maxx"

HACKRF="$ROOT/portable/MaxxSim/hackrf/linux"
mkdir -p "$HACKRF"
if [ -x /usr/bin/hackrf_info ] && [ -x /usr/bin/hackrf_transfer ]; then
  install -m 0755 /usr/bin/hackrf_info /usr/bin/hackrf_transfer "$HACKRF/"
  lib=$(ldd /usr/bin/hackrf_transfer | awk '/libhackrf\.so/{print $3; exit}')
  if [ -n "$lib" ]; then
    install -m 0644 "$lib" "$HACKRF/libhackrf.so.0"
  fi
  if command -v patchelf >/dev/null 2>&1; then
    patchelf --set-rpath '$ORIGIN' "$HACKRF/hackrf_info" "$HACKRF/hackrf_transfer"
  fi
  echo "ok: $HACKRF"
else
  echo "note: hackrf_info not installed; PATH fallback remains" >&2
fi
