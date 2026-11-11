#!/usr/bin/env python3
"""Summarize idle levels, transitions, and pulse widths from a digital CSV."""

from __future__ import annotations

import argparse
import statistics
from pathlib import Path

from csv_capture import load_csv, pick_channel


def pulses(times: list[float], levels: list[int], level: int) -> list[float]:
    widths = []
    start = None
    for i, value in enumerate(levels):
        if value == level and start is None:
            start = times[i]
        elif value != level and start is not None:
            widths.append(times[i] - start)
            start = None
    return widths


def summarize(name: str, times: list[float], levels: list[int]) -> None:
    n = min(len(times), len(levels))
    times, levels = times[:n], levels[:n]
    if n == 0:
        print(f"{name}: no samples")
        return
    high = sum(levels)
    duty = high / n
    edges = sum(1 for i in range(1, n) if levels[i] != levels[i - 1])
    print(f"{name}")
    print(f"  samples {n}, first {levels[0]}, last {levels[-1]}")
    print(f"  high duty {duty * 100:.2f}%, transitions {edges}")
    if n >= 2 and times[-1] > times[0]:
        span = times[-1] - times[0]
        print(f"  time span {span:.6f}")
        for level, label in ((1, "high"), (0, "low")):
            widths = pulses(times, levels, level)
            if not widths:
                continue
            mid = statistics.median(widths)
            print(
                f"  {label} pulses: {len(widths)}, "
                f"median {mid:.6g}, min {min(widths):.6g}, max {max(widths):.6g}"
            )
            # Radio bits are expected around 1.5–1.7 ms.
            if 0.0005 <= mid <= 0.005:
                print(f"    (median {label} width is {mid * 1000:.3f} ms)")
    elif edges == 0:
        print("  line did not change in this capture")
    print()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("csv", type=Path)
    parser.add_argument(
        "--channels",
        help="Comma-separated column names (default: every channel)",
    )
    args = parser.parse_args()
    times, series, aliases = load_csv(args.csv)
    if args.channels:
        names = [pick_channel(series, aliases, part.strip()) for part in args.channels.split(",")]
    else:
        names = list(series)
    for name in names:
        summarize(name, times, series[name])


if __name__ == "__main__":
    main()
