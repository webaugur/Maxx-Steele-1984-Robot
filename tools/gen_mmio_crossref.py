#!/usr/bin/env python3
"""Mine memory-mapped I/O accesses from maxx_internal_ROM.dsm."""

from __future__ import annotations

import argparse
import re
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DSM = ROOT / "Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm"

TARGETS = {
    "0010": "$1000",
    "0012": "$1200",
    "0014": "$1400",
    "0016": "$1600",
    "001C": "$1C00",
    "001E": "$1E00",
}

OPS = {
    "8D": "STA",
    "8E": "STX",
    "8C": "STY",
    "AD": "LDA",
    "AC": "LDY",
    "AE": "LDX",
    "2C": "BIT",
    "0D": "ORA",
    "0E": "ASL",
    "CE": "DEC",
    "EE": "INC",
}

LINE_RE = re.compile(r"^([0-9A-F]{4}):\s+([0-9A-F]{2})([0-9A-F]{4})\s+(.*)$")


def rom_addr(file_off: str) -> str:
    return f"${int(file_off, 16) + 0xE000:04X}"


def parse_dsm(path: Path) -> dict[str, list[tuple[str, str, str]]]:
    hits: dict[str, list[tuple[str, str, str]]] = defaultdict(list)
    with path.open() as fh:
        for raw in fh:
            m = LINE_RE.match(raw.strip())
            if not m:
                continue
            off, opb, addr, rest = m.group(1), m.group(2).upper(), m.group(3).upper(), m.group(4)
            if addr not in TARGETS:
                continue
            op = OPS.get(opb, opb)
            comment = rest.split("\t")[-1].strip() if "\t" in rest else rest.strip()
            hits[TARGETS[addr]].append((rom_addr(off), op, comment))
    return hits


def markdown_table(hits: dict[str, list[tuple[str, str, str]]]) -> str:
    lines = [
        "# MMIO access cross-reference (generated)",
        "",
        f"Source: [`maxx_internal_ROM.dsm`](../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm)  ",
        "Regenerate: `python3 tools/gen_mmio_crossref.py --md`",
        "",
    ]
    order = sorted(hits, key=lambda a: int(a[1:], 16))
    for addr in order:
        rows = hits[addr]
        lines.append(f"## {addr} ({len(rows)} accesses)")
        lines.append("")
        lines.append("| ROM | Op | Notes |")
        lines.append("|-----|-----|-------|")
        for rom, op, note in rows:
            note = note.replace("|", "\\|")
            lines.append(f"| `{rom}` | {op} | {note or '—'} |")
        lines.append("")
    return "\n".join(lines)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--md", action="store_true", help="Write Mainboard/Schematic/MMIO-ROM-Crossref.md")
    ap.add_argument("--json", action="store_true", help="Print JSON summary counts")
    args = ap.parse_args()

    hits = parse_dsm(DSM)
    if args.json:
        import json

        print(json.dumps({k: len(v) for k, v in sorted(hits.items())}, indent=2))
        return

    text = markdown_table(hits)
    if args.md:
        out = ROOT / "Mainboard/Schematic/MMIO-ROM-Crossref.md"
        out.write_text(text)
        print(out)
    else:
        print(text)


if __name__ == "__main__":
    main()