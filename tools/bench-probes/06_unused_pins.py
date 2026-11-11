#!/usr/bin/env python3
"""Test 6: DC level of unused J400 fingers, one at a time, with the Bus Pirate ADC."""

from __future__ import annotations

import argparse

from buspirate_util import connect_or_exit
from wiring import (
    DIAGRAMS,
    SAFETY,
    SALEAE_VIH,
    SALEAE_VIL,
    UNUSED_DC_PINS,
    add_common_args,
    pause,
)


def classify(volts: float) -> str:
    if volts <= SALEAE_VIL:
        return "low"
    if volts >= SALEAE_VIH:
        return "high"
    return "mid / maybe floating"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    add_common_args(parser)
    args = parser.parse_args()

    print(SAFETY)
    print()
    print(DIAGRAMS["06_unused"])
    if args.wiring:
        return

    print("The ADC lead stays on whichever pin the next prompt names.")
    print("Skip pin 39 (VBAT) if you are ever tempted to add it. It is not in this list.")
    print()
    bp = connect_or_exit(args.port)
    rows = []
    try:
        for pin, name in UNUSED_DC_PINS:
            print(f"Move ADC (yellow) to J400 pin {pin}  ({name})")
            if pause() == "s":
                print("  skipped")
                rows.append((pin, name, None, "skipped"))
                continue
            volts = bp.voltage()
            kind = classify(volts)
            print(f"  {volts:.2f} V  ({kind})")
            rows.append((pin, name, volts, kind))
    finally:
        bp.close()

    print()
    print(f"{'Pin':>4}  {'Signal':<12}  {'Volts':>6}  Level")
    for pin, name, volts, kind in rows:
        value = "  n/a" if volts is None else f"{volts:6.2f}"
        print(f"{pin:4d}  {name:<12}  {value}  {kind}")
    print()
    print(
        "A mid level often means the net is floating under the Bus Pirate's "
        "20 kΩ ADC load. High or low while the robot is running usually means "
        "something is driving the pin. Recheck any mid pin with a meter if you "
        "plan to reuse it."
    )


if __name__ == "__main__":
    main()
