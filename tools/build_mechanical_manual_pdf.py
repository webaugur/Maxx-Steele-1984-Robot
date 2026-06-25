#!/usr/bin/env python3
"""Build the Maxx Steele Mechanical Manual PDF from markdown chapters and covers."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from manual_pdf import ManualSpec, build_pdf
from project_paths import project_root, resolve_from_root

MANUAL_DIR = Path("MechanicalManual")
OUTPUT_NAME = "Maxx-Steele-Mechanical-Manual.pdf"

CHAPTERS = (
    "README.md",
    "01-Disassembly.md",
    "02-Reassembly.md",
    "03-Chassis-Photos.md",
)

MECHANICAL_SPEC = ManualSpec(
    manual_dir=MANUAL_DIR,
    chapters=CHAPTERS,
    output_name=OUTPUT_NAME,
    title="Maxx Steele Mechanical Manual",
)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        default=MANUAL_DIR / OUTPUT_NAME,
        help=f"Output PDF path (default: {MANUAL_DIR / OUTPUT_NAME})",
    )
    parser.add_argument("--pandoc", default="pandoc")
    parser.add_argument("--pdf-engine", default="xelatex")
    parser.add_argument("--img2pdf", default="img2pdf")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv)

    root = project_root()
    manual_dir = resolve_from_root(MECHANICAL_SPEC.manual_dir, must_exist=True)
    output_path = args.output if args.output.is_absolute() else root / args.output
    spec = ManualSpec(
        manual_dir=manual_dir,
        chapters=MECHANICAL_SPEC.chapters,
        output_name=MECHANICAL_SPEC.output_name,
        title=MECHANICAL_SPEC.title,
        author=MECHANICAL_SPEC.author,
    )

    try:
        build_pdf(
            spec,
            output=output_path,
            pandoc=args.pandoc,
            pdf_engine=args.pdf_engine,
            img2pdf=args.img2pdf,
            check_only=args.check,
        )
    except (FileNotFoundError, RuntimeError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())