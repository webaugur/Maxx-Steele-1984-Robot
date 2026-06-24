#!/usr/bin/env python3
"""Build the Maxx Steele Technical Manual PDF from markdown chapters and covers."""

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

MANUAL_DIR = Path("TechnicalManual")
OUTPUT_NAME = "Maxx-Steele-Technical-Manual.pdf"
COVER_FRONT = "cover-front.jpg"
COVER_REAR = "cover-rear.jpg"

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


def require_tool(name: str) -> str:
    path = shutil.which(name)
    if path is None:
        raise RuntimeError(f"{name} not found on PATH")
    return path


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


def merge_chapters(manual_dir: Path) -> str:
    """Concatenate manual chapters with page breaks between files."""
    parts: list[str] = []
    for index, name in enumerate(CHAPTERS):
        path = manual_dir / name
        if not path.is_file():
            raise FileNotFoundError(path)
        body = preprocess_markdown(path.read_text(encoding="utf-8").strip())
        if index:
            parts.append(r"\newpage")
        parts.append(body)
    return "\n\n".join(parts) + "\n"


def build_body_pdf(
    *,
    manual_dir: Path,
    output: Path,
    pandoc: str,
    pdf_engine: str,
    check_only: bool,
) -> list[str]:
    merged = merge_chapters(manual_dir)
    today = date.today().isoformat()
    metadata = f"""---
title: "Maxx Steele Technical Manual"
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

    command = [
        pandoc,
        "-",
        "-o",
        str(output),
        f"--pdf-engine={pdf_engine}",
        "--highlight-style=tango",
        "-V",
        "mainfont=DejaVu Serif",
        "-V",
        "monofont=DejaVu Sans Mono",
    ]

    if check_only:
        print(" ".join(command))
        return command

    subprocess.run(
        command,
        check=True,
        input=metadata + merged,
        text=True,
        encoding="utf-8",
    )
    return command


def build_cover_pdf(*, image: Path, output: Path, img2pdf: str) -> None:
    subprocess.run(
        [
            img2pdf,
            "--pagesize",
            "Letter",
            "--fit",
            "shrink",
            str(image),
            "-o",
            str(output),
        ],
        check=True,
    )


def merge_pdfs(*, parts: list[Path], output: Path, qpdf: str) -> None:
    subprocess.run(
        [qpdf, "--empty", "--pages", *[str(part) for part in parts], "--", str(output)],
        check=True,
    )


def build_pdf(
    *,
    output: Path,
    pandoc: str = "pandoc",
    pdf_engine: str = "xelatex",
    img2pdf: str = "img2pdf",
    qpdf: str = "qpdf",
    check_only: bool = False,
) -> None:
    root = project_root()
    manual_dir = resolve_from_root(MANUAL_DIR, must_exist=True)
    output_path = output if output.is_absolute() else root / output
    front_cover = manual_dir / COVER_FRONT
    rear_cover = manual_dir / COVER_REAR

    for path in (front_cover, rear_cover):
        if not path.is_file():
            raise FileNotFoundError(path)

    if shutil.which(pandoc) is None:
        raise RuntimeError(
            f"{pandoc} not found. Install pandoc and a LaTeX engine "
            "(e.g. texlive-xetex) to build the technical manual PDF."
        )
    if not check_only:
        require_tool(img2pdf)
        require_tool(qpdf)

    with tempfile.TemporaryDirectory(prefix="maxx-manual-") as tmp:
        tmp_dir = Path(tmp)
        body_pdf = tmp_dir / "body.pdf"
        front_pdf = tmp_dir / "front.pdf"
        rear_pdf = tmp_dir / "rear.pdf"

        build_body_pdf(
            manual_dir=manual_dir,
            output=body_pdf,
            pandoc=pandoc,
            pdf_engine=pdf_engine,
            check_only=check_only,
        )
        if check_only:
            return

        build_cover_pdf(image=front_cover, output=front_pdf, img2pdf=img2pdf)
        build_cover_pdf(image=rear_cover, output=rear_pdf, img2pdf=img2pdf)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        merge_pdfs(
            parts=[front_pdf, body_pdf, rear_pdf],
            output=output_path,
            qpdf=qpdf,
        )
        print(output_path)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        default=MANUAL_DIR / OUTPUT_NAME,
        help=f"Output PDF path (default: {MANUAL_DIR / OUTPUT_NAME})",
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
        "--img2pdf",
        default="img2pdf",
        help="img2pdf executable (default: img2pdf)",
    )
    parser.add_argument(
        "--qpdf",
        default="qpdf",
        help="qpdf executable (default: qpdf)",
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
            img2pdf=args.img2pdf,
            qpdf=args.qpdf,
            check_only=args.check,
        )
    except (FileNotFoundError, RuntimeError, subprocess.CalledProcessError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())