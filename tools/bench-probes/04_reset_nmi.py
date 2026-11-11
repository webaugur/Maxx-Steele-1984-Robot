#!/usr/bin/env python3
"""Test 4: hook up the Saleae to watch RESETSYSB and MNMIB."""

from __future__ import annotations

import argparse
from pathlib import Path

from saleae_capture import maybe_capture
from wiring import DIAGRAMS, SAFETY, add_common_args, pause

CHANNELS = {
    0: "RESETSYSB",
    1: "MNMIB",
    2: "OEB",
    3: "BIGSPACEB",
}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    add_common_args(parser, need_port=False)
    parser.add_argument("--capture", action="store_true")
    parser.add_argument("--rate", default="12m")
    parser.add_argument("--time", default="8s", help="Long enough for a button press")
    parser.add_argument("--output", default="captures/reset_nmi.sr")
    args = parser.parse_args()

    print(SAFETY)
    print()
    print(DIAGRAMS["04_reset"])
    if args.wiring:
        return
    print()
    print("Do not drive pin 37. Watch it only.")
    if pause() == "s":
        return
    csv_path = maybe_capture(
        CHANNELS, Path(args.output), args.rate, args.time, args.capture
    )
    if csv_path:
        print(
            f"Analyze with:  python3 analyze_timing.py {csv_path} "
            "--channels RESETSYSB,MNMIB,OEB"
        )
    else:
        print(
            "After you export a CSV:\n"
            "  python3 analyze_timing.py capture.csv --channels RESETSYSB,MNMIB,OEB"
        )


if __name__ == "__main__":
    main()
