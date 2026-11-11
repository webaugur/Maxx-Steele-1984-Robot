#!/usr/bin/env python3
"""Test 5: hook up the Saleae to CPU pin 25 (RadioIn) and capture a key press."""

from __future__ import annotations

import argparse
from pathlib import Path

from saleae_capture import maybe_capture
from wiring import DIAGRAMS, SAFETY, add_common_args, pause

CHANNELS = {
    0: "RadioIn",
    1: "OEB",
}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    add_common_args(parser, need_port=False)
    parser.add_argument("--capture", action="store_true")
    parser.add_argument("--rate", default="1m", help="1 MHz is enough for ~1.5 ms bits")
    parser.add_argument("--time", default="5s")
    parser.add_argument("--output", default="captures/radio.sr")
    args = parser.parse_args()

    print(SAFETY)
    print()
    print(DIAGRAMS["05_radio"])
    if args.wiring:
        return
    print()
    print("Start the capture, then press keys on the 27 MHz transmitter.")
    if pause() == "s":
        return
    csv_path = maybe_capture(
        CHANNELS, Path(args.output), args.rate, args.time, args.capture
    )
    if csv_path:
        print(f"Analyze with:  python3 analyze_timing.py {csv_path} --channels RadioIn")
    else:
        print(
            "After you export a CSV:\n"
            "  python3 analyze_timing.py capture.csv --channels RadioIn"
        )


if __name__ == "__main__":
    main()
