#!/usr/bin/env python3
"""Classify a Test 3 CSV: is OEB active during internal ROM fetches?"""

from __future__ import annotations

import argparse
from collections import Counter
from pathlib import Path

from csv_capture import load_csv, pick_channel


REGIONS = (
    ("internal ROM $E000-$FFFF", lambda a: (a & 0xE000) == 0xE000),
    ("I/O $1000-$1FFF", lambda a: (a & 0xF000) == 0x1000),
    ("low $0000-$0FFF", lambda a: (a & 0xF000) == 0x0000),
    ("cart $2000-$7FFF", lambda a: 0x2000 <= a <= 0x7FFF),
    ("cart $8000-$BFFF", lambda a: 0x8000 <= a <= 0xBFFF),
    ("other $C000-$DFFF", lambda a: 0xC000 <= a <= 0xDFFF),
)


def region_name(addr: int) -> str:
    for name, test in REGIONS:
        if test(addr):
            return name
    return f"other ${addr:04X}"


def address(a15: int, a14: int, a13: int, a12: int) -> int:
    return (a15 << 15) | (a14 << 14) | (a13 << 13) | (a12 << 12)


def duty(levels: list[int]) -> float:
    if not levels:
        return 0.0
    return sum(levels) / len(levels)


def count_active(times: list[float], oeb: list[int], active: int) -> tuple[int, Counter]:
    """Count rising/falling edges into the active level, and region of that strobe."""
    # Caller passes already-classified per-sample regions separately.
    strobes = 0
    for i in range(1, len(oeb)):
        if oeb[i] == active and oeb[i - 1] != active:
            strobes += 1
    return strobes


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("csv", type=Path, help="CSV exported from Logic 1.2.18 or sigrok")
    parser.add_argument("--oeb", default="OEB")
    parser.add_argument("--a15", default="A15")
    parser.add_argument("--a14", default="A14")
    parser.add_argument("--a13", default="A13")
    parser.add_argument("--a12", default="A12")
    parser.add_argument(
        "--active",
        choices=("auto", "low", "high"),
        default="auto",
        help="OEB active level; default is the rarer level (the strobe)",
    )
    args = parser.parse_args()

    times, series, aliases = load_csv(args.csv)
    oeb_name = pick_channel(series, aliases, args.oeb)
    a15 = series[pick_channel(series, aliases, args.a15)]
    a14 = series[pick_channel(series, aliases, args.a14)]
    a13 = series[pick_channel(series, aliases, args.a13)]
    a12 = series[pick_channel(series, aliases, args.a12)]
    oeb = series[oeb_name]
    n = min(len(oeb), len(a15), len(a14), len(a13), len(a12))
    oeb, a15, a14, a13, a12 = oeb[:n], a15[:n], a14[:n], a13[:n], a12[:n]
    times = times[:n]

    high_duty = duty(oeb)
    print(f"Samples: {n}")
    if n >= 2:
        span = times[-1] - times[0]
        print(f"Time span: {span:.6f} (same units as the CSV time column)")
    print(f"OEB high duty cycle: {high_duty * 100:.2f}%")

    if args.active == "low":
        active = 0
        source = "forced active-low"
    elif args.active == "high":
        active = 1
        source = "forced active-high"
    else:
        # A chip-select / output-enable strobe usually sits inactive most of
        # the time. If the line is stuck, duty will be near 0 or 1.
        active = 0 if high_duty >= 0.5 else 1
        source = "auto (rarer level treated as the strobe)"
    print(f"Treating OEB active as {active} ({source})")
    if high_duty < 0.02 or high_duty > 0.98:
        print(
            "CAUTION: OEB barely toggles. It may be stuck, or this may not be "
            "the right pin. The region counts below may be meaningless."
        )

    regions = Counter()
    active_samples = 0
    for i in range(n):
        if oeb[i] != active:
            continue
        active_samples += 1
        addr = address(a15[i], a14[i], a13[i], a12[i])
        regions[region_name(addr)] += 1

    strobes = 0
    for i in range(1, n):
        if oeb[i] == active and oeb[i - 1] != active:
            strobes += 1
    print(f"OEB strobes (edges into the active level): {strobes}")
    print(f"Samples with OEB active: {active_samples}")
    print()
    print("Address region while OEB is active:")
    if not active_samples:
        print("  none")
        raise SystemExit(1)
    for name, _test in REGIONS:
        count = regions[name]
        pct = 100.0 * count / active_samples
        print(f"  {name:<28} {count:10d}  {pct:6.2f}%")

    rom = regions["internal ROM $E000-$FFFF"]
    cart = regions["cart $2000-$7FFF"] + regions["cart $8000-$BFFF"]
    io = regions["I/O $1000-$1FFF"]
    print()
    if rom and cart == 0 and io == 0 and rom == active_samples:
        print(
            "Result: OEB is active during internal ROM fetches, and only there "
            "in this capture. A PicoROM with /CE tied to ground would fight the "
            "internal ROM on every fetch. Do not use the PR55 /CE wiring as drawn."
        )
        raise SystemExit(2)
    if rom:
        pct = 100.0 * rom / active_samples
        print(
            f"Result: OEB is active during internal ROM fetches ({pct:.1f}% of "
            "active samples). A PicoROM with /CE tied to ground would drive the "
            "data bus at the same time as the internal ROM. Do not use the PR55 "
            "/CE wiring as drawn until this is resolved (a decoded chip select, "
            "or /CE only qualified into the cart address range)."
        )
        raise SystemExit(2)
    if cart and not rom:
        print(
            "Result: OEB was not active in $E000–$FFFF in this capture, and it "
            "was active in the cart range. That is the safer pattern: OEB is "
            "behaving like a cart select. Recapture around reset and other "
            "modes before treating it as settled."
        )
        raise SystemExit(0)
    print(
        "Result: OEB was not active in $E000–$FFFF in this capture, but it also "
        "was not clearly limited to the cart range. Recheck the wiring and take "
        "another capture during boot and idle."
    )
    raise SystemExit(0)


if __name__ == "__main__":
    main()
