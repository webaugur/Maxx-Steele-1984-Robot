#!/usr/bin/env python3
"""Convert factory Chassis/Sounds maxx-song-*.wma to sim OGG assets."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOUNDS = ROOT / "Chassis" / "Sounds"
OUT = Path(__file__).resolve().parents[1] / "assets" / "music"


def main() -> int:
    ffmpeg = subprocess.run(["which", "ffmpeg"], capture_output=True, text=True)
    if ffmpeg.returncode != 0:
        print("ffmpeg required on PATH", file=sys.stderr)
        return 1
    ffmpeg_bin = ffmpeg.stdout.strip()

    OUT.mkdir(parents=True, exist_ok=True)
    for tune in range(1, 9):
        src = SOUNDS / f"maxx-song-{tune}.wma"
        dst = OUT / f"{tune}.ogg"
        if not src.is_file():
            print(f"missing {src}", file=sys.stderr)
            return 1
        print(f"tune {tune}: {src.name} -> {dst.name}")
        subprocess.run(
            [
                ffmpeg_bin,
                "-y",
                "-loglevel",
                "error",
                "-i",
                str(src),
                "-c:a",
                "libvorbis",
                "-q:a",
                "6",
                str(dst),
            ],
            check=True,
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())