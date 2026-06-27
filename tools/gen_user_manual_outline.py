#!/usr/bin/env python3
"""Build or validate Docs/User/Sources/outline.json from OCR section headers."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

from project_paths import project_root

from manual_paths import USER_MANUAL

TEXT_DIR = USER_MANUAL / "Sources/text"
OUTLINE = USER_MANUAL / "Sources/outline.json"

SECTION_RE = re.compile(
    r"^("
    r"\d+\.\s+[A-Z][A-Z\s\-/™®]+|"
    r"\d+\.\d+[A-Z]?\s+[A-Z][A-Z\s\-/™®]+|"
    r"APPENDIX\s+[A-Z]|"
    r"[A-J]\.\s+[A-Z][A-Z\s\-/™®]+"
    r")",
    re.MULTILINE,
)


def load_text(page: int, text_dir: Path) -> str:
    path = text_dir / f"page-{page:02d}.txt"
    if not path.is_file():
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def scan_sections(text_dir: Path, *, page_count: int) -> list[dict]:
    entries: list[dict] = []
    for page in range(1, page_count + 1):
        text = load_text(page, text_dir)
        if not text.strip():
            entries.append({"page": page, "headers": [], "chars": 0})
            continue
        headers = [m.group(1).strip() for m in SECTION_RE.finditer(text)]
        entries.append({"page": page, "headers": headers, "chars": len(text)})
    return entries


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--text", type=Path, default=TEXT_DIR)
    ap.add_argument("--outline", type=Path, default=OUTLINE)
    ap.add_argument("--pages", type=int, default=48)
    ap.add_argument("--write", action="store_true", help="Reserved; outline is hand-curated")
    args = ap.parse_args(argv)

    root = project_root()
    text_dir = args.text if args.text.is_absolute() else root / args.text
    outline_path = args.outline if args.outline.is_absolute() else root / args.outline

    if not text_dir.is_dir():
        print(f"error: missing OCR text dir {text_dir}", file=sys.stderr)
        return 1

    scan = scan_sections(text_dir, page_count=args.pages)
    for item in scan:
        if item["headers"]:
            joined = "; ".join(item["headers"][:4])
            if len(item["headers"]) > 4:
                joined += f" (+{len(item['headers']) - 4} more)"
            print(f"page-{item['page']:02d}: {joined}")
        elif item["chars"] < 80:
            print(f"page-{item['page']:02d}: [sparse/image — {item['chars']} chars]")

    if outline_path.is_file():
        outline = json.loads(outline_path.read_text(encoding="utf-8"))
        covered = set()
        for ch in outline.get("chapters", []):
            covered.update(ch.get("pages", []))
        missing = [p for p in range(3, args.pages + 1) if p not in covered and p not in (41, 42)]
        if missing:
            print(f"\nwarning: pages not assigned in outline: {missing}", file=sys.stderr)
        else:
            print(f"\noutline OK — {outline_path.relative_to(root)}")
    else:
        print(f"\nwarning: {outline_path} not found", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())