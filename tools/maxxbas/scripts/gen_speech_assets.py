#!/usr/bin/env python3
"""Legacy OGG generator (optional). Live sim synthesizes phrases at runtime via rustsam/SAM."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

# Phrase index (hex) → English (User Manual / CBS Demo / MaxxOS)
PHRASES: dict[int, str] = {
    0x00: "I am great, and you.",
    0x01: "I am ready when you are.",
    0x02: "I am a great match for humans.",
    0x03: "Goodbye for now, have a good day.",
    0x10: "Hello. I am Maxx Steele.",
    0x11: "Please choose how tough.",
    0x12: "Please choose game.",
    0x13: "Good play.",
    0x14: "Thank you.",
    0x15: "Is there anything I can do for you?",
    0x16: "Good morning.",
    0x17: "It is time to get up.",
    0x18: "Maxx Steele wins.",
    0x19: "Congratulations, you win.",
    0x1A: "I need energy, please recharge me.",
    0x1B: "Game over.",
    0x1C: "Choose enter to play again.",
    0x1D: "Sorry, my circuits are full.",
    0x1E: "Please teach me.",
    0x1F: "Please program me.",
    0x20: "I'm ready.",
    0x3F: "Ha ha ha ha ha.",
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
        # Slow deep UK TTS; runtime AM vocoder adds robot timbre.
        gTTS(text=text, lang="en", tld="co.uk", slow=True).save(str(mp3))
        robot_af = (
            "asetrate=48000*0.68,aresample=48000,atempo=0.84,"
            "highpass=f=110,lowpass=f=2600,"
            "equalizer=f=480:t=h:width=350:g=4,"
            "equalizer=f=1100:t=h:width=450:g=3,"
            "acompressor=threshold=-20dB:ratio=3:attack=12:release=100,"
            "loudnorm=I=-15:TP=-1.0:LRA=9"
        )
        subprocess.run(
            [
                ffmpeg_bin,
                "-y",
                "-i",
                str(mp3),
                "-af",
                robot_af,
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