#!/usr/bin/env python3
"""Build a single PDF from ProgrammersGuide markdown chapters."""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
from datetime import date
from pathlib import Path

from project_paths import project_root, resolve_from_root

GUIDE_DIR = Path("ProgrammersGuide")
OUTPUT_NAME = "Maxx-Steele-Programmers-Reference.pdf"

CHAPTERS = (
    "README.md",
    "01-Bytecode-Programming-Rules.md",
    "02-Opcode-Vocabulary.md",
    "03-Programming-Motion-and-Display.md",
    "04-Programming-Speech-and-Music.md",
    "05-Cartridge-Bootstrap-and-Internal-ROM.md",
    "06-Input-Output-Guide.md",
    "Appendices.md",
    "Quick-Reference.md",
    "Schematics.md",
)

SAME_GUIDE_LINK = re.compile(
    r"\[([^\]]+)\]\((?!https?://)([^)/]+\.md)(#[^)]+)?\)"
)
REPO_LINK = re.compile(r"\[([^\]]+)\]\(\.\./([^)]+)\)")


def preprocess_markdown(text: str) -> str:
    """Normalize links for a single-file PDF build."""

    def _same_guide_link(match: re.Match[str]) -> str:
        label, _file, anchor = match.group(1), match.group(2), match.group(3)
        return f"[{label}]({anchor})" if anchor else label

    def _repo_link(match: re.Match[str]) -> str:
        label, path = match.group(1), match.group(2)
        return f"{label} (`{path}`)"

    text = SAME_GUIDE_LINK.sub(_same_guide_link, text)
    text = REPO_LINK.sub(_repo_link, text)
    return text


def merge_chapters(guide_dir: Path) -> str:
    """Concatenate guide chapters with page breaks between files."""
    parts: list[str] = []
    for index, name in enumerate(CHAPTERS):
        path = guide_dir / name
        if not path.is_file():
            raise FileNotFoundError(path)
        body = preprocess_markdown(path.read_text(encoding="utf-8").strip())
        if index:
            parts.append(r"\newpage")
        parts.append(body)
    return "\n\n".join(parts) + "\n"


def build_pdf(
    *,
    output: Path,
    pandoc: str = "pandoc",
    pdf_engine: str = "xelatex",
    check_only: bool = False,
) -> None:
    root = project_root()
    guide_dir = resolve_from_root(GUIDE_DIR, must_exist=True)
    output_path = output if output.is_absolute() else root / output

    if shutil.which(pandoc) is None:
        raise RuntimeError(
            f"{pandoc} not found. Install pandoc and a LaTeX engine "
            "(e.g. texlive-xetex) to build the programmers guide PDF."
        )

    merged = merge_chapters(guide_dir)
    today = date.today().isoformat()
    metadata = f"""---
title: "Maxx Steele Programmer's Reference Guide"
author: "Maxx-Steele-1984-Robot contributors"
date: "{today}"
lang: en-US
toc: true
toc-depth: 2
numbersections: true
geometry: margin=1in
fontsize: 11pt
documentclass: report
---

"""

    with tempfile.TemporaryDirectory(prefix="maxx-guide-") as tmp:
        source = Path(tmp) / "merged.md"
        source.write_text(metadata + merged, encoding="utf-8")

        command = [
            pandoc,
            str(source),
            "-o",
            str(output_path),
            f"--pdf-engine={pdf_engine}",
            "--highlight-style=tango",
            "-V",
            "mainfont=DejaVu Serif",
            "-V",
            "monofont=DejaVu Sans Mono",
        ]

        if check_only:
            print(" ".join(command))
            return

        output_path.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run(command, check=True)
        print(output_path)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        default=GUIDE_DIR / OUTPUT_NAME,
        help=f"Output PDF path (default: {GUIDE_DIR / OUTPUT_NAME})",
    )
    parser.add_argument(
        "--pandoc",
        default="pandoc",
        help="Pandoc executable (default: pandoc)",
    )
    parser.add_argument(
        "--pdf-engine",
        default="xelatex",
        help="Pandoc PDF engine (default: xelatex)",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Print the pandoc command without building",
    )
    args = parser.parse_args(argv)

    try:
        build_pdf(
            output=args.output,
            pandoc=args.pandoc,
            pdf_engine=args.pdf_engine,
            check_only=args.check,
        )
    except (FileNotFoundError, RuntimeError, subprocess.CalledProcessError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())