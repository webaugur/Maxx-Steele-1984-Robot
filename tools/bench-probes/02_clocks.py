#!/usr/bin/env python3
"""Test 2: measure F2, F244, and F499 and compare 455 kHz vs 500 kHz timing."""

from __future__ import annotations

import argparse

from buspirate_util import connect_or_exit
from wiring import DIAGRAMS, SAFETY, add_common_args, pause

F244_455 = 455_000 / 2048  # ~222.17 Hz
F244_500 = 500_000 / 2048  # ~244.14 Hz
F499_455 = 455_000
F499_500 = 500_000
WINDOW = 0.05  # +/- 5 %, ceramic resonators are loose, 455 vs 500 is 9 %


def near(value: float, expected: float, window: float = WINDOW) -> bool:
    return expected > 0 and abs(value - expected) / expected <= window


def classify_f244(hz: float) -> str:
    if near(hz, F244_455):
        return "matches 455 kHz / 2048 (~222 Hz)"
    if near(hz, F244_500):
        return "matches 500 kHz / 2048 (~244 Hz)"
    return "does not match 222 Hz or 244 Hz"


def classify_f499(hz: float) -> str:
    if near(hz, F499_455):
        return "matches a 455 kHz clock"
    if near(hz, F499_500):
        return "matches a 500 kHz clock"
    return "does not match 455 kHz or 500 kHz"


def measure(bp, name: str) -> float | None:
    answer = pause(f"Press Enter to measure {name} (or type s to skip)...")
    if answer == "s":
        print(f"Skipped {name}.")
        return None
    hz = bp.frequency_hz()
    if hz <= 0:
        print(f"{name}: no frequency detected (0 Hz). Is AUX on the signal?")
    else:
        print(f"{name}: {hz:,.2f} Hz")
    return hz


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    add_common_args(parser)
    args = parser.parse_args()

    print(SAFETY)
    print()
    print(DIAGRAMS["02_f2"])
    print()
    print(DIAGRAMS["02_f244"])
    print()
    print(DIAGRAMS["02_f499"])
    if args.wiring:
        return

    bp = connect_or_exit(args.port)
    results: dict[str, float] = {}
    try:
        print(DIAGRAMS["02_f2"])
        hz = measure(bp, "F2 (J400 pin 29)")
        if hz is not None:
            results["F2"] = hz

        print()
        print(DIAGRAMS["02_f244"])
        hz = measure(bp, "F244 (CPU pin 54)")
        if hz is not None:
            results["F244"] = hz
            if hz > 0:
                print("  " + classify_f244(hz))

        print()
        print(DIAGRAMS["02_f499"])
        hz = measure(bp, "F499 (CPU pin 57)")
        if hz is not None:
            results["F499"] = hz
            if hz > 0:
                print("  " + classify_f499(hz))
    finally:
        bp.close()

    print()
    print("Summary")
    for name, hz in results.items():
        print(f"  {name}: {hz:,.2f} Hz")
    votes_455 = 0
    votes_500 = 0
    if "F244" in results and results["F244"] > 0:
        votes_455 += int(near(results["F244"], F244_455))
        votes_500 += int(near(results["F244"], F244_500))
    if "F499" in results and results["F499"] > 0:
        votes_455 += int(near(results["F499"], F499_455))
        votes_500 += int(near(results["F499"], F499_500))
    if votes_455 and not votes_500:
        print(
            "These readings mesh with a 455 kHz clock. The 244 Hz / 499 kHz "
            "labels look like they were written for 500 kHz."
        )
    elif votes_500 and not votes_455:
        print(
            "These readings mesh with a 500 kHz clock, not 455 kHz. Worth "
            "double-checking the marking on the resonator."
        )
    elif votes_455 and votes_500:
        print("The two clock pins did not agree on 455 vs 500 kHz. Recheck the hookup.")
    elif results:
        print("Recorded the values above; they did not fall clearly on 455 or 500 kHz.")


if __name__ == "__main__":
    main()
