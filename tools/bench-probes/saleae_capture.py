"""Shared helpers for Saleae / sigrok captures."""

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path

from wiring import SALEAE_COLOR


def channel_table(mapping: dict[int, str]) -> str:
    lines = [f"  {'CH':<5} {'Color':<24} {'Signal'}"]
    lines.append(f"  {'GND':<5} {SALEAE_COLOR['GND']:<24} GND")
    for ch in range(8):
        signal = mapping.get(ch, "unused")
        lines.append(f"  {('CH'+str(ch)):<5} {SALEAE_COLOR[ch]:<24} {signal}")
    return "\n".join(lines)


def sigrok_command(
    mapping: dict[int, str],
    outfile: Path,
    samplerate: str,
    duration: str,
) -> list[str]:
    channels = ",".join(
        f"D{ch}={name}" for ch, name in sorted(mapping.items()) if name
    )
    return [
        "sigrok-cli",
        "-d",
        "fx2lafw",
        "--config",
        f"samplerate={samplerate}",
        "--channels",
        channels,
        "--time",
        duration,
        "-o",
        str(outfile),
    ]


def maybe_capture(
    mapping: dict[int, str],
    outfile: Path,
    samplerate: str,
    duration: str,
    do_capture: bool,
) -> Path | None:
    cmd = sigrok_command(mapping, outfile, samplerate, duration)
    print("sigrok-cli (optional, if PulseView / sigrok is installed):")
    print("  " + " ".join(cmd))
    print()
    print(
        "sigrok loads temporary fx2lafw firmware onto the Saleae. Unplug and "
        "replug the analyzer before opening official Logic 1.2.18 again."
    )
    print(
        "If you are using Logic 1.2.18 instead, set the same channel names, "
        f"sample at {samplerate}, capture {duration}, and export CSV."
    )
    if not do_capture:
        return None
    if not shutil.which("sigrok-cli"):
        raise SystemExit(
            "sigrok-cli is not on PATH. Run without --capture and use "
            "Logic 1.2.18 or PulseView, or install sigrok."
        )
    outfile.parent.mkdir(parents=True, exist_ok=True)
    print("Starting capture...")
    subprocess.run(cmd, check=True)
    print(f"Wrote {outfile}")
    csv_path = outfile.with_suffix(".csv")
    export = ["sigrok-cli", "-i", str(outfile), "-O", "csv", "-o", str(csv_path)]
    subprocess.run(export, check=True)
    print(f"Exported {csv_path}")
    return csv_path
