#!/usr/bin/env python3
"""Test 1: identify J400 pin 1, then measure VSW before any Saleae hookup."""

from __future__ import annotations

import argparse

from buspirate_util import connect_or_exit
from wiring import DIAGRAMS, SAFETY, SALEAE_VMAX, VSW_OK, add_common_args, pause


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    add_common_args(parser)
    args = parser.parse_args()

    print(SAFETY)
    print()
    print(DIAGRAMS["01_continuity"])
    if args.wiring:
        print()
        print(DIAGRAMS["01_vsw"])
        return
    if pause() == "s":
        print("Skipped continuity. Identify pin 1 before you rely on later pin numbers.")

    print()
    print(DIAGRAMS["01_vsw"])
    if pause() == "s":
        print("Skipped VSW measurement.")
        return

    bp = connect_or_exit(args.port)
    try:
        volts = bp.voltage()
    finally:
        bp.close()

    print(f"VSW (pin 41) = {volts:.2f} V")
    low, high = VSW_OK
    if volts > SALEAE_VMAX:
        print(
            f"FAIL: {volts:.2f} V is above the Saleae continuous limit of "
            f"{SALEAE_VMAX} V. Do not connect the Saleae. Use a scope if you "
            "need to see whether this rail has spikes."
        )
        raise SystemExit(2)
    if volts < low or volts > high:
        print(
            f"CAUTION: {volts:.2f} V is outside the expected {low}–{high} V "
            "range. You can proceed only if you trust this reading. A regular "
            "meter on VBAT (pin 39) is still safe; the Saleae is not."
        )
        raise SystemExit(3)
    print(
        f"OK: VSW is inside {low}–{high} V and under the Saleae {SALEAE_VMAX} V "
        "limit. This does not prove there are no spikes; that needs a scope."
    )
    print("Measure VBAT on pin 39 with a regular meter only. It should be about 10 V.")


if __name__ == "__main__":
    main()
