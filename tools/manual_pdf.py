"""Shared booklet PDF build for Docs/Technical and Docs/Mechanical."""

from __future__ import annotations

import re
import shutil
import subprocess
import tempfile
from dataclasses import dataclass
from datetime import date
from pathlib import Path

from display_codes import convert_display_backticks_to_latex
from PIL import Image, ImageDraw, ImageFont

BOOKLET_WIDTH_IN = 6.5
BOOKLET_HEIGHT_IN = 8.5
BOOKLET_DPI = 300
BOOKLET_SIZE_PX = (
    int(BOOKLET_WIDTH_IN * BOOKLET_DPI),
    int(BOOKLET_HEIGHT_IN * BOOKLET_DPI),
)
BOOKLET_PAGESIZE = f"{BOOKLET_WIDTH_IN}inx{BOOKLET_HEIGHT_IN}in"
COVER_FRONT = "cover-front.jpg"
COVER_REAR = "cover-rear.jpg"

SAME_GUIDE_LINK = re.compile(
    r"\[([^\]]+)\]\((?!https?://)([^)/]+\.md)(#[^)]+)?\)"
)
REPO_LINK = re.compile(r"(?<!!)\[([^\]]+)\]\(\.\./([^)]+)\)")
LOCAL_LINK = re.compile(r"(?<!!)\[([^\]]+)\]\((?!https?://|\.\./|#)([^)]+)\)")
HTTP_LINK = re.compile(r"\[([^\]]+)\]\(https?://[^)]+\)")
FILE_LINK = re.compile(r"\[([^\]]+)\]\((?!https?://)([^)]+\.(?:pdf|md|py|532|64|dsm|docx))\)")
IMAGE_MD = re.compile(r"!\[([^\]]*)\]\((?!https?://|\.\./|#)([^)]+)\)")
IMAGE_REPO_MD = re.compile(r"!\[([^\]]*)\]\(\.\./([^)]+)\)")
RASTER_IMAGE_EXT = {".png", ".jpg", ".jpeg", ".gif", ".webp"}
TABLE_IMAGE_EMBED_MAX_BYTES = 400_000
COVER_IMAGE_NAMES = frozenset({"cover-front.jpg", "cover-rear.jpg"})


@dataclass(frozen=True)
class ManualSpec:
    manual_dir: Path
    chapters: tuple[str, ...]
    output_name: str
    title: str
    author: str = "Maxx-Steele-1984-Robot contributors"
    number_sections: bool = False
    display_message_style: bool = False
    display_font_dir: Path | None = None


def require_tool(name: str) -> str:
    path = shutil.which(name)
    if path is None:
        raise RuntimeError(f"{name} not found on PATH")
    return path


def chapter_anchor_map(
    manual_dir: Path,
    chapters: tuple[str, ...],
    *,
    pandoc: str,
) -> dict[str, str]:
    mapping: dict[str, str] = {}
    for name in chapters:
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
    return str(path.resolve()).replace("\\", "/")


def rear_cover_latex_block(*, cover_pdf: Path) -> str:
    path = latex_path(cover_pdf)
    return f"```{{=latex}}\n\\includepdf[pages=-]{{{path}}}\n```\n\n"


def wrap_with_rear_cover(*, merged: str, rear_cover_pdf: Path) -> str:
    return merged + rear_cover_latex_block(cover_pdf=rear_cover_pdf)


def front_cover_header_includes(front_cover_pdf: Path) -> list[str]:
    path = latex_path(front_cover_pdf)
    return [
        "\\usepackage{pdfpages}",
        f"\\pretocmd{{\\maketitle}}{{\\includepdf[pages=-]{{{path}}}}}{{}}{{}}",
    ]


def _link_label(label: str) -> tuple[str, bool]:
    stripped = label.strip()
    if stripped.startswith("`") and stripped.endswith("`") and len(stripped) >= 2:
        return stripped[1:-1], True
    return stripped, False


def _format_plain_label(label: str) -> str:
    bare, backticks = _link_label(label)
    return f"`{bare}`" if backticks else bare


def _repo_path_label(label: str, path: str) -> str:
    bare, _ = _link_label(label)
    if bare == path or bare.rstrip("/") == path.rstrip("/"):
        return _format_plain_label(label)
    if Path(path).name == bare:
        return _format_plain_label(label)
    return _format_plain_label(label)


def _maybe_embed_raster(
    resolved: Path,
    label: str,
    *,
    in_table: bool,
) -> str | None:
    if not resolved.is_file():
        return None
    if resolved.suffix.lower() not in RASTER_IMAGE_EXT:
        return None
    bare, _ = _link_label(label)
    if resolved.name in COVER_IMAGE_NAMES or bare in COVER_IMAGE_NAMES:
        return _format_plain_label(label)
    if in_table and resolved.stat().st_size > TABLE_IMAGE_EMBED_MAX_BYTES:
        return _format_plain_label(label)
    return f"![{bare}]({resolved})"


def _yaml_header_line(line: str) -> str:
    if "#" in line or ":" in line:
        return f"  - '{line}'"
    return f"  - {line}"


def led_header_includes(*, font_dir: Path | None) -> list[str]:
    font_path = None
    if font_dir is not None:
        candidate = font_dir / "DSEG7Classic-Bold.ttf"
        if candidate.is_file():
            font_path = latex_path(candidate.parent) + "/"
    if font_path:
        return [
            "\\usepackage{fontspec}",
            f"\\newfontfamily\\LEDfont["
            f"Path={font_path},"
            "Extension=.ttf,"
            "UprightFont=DSEG7Classic-Bold,"
            "LetterSpace=1.5"
            "]{DSEG7Classic-Bold}",
            "\\newcommand{\\LED}[1]{{\\begingroup\\color{red}\\LEDfont\\fontsize{18}{22}\\selectfont #1\\endgroup}}",
        ]
    return [
        "\\usepackage{fontspec}",
        "\\newcommand{\\LED}[1]{{\\begingroup\\color{red}\\fontfamily{lmtt}\\fontseries{b}\\fontsize{18}{22}\\selectfont #1\\endgroup}}",
    ]


def preprocess_markdown(
    text: str,
    anchor_map: dict[str, str],
    *,
    manual_dir: Path,
    display_message_style: bool = False,
) -> str:
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
        resolved = (manual_dir / ".." / path).resolve()
        line = text[: match.start()].rsplit("\n", 1)[-1]
        in_table = line.lstrip().startswith("|")
        embedded = _maybe_embed_raster(resolved, label, in_table=in_table)
        if embedded is not None:
            return embedded
        return _repo_path_label(label, path)

    def _local_link(match: re.Match[str]) -> str:
        label, path = match.group(1), match.group(2)
        resolved = (manual_dir / path).resolve()
        line = text[: match.start()].rsplit("\n", 1)[-1]
        in_table = line.lstrip().startswith("|")
        embedded = _maybe_embed_raster(resolved, label, in_table=in_table)
        if embedded is not None:
            return embedded
        return _format_plain_label(label)

    def _http_link(match: re.Match[str]) -> str:
        return match.group(1)

    def _file_link(match: re.Match[str]) -> str:
        return match.group(1)

    def _image_md(match: re.Match[str]) -> str:
        alt, path = match.group(1), match.group(2)
        resolved = (manual_dir / path).resolve()
        if resolved.is_file():
            return f"![{alt}]({resolved})"
        return match.group(0)

    def _image_repo_md(match: re.Match[str]) -> str:
        alt, path = match.group(1), match.group(2)
        resolved = (manual_dir / ".." / path).resolve()
        if resolved.is_file() and resolved.suffix.lower() in RASTER_IMAGE_EXT:
            return f"![{alt}]({resolved})"
        return match.group(0)

    text = SAME_GUIDE_LINK.sub(_same_guide_link, text)
    text = REPO_LINK.sub(_repo_link, text)
    text = LOCAL_LINK.sub(_local_link, text)
    text = IMAGE_REPO_MD.sub(_image_repo_md, text)
    text = IMAGE_MD.sub(_image_md, text)
    text = HTTP_LINK.sub(_http_link, text)
    text = FILE_LINK.sub(_file_link, text)
    text = re.sub(r"\n---\n", "\n\n", text)
    if display_message_style:
        text = convert_display_backticks_to_latex(text)
    return text


def merge_chapters(spec: ManualSpec, *, pandoc: str) -> str:
    anchor_map = chapter_anchor_map(spec.manual_dir, spec.chapters, pandoc=pandoc)
    parts: list[str] = []
    for index, name in enumerate(spec.chapters):
        path = spec.manual_dir / name
        if not path.is_file():
            raise FileNotFoundError(path)
        body = preprocess_markdown(
            path.read_text(encoding="utf-8").strip(),
            anchor_map,
            manual_dir=spec.manual_dir,
            display_message_style=spec.display_message_style,
        )
        if index > 0:
            parts.append(r"\clearpage")
        parts.append(body)
    return "\n\n".join(parts) + "\n"


def build_manual_pdf(
    spec: ManualSpec,
    *,
    output: Path,
    front_cover_pdf: Path,
    rear_cover_pdf: Path,
    pandoc: str,
    pdf_engine: str,
    check_only: bool,
) -> list[str]:
    merged = wrap_with_rear_cover(
        merged=merge_chapters(spec, pandoc=pandoc),
        rear_cover_pdf=rear_cover_pdf,
    )
    today = date.today().isoformat()
    cover_lines = "\n".join(
        f"  - {line}" for line in front_cover_header_includes(front_cover_pdf)
    )
    display_lines = ""
    if spec.display_message_style:
        font_dir = spec.display_font_dir or (spec.manual_dir / "Fonts")
        display_lines = "\n".join(
            _yaml_header_line(line) for line in led_header_includes(font_dir=font_dir)
        )
        if display_lines:
            display_lines = display_lines + "\n"
    if spec.number_sections:
        section_label = r"{\thesection}"
        subsection_label = r"{\thesubsection}"
        subsubsection_label = r"{\thesubsubsection}"
        title_sep = "1em"
        bookmarks_numbered = "true"
    else:
        section_label = ""
        subsection_label = ""
        subsubsection_label = ""
        title_sep = "0em"
        bookmarks_numbered = "false"
    metadata = f"""---
title: "{spec.title}"
author: "{spec.author}"
date: "{today}"
lang: en-US
toc: true
toc-depth: 2
numbersections: {str(spec.number_sections).lower()}
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
  - \\usepackage{{xcolor}}
  - \\definecolor{{maxxnavy}}{{RGB}}{{0,35,102}}
  - \\usepackage{{titlesec}}
  - \\usepackage{{needspace}}
  - \\usepackage{{etoolbox}}
  - \\usepackage{{enumitem}}
  - \\usepackage{{setspace}}
  - \\titleformat{{\\section}}{{\\color{{maxxnavy}}\\bfseries\\Large}}{{{section_label}}}{{{title_sep}}}{{}}
  - \\titleformat{{\\subsection}}{{\\color{{maxxnavy}}\\bfseries\\large}}{{{subsection_label}}}{{{title_sep}}}{{}}
  - \\titleformat{{\\subsubsection}}{{\\color{{maxxnavy}}\\bfseries\\normalsize}}{{{subsubsection_label}}}{{{title_sep}}}{{}}
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
  - \\AtBeginDocument{{\\hypersetup{{colorlinks=true,linkcolor=maxxnavy,urlcolor=maxxnavy,hidelinks=false,bookmarks=true,bookmarksopen=true,bookmarksnumbered={bookmarks_numbered}}}}}
{display_lines}{cover_lines}
---

"""

    input_format = "markdown+raw_tex" if spec.display_message_style else "markdown"
    command = [
        pandoc,
        "-f",
        input_format,
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


def build_cover_pdf(*, image: Path, output: Path, img2pdf: str) -> None:
    with tempfile.NamedTemporaryFile(suffix=".jpg", delete=False) as handle:
        prepared = Path(handle.name)

    try:
        prepare_cover_image(source=image, output=prepared)
        subprocess.run(
            [
                img2pdf,
                "--pagesize",
                BOOKLET_PAGESIZE,
                "--fit",
                "shrink",
                str(prepared),
                "-o",
                str(output),
            ],
            check=True,
        )
    finally:
        prepared.unlink(missing_ok=True)


def build_pdf(
    spec: ManualSpec,
    *,
    output: Path,
    pandoc: str = "pandoc",
    pdf_engine: str = "xelatex",
    img2pdf: str = "img2pdf",
    check_only: bool = False,
) -> None:
    manual_dir = spec.manual_dir
    front_cover = manual_dir / COVER_FRONT
    rear_cover = manual_dir / COVER_REAR

    for path in (front_cover, rear_cover):
        if not path.is_file():
            raise FileNotFoundError(path)

    if shutil.which(pandoc) is None:
        raise RuntimeError(
            f"{pandoc} not found. Install pandoc and a LaTeX engine "
            "(e.g. texlive-xetex) to build the manual PDF."
        )
    if not check_only:
        require_tool(img2pdf)

    with tempfile.TemporaryDirectory(prefix="maxx-manual-") as tmp:
        tmp_dir = Path(tmp)
        front_cover_pdf = tmp_dir / "cover-front.pdf"
        rear_cover_pdf = tmp_dir / "cover-rear.pdf"
        if check_only:
            front_cover_pdf.write_bytes(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n")
            rear_cover_pdf.write_bytes(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n")
        else:
            build_cover_pdf(image=front_cover, output=front_cover_pdf, img2pdf=img2pdf)
            build_cover_pdf(image=rear_cover, output=rear_cover_pdf, img2pdf=img2pdf)

        build_manual_pdf(
            spec,
            output=output,
            front_cover_pdf=front_cover_pdf,
            rear_cover_pdf=rear_cover_pdf,
            pandoc=pandoc,
            pdf_engine=pdf_engine,
            check_only=check_only,
        )
        if check_only:
            return

        output.parent.mkdir(parents=True, exist_ok=True)
        print(output)


def make_title_cover(
    *,
    output: Path,
    title: str,
    subtitle: str,
    background: Path | None = None,
) -> None:
    """Create a 6.5x8.5 booklet cover JPEG (navy title card, optional photo background)."""
    page_w, page_h = BOOKLET_SIZE_PX
    navy = (0, 35, 102)

    if background and background.is_file():
        page = Image.open(background).convert("RGB")
        page = page.resize((page_w, page_h), Image.Resampling.LANCZOS)
        overlay = Image.new("RGBA", (page_w, page_h), (*navy, 170))
        page = Image.alpha_composite(page.convert("RGBA"), overlay).convert("RGB")
    else:
        page = Image.new("RGB", (page_w, page_h), navy)

    draw = ImageDraw.Draw(page)
    try:
        title_font = ImageFont.truetype("DejaVuSans-Bold.ttf", 96)
        sub_font = ImageFont.truetype("DejaVuSans.ttf", 48)
    except OSError:
        title_font = ImageFont.load_default()
        sub_font = ImageFont.load_default()

    draw.multiline_text(
        (page_w // 2, page_h // 2 - 80),
        title,
        font=title_font,
        fill=(255, 255, 255),
        anchor="mm",
        align="center",
        spacing=12,
    )
    draw.text(
        (page_w // 2, page_h // 2 + 120),
        subtitle,
        font=sub_font,
        fill=(200, 220, 255),
        anchor="mm",
    )
    page.save(output, format="JPEG", quality=95, dpi=(BOOKLET_DPI, BOOKLET_DPI))