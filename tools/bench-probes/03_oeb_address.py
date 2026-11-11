#!/usr/bin/env python3
"""Test 3: hook up the Saleae for the OEB-versus-address go/no-go capture."""

from __future__ import annotations

import argparse
from pathlib import Path

from saleae_capture import maybe_capture
from wiring import DIAGRAMS, SAFETY, add_common_args, pause

CHANNELS = {
    0: "OEB",
    1: "A15",
    2: "A14",
    3: "A13",
    4: "A12",
    5: "WRB",
    6: "WEB",
}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    add_common_args(parser, need_port=False)
    parser.add_argument(
        "--capture",
        action="store_true",
        help="Run sigrok-cli if it is installed (loads fx2lafw firmware)",
    )
    parser.add_argument("--rate", default="12m", help="Sample rate, default 12m")
    parser.add_argument("--time", default="3s", help="Capture length, default 3s")
    parser.add_argument(
        "--output",
        default="captures/oeb_address.sr",
        help="sigrok session file to write when --capture is set",
    )
    args = parser.parse_args()

    print(SAFETY)
    print()
    print(DIAGRAMS["03_oeb"])
    if args.wiring:
        return
    print()
    print("No PicoROM in the slot. Robot should be on and already running.")
    if pause("Press Enter to print the capture command (or after hookup, if using --capture)...") == "s":
        return
    csv_path = maybe_capture(
        CHANNELS, Path(args.output), args.rate, args.time, args.capture
    )
    if csv_path:
        print(f"Analyze with:  python3 analyze_oeb.py {csv_path}")
    else:
        print("After you export a CSV from Logic 1.2.18 or PulseView:")
        print("  python3 analyze_oeb.py capture.csv")


if __name__ == "__main__":
    main()
