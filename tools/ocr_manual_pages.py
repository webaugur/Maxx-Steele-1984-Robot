#!/usr/bin/env python3
"""OCR extracted manual page PNGs with tesseract."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

from project_paths import project_root

DEFAULT_PAGES = Path("UserManual/Sources/pages")
DEFAULT_TEXT = Path("UserManual/Sources/text")


def ocr_page(image: Path, out_txt: Path, *, tesseract: str) -> None:
    out_txt.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        [tesseract, str(image), str(out_txt.with_suffix("")), "-l", "eng"],
        check=True,
        capture_output=True,
    )


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--pages", type=Path, default=DEFAULT_PAGES)
    ap.add_argument("--text", type=Path, default=DEFAULT_TEXT)
    ap.add_argument("--tesseract", default="tesseract")
    ap.add_argument("--page", type=int, help="OCR single page number only")
    args = ap.parse_args(argv)

    root = project_root()
    pages_dir = args.pages if args.pages.is_absolute() else root / args.pages
    text_dir = args.text if args.text.is_absolute() else root / args.text

    images = sorted(pages_dir.glob("page-*.png"))
    if args.page is not None:
        images = [pages_dir / f"page-{args.page:02d}.png"]

    if not images:
        print(f"error: no pages in {pages_dir}", file=sys.stderr)
        return 1

    for img in images:
        num = img.stem.split("-", 1)[1]
        out = text_dir / f"page-{num}.txt"
        try:
            ocr_page(img, out, tesseract=args.tesseract)
            print(out)
        except subprocess.CalledProcessError as exc:
            print(f"error on {img}: {exc.stderr.decode()}", file=sys.stderr)
            return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())