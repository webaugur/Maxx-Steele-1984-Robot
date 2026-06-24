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

from PIL import Image

from project_paths import project_root, resolve_from_root

MANUAL_DIR = Path("TechnicalManual")
# 6.5 x 8.5 inch booklet trim (portrait page: wide x tall).
BOOKLET_WIDTH_IN = 6.5
BOOKLET_HEIGHT_IN = 8.5
BOOKLET_DPI = 300
BOOKLET_SIZE_PX = (
    int(BOOKLET_WIDTH_IN * BOOKLET_DPI),
    int(BOOKLET_HEIGHT_IN * BOOKLET_DPI),
)
BOOKLET_PAGESIZE = f"{BOOKLET_WIDTH_IN}inx{BOOKLET_HEIGHT_IN}in"
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
HTTP_LINK = re.compile(r"\[([^\]]+)\]\(https?://[^)]+\)")
FILE_LINK = re.compile(r"\[([^\]]+)\]\((?!https?://)([^)]+\.(?:pdf|md|py|532|64|dsm))\)")


def require_tool(name: str) -> str:
    path = shutil.which(name)
    if path is None:
        raise RuntimeError(f"{name} not found on PATH")
    return path


def chapter_anchor_map(manual_dir: Path, pandoc: str) -> dict[str, str]:
    """Map each chapter filename to the LaTeX \\label Pandoc assigns its top heading."""
    mapping: dict[str, str] = {}
    for name in CHAPTERS:
        path = manual_dir / name
        result = subprocess.run(
            [pandoc, str(path), "-t", "latex"],
            capture_output=True,
            text=True,
            check=True,
        )
        match = re.search(r"\\label\{([^}]+)\}", result.stdout)
        if match is None:
            raise RuntimeError(f"could not find top-level label for {path}")
        mapping[name] = match.group(1)
    return mapping


def latex_path(path: Path) -> str:
    """Escape a filesystem path for use inside LaTeX \\includegraphics."""
    return str(path.resolve()).replace("\\", "/")


def cover_latex_block(*, image: Path) -> str:
    """Return a Pandoc raw-LaTeX block for the rear cover page."""
    graphic = latex_path(image)
    body = (
        "\\clearpage\n"
        "\\begin{titlepage}\n"
        "\\centering\n"
        f"\\includegraphics[width=\\paperwidth,height=\\paperheight]{{{graphic}}}\n"
        "\\end{titlepage}\n"
    )
    return f"```{{=latex}}\n{body}```\n\n"


def wrap_with_rear_cover(*, merged: str, rear_image: Path) -> str:
    return merged + cover_latex_block(image=rear_image)


def front_cover_header_includes(front_image: Path) -> list[str]:
    """Inject the front cover before the title page without a post-build PDF merge."""
    graphic = latex_path(front_image)
    cover = (
        f"\\begin{{titlepage}}\\centering"
        f"\\includegraphics[width=\\paperwidth,height=\\paperheight]{{{graphic}}}"
        "\\end{titlepage}\\clearpage"
    )
    return [
        "\\usepackage{graphicx}",
        f"\\pretocmd{{\\maketitle}}{{{cover}}}{{}}{{}}",
    ]


def preprocess_markdown(text: str, anchor_map: dict[str, str]) -> str:
    """Normalize links for a single-file PDF build with internal PDF navigation."""

    def _same_guide_link(match: re.Match[str]) -> str:
        label, file_name, anchor = match.group(1), match.group(2), match.group(3)
        if anchor:
            return f"[{label}]({anchor})"
        chapter_anchor = anchor_map.get(file_name)
        if chapter_anchor:
            return f"[{label}](#{chapter_anchor})"
        return label

    def _repo_link(match: re.Match[str]) -> str:
        label, path = match.group(1), match.group(2)
        return f"{label} (`{path}`)"

    def _http_link(match: re.Match[str]) -> str:
        return match.group(1)

    def _file_link(match: re.Match[str]) -> str:
        return match.group(1)

    text = SAME_GUIDE_LINK.sub(_same_guide_link, text)
    text = REPO_LINK.sub(_repo_link, text)
    text = HTTP_LINK.sub(_http_link, text)
    text = FILE_LINK.sub(_file_link, text)
    # Thematic breaks render as large vertical gaps in LaTeX PDF output.
    text = re.sub(r"\n---\n", "\n\n", text)
    return text


def merge_chapters(manual_dir: Path, *, pandoc: str) -> str:
    """Concatenate manual chapters; each chapter file starts at the top of a page."""
    anchor_map = chapter_anchor_map(manual_dir, pandoc)
    parts: list[str] = []
    for index, name in enumerate(CHAPTERS):
        path = manual_dir / name
        if not path.is_file():
            raise FileNotFoundError(path)
        body = preprocess_markdown(
            path.read_text(encoding="utf-8").strip(),
            anchor_map,
        )
        if index > 0:
            parts.append(r"\clearpage")
        parts.append(body)
    return "\n\n".join(parts) + "\n"


def build_manual_pdf(
    *,
    manual_dir: Path,
    output: Path,
    front_cover_image: Path,
    rear_cover_image: Path,
    pandoc: str,
    pdf_engine: str,
    check_only: bool,
) -> list[str]:
    merged = wrap_with_rear_cover(
        merged=merge_chapters(manual_dir, pandoc=pandoc),
        rear_image=rear_cover_image,
    )
    today = date.today().isoformat()
    cover_lines = "\n".join(
        f"  - {line}" for line in front_cover_header_includes(front_cover_image)
    )
    metadata = f"""---
title: "Maxx Steele Technical Manual"
author: "Maxx-Steele-1984-Robot contributors"
date: "{today}"
lang: en-US
toc: true
toc-depth: 2
numbersections: true
geometry:
  - paperwidth=6.5in
  - paperheight=8.5in
  - margin=0.4in
  - top=0.45in
  - bottom=0.45in
documentclass: extarticle
classoption:
  - 9pt
linestretch: 0.92
header-includes:
{cover_lines}
  - \\usepackage{{xcolor}}
  - \\definecolor{{maxxnavy}}{{RGB}}{{0,35,102}}
  - \\usepackage{{titlesec}}
  - \\usepackage{{needspace}}
  - \\usepackage{{etoolbox}}
  - \\usepackage{{enumitem}}
  - \\usepackage{{setspace}}
  - \\titleformat{{\\section}}{{\\color{{maxxnavy}}\\bfseries\\Large}}{{\\thesection}}{{1em}}{{}}
  - \\titleformat{{\\subsection}}{{\\color{{maxxnavy}}\\bfseries\\large}}{{\\thesubsection}}{{1em}}{{}}
  - \\titleformat{{\\subsubsection}}{{\\color{{maxxnavy}}\\bfseries\\normalsize}}{{\\thesubsubsection}}{{1em}}{{}}
  - \\makeatletter
  - \\pretocmd{{\\section}}{{\\needspace{{8\\baselineskip}}}}{{}}{{}}
  - \\pretocmd{{\\subsection}}{{\\needspace{{6\\baselineskip}}}}{{}}{{}}
  - \\pretocmd{{\\subsubsection}}{{\\needspace{{5\\baselineskip}}}}{{}}{{}}
  - \\makeatother
  - \\titlespacing*{{\\section}}{{0pt}}{{1.1ex plus .15ex}}{{0.55ex plus .1ex}}
  - \\titlespacing*{{\\subsection}}{{0pt}}{{0.9ex plus .15ex}}{{0.4ex plus .1ex}}
  - \\titlespacing*{{\\subsubsection}}{{0pt}}{{0.75ex plus .1ex}}{{0.3ex plus .1ex}}
  - \\setlist{{nosep,leftmargin=*,topsep=0.25ex,parsep=0pt,itemsep=0.15ex}}
  - \\setlength{{\\parskip}}{{0.3em}}
  - \\setlength{{\\parindent}}{{0pt}}
  - \\setlength{{\\emergencystretch}}{{2em}}
  - \\usepackage{{float}}
  - \\floatplacement{{figure}}{{H}}
  - \\usepackage{{caption}}
  - \\captionsetup{{skip=4pt}}
  - \\AtBeginDocument{{\\hypersetup{{colorlinks=true,linkcolor=maxxnavy,urlcolor=maxxnavy,hidelinks=false,bookmarks=true,bookmarksopen=true,bookmarksnumbered=true}}}}
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
        "-V",
        "monofontsize=\\footnotesize",
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


def prepare_cover_image(*, source: Path, output: Path) -> None:
    """Shrink-fit like img2pdf, then stretch edge columns to full page width."""
    page_w, page_h = BOOKLET_SIZE_PX
    image = Image.open(source).convert("RGB")

    scale = min(page_w / image.width, page_h / image.height)
    scaled_w = max(1, round(image.width * scale))
    scaled_h = max(1, round(image.height * scale))
    scaled = image.resize((scaled_w, scaled_h), Image.Resampling.LANCZOS)

    x0 = (page_w - scaled_w) // 2
    y0 = (page_h - scaled_h) // 2
    left_gap = x0
    right_gap = page_w - scaled_w - x0

    page = Image.new("RGB", (page_w, page_h))

    if left_gap > 0:
        left_col = scaled.crop((0, 0, 1, scaled_h))
        page.paste(
            left_col.resize((left_gap, scaled_h), Image.Resampling.NEAREST),
            (0, y0),
        )

    page.paste(scaled, (x0, y0))

    if right_gap > 0:
        right_col = scaled.crop((scaled_w - 1, 0, scaled_w, scaled_h))
        page.paste(
            right_col.resize((right_gap, scaled_h), Image.Resampling.NEAREST),
            (x0 + scaled_w, y0),
        )

    if y0 > 0:
        top_row = scaled.crop((0, 0, scaled_w, 1))
        page.paste(
            top_row.resize((scaled_w, y0), Image.Resampling.NEAREST),
            (x0, 0),
        )
        if left_gap > 0:
            corner = scaled.getpixel((0, 0))
            page.paste(Image.new("RGB", (left_gap, y0), corner), (0, 0))
        if right_gap > 0:
            corner = scaled.getpixel((scaled_w - 1, 0))
            page.paste(Image.new("RGB", (right_gap, y0), corner), (x0 + scaled_w, 0))

    bottom_gap = page_h - scaled_h - y0
    if bottom_gap > 0:
        bottom_row = scaled.crop((0, scaled_h - 1, scaled_w, scaled_h))
        page.paste(
            bottom_row.resize((scaled_w, bottom_gap), Image.Resampling.NEAREST),
            (x0, y0 + scaled_h),
        )
        if left_gap > 0:
            corner = scaled.getpixel((0, scaled_h - 1))
            page.paste(Image.new("RGB", (left_gap, bottom_gap), corner), (0, y0 + scaled_h))
        if right_gap > 0:
            corner = scaled.getpixel((scaled_w - 1, scaled_h - 1))
            page.paste(
                Image.new("RGB", (right_gap, bottom_gap), corner),
                (x0 + scaled_w, y0 + scaled_h),
            )

    page.save(output, format="JPEG", quality=95, dpi=(BOOKLET_DPI, BOOKLET_DPI))


def build_pdf(
    *,
    output: Path,
    pandoc: str = "pandoc",
    pdf_engine: str = "xelatex",
    img2pdf: str = "img2pdf",
    qpdf: str = "qpdf",
    check_only: bool = False,
) -> None:
    del img2pdf, qpdf  # covers are embedded in the LaTeX build (no post-merge).
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

    with tempfile.TemporaryDirectory(prefix="maxx-manual-") as tmp:
        tmp_dir = Path(tmp)
        front_image = tmp_dir / "cover-front-prepared.jpg"
        rear_image = tmp_dir / "cover-rear-prepared.jpg"
        if check_only:
            front_image.write_bytes(b"")
            rear_image.write_bytes(b"")
        else:
            prepare_cover_image(source=front_cover, output=front_image)
            prepare_cover_image(source=rear_cover, output=rear_image)

        build_manual_pdf(
            manual_dir=manual_dir,
            output=output_path,
            front_cover_image=front_image,
            rear_cover_image=rear_image,
            pandoc=pandoc,
            pdf_engine=pdf_engine,
            check_only=check_only,
        )
        if check_only:
            return

        output_path.parent.mkdir(parents=True, exist_ok=True)
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