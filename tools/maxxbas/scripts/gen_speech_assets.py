#!/usr/bin/env python3
"""Generate OGG speech clips for the live simulator (ROM phrase indices)."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

# ROM phrase index (hex) -> English (User Manual / Technical Appendix J)
PHRASES: dict[int, str] = {
    0x10: "Hello. I am Maxx Steele.",
    0x13: "Good play.",
    0x20: "I'm ready.",
}

CRATE = Path(__file__).resolve().parents[1]
OUT = CRATE / "assets" / "speech" / "rom"


def main() -> int:
    try:
        from gtts import gTTS
    except ImportError:
        print(
            "gTTS required: python3 -m venv tools/maxxbas/.venv-tts && "
            ".venv-tts/bin/pip install gTTS",
            file=sys.stderr,
        )
        return 1

    ffmpeg = subprocess.run(["which", "ffmpeg"], capture_output=True, text=True)
    if ffmpeg.returncode != 0:
        print("ffmpeg required on PATH", file=sys.stderr)
        return 1
    ffmpeg_bin = ffmpeg.stdout.strip()

    OUT.mkdir(parents=True, exist_ok=True)
    for phrase_id, text in sorted(PHRASES.items()):
        mp3 = OUT / f"{phrase_id:02X}.mp3"
        ogg = OUT / f"{phrase_id:02X}.ogg"
        print(f"phrase ${phrase_id:02X}: {text!r}")
        gTTS(text=text, lang="en", tld="com").save(str(mp3))
        subprocess.run(
            [
                ffmpeg_bin,
                "-y",
                "-i",
                str(mp3),
                "-c:a",
                "libvorbis",
                "-q:a",
                "4",
                str(ogg),
            ],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        mp3.unlink(missing_ok=True)
        print(f"  -> {ogg.relative_to(CRATE)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())