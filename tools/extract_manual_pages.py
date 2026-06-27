#!/usr/bin/env python3
"""Extract PDF manual pages to PNG for OCR and figure embedding."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

from project_paths import project_root, resolve_from_root

DEFAULT_PDF = Path("Chassis/Manual/MaxxSteeleManual.pdf")
from manual_paths import USER_MANUAL

DEFAULT_OUT = USER_MANUAL / "Sources/pages"


def require_tool(name: str) -> str:
    from shutil import which

    path = which(name)
    if path is None:
        raise RuntimeError(f"{name} not found on PATH")
    return path


def page_count(pdf: Path, pdfinfo: str) -> int:
    result = subprocess.run(
        [pdfinfo, str(pdf)],
        capture_output=True,
        text=True,
        check=True,
    )
    for line in result.stdout.splitlines():
        if line.startswith("Pages:"):
            return int(line.split(":", 1)[1].strip())
    raise RuntimeError(f"could not read page count from {pdf}")


def extract(pdf: Path, out_dir: Path, *, dpi: int, pdftoppm: str) -> int:
    out_dir.mkdir(parents=True, exist_ok=True)
    prefix = out_dir / "page"
    subprocess.run(
        [
            pdftoppm,
            "-png",
            "-r",
            str(dpi),
            str(pdf),
            str(prefix),
        ],
        check=True,
    )
    pages = sorted(out_dir.glob("page-*.png"))
    # Normalize to page-01.png … page-NN.png
    for i, src in enumerate(pages, start=1):
        dest = out_dir / f"page-{i:02d}.png"
        if src != dest:
            if dest.exists():
                dest.unlink()
            src.rename(dest)
    return len(pages)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--pdf", type=Path, default=DEFAULT_PDF)
    ap.add_argument("--out", type=Path, default=DEFAULT_OUT)
    ap.add_argument("--dpi", type=int, default=300)
    ap.add_argument("--pdftoppm", default="pdftoppm")
    args = ap.parse_args(argv)

    root = project_root()
    pdf = resolve_from_root(args.pdf, must_exist=True)
    out_dir = args.out if args.out.is_absolute() else root / args.out

    try:
        require_tool(args.pdftoppm)
        n = extract(pdf, out_dir, dpi=args.dpi, pdftoppm=args.pdftoppm)
        print(f"Extracted {n} pages to {out_dir}")
    except (RuntimeError, subprocess.CalledProcessError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())