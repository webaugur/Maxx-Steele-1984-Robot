#!/usr/bin/env python3
"""Generate Docs/User chapter markdown from OCR text and factory page scans.

By default this script does nothing — chapter .md files are edited directly.
Pass --from-sources to overwrite chapters from Docs/User/Sources/text (bootstrap
or re-OCR only). See tools/bootstrap_user_manual_from_pdf.py for the full chain.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

from display_codes import wrap_display_codes
from manual_paths import USER_MANUAL
from project_paths import project_root

MANUAL_DIR = USER_MANUAL
TEXT_DIR = MANUAL_DIR / "Sources/text"
PAGES_DIR = MANUAL_DIR / "Sources/pages"
OUTLINE = MANUAL_DIR / "Sources/outline.json"

# OCR noise and factory manual typos (ordered: longer matches first).
REPLACEMENTS: tuple[tuple[str, str], ...] = (
    (r"\bMOTE\b", "NOTE"),
    (r"\bMoxx\b", "Maxx"),
    (r"\bNoxx\b", "Maxx"),
    (r"\bFROMPT\b", "PROMPT"),
    (r"\bclack\b", "clock"),
    (r"\bClack\b", "Clock"),
    (r"\bMade\b", "Mode"),
    (r"\bmation\b", "motion"),
    (r"\bthot\b", "that"),
    (r"\bfo\b", "to"),
    (r"\bont\b", "not"),
    (r"\bore\b", "are"),
    (r"\bwerds\b", "words"),
    (r"\byau\b", "you"),
    (r"\bcon\b", "can"),
    (r"\bmemary\b", "memory"),
    (r"\bExecytion\b", "Execution"),
    (r"\bCONTINUQUS\b", "CONTINUOUS"),
    (r"\bPO'WER\b", "POWER"),
    (r"\bCPOWER\b", "POWER"),
    (r"\bKeybar:\s*", "Key: "),
    (r"\bPower/Stop\b", "`<POWER/STOP>`"),
    (r"<POWER/STOP=", "<POWER/STOP>"),
    (r"<PROGRAM-\d*", "<PROGRAM>"),
    (r"<ARMS DOWN\?", "<ARMS DOWN>"),
    (r"<WRIST DOWN\b", "<WRIST DOWN>"),
    (r"<TURN RIGHT=", "<DRIVE RIGHT>"),
    (r"\[CSONG>", "<SONG>"),
    (r"\[SONG\b", "<SONG>"),
    (r"<—GAME>", "<GAME>"),
    (r"<SPEECH™>", "<SPEECH>"),
    (r"<PROGRAM\*", "<PROGRAM>"),
    (r"\|", "I"),
    (r"''", "'"),
    (r"''", '"'),
    (r"  +", " "),
)

HEADING_PATTERNS: tuple[tuple[re.Pattern[str], str], ...] = (
    (re.compile(r"^1\.\s+INTRODUCTION$", re.I), "## 1. Introduction"),
    (re.compile(r"^2\.\s+SETUP$", re.I), "## 2. Setup"),
    (re.compile(r"^2\.1\s+UNPACKING$", re.I), "### 2.1 Unpacking"),
    (re.compile(r"^2\.2\s+ACTIVATING", re.I), "### 2.2 Activating Maxx's power"),
    (re.compile(r"^2\.3\s+CHARGING", re.I), "### 2.3 Charging Maxx"),
    (re.compile(r"^2\.4\s+THE CONTROLLER$", re.I), "### 2.4 The controller"),
    (re.compile(r"^3\.\s+STARTING MAXX$", re.I), "## 3. Starting Maxx"),
    (re.compile(r"^4,?\s+MODES OF OPERATION$", re.I), "## 4. Modes of operation"),
    (re.compile(r"^4\.1\s+IMMEDIATE MODE$", re.I), "### 4.1 Immediate mode"),
    (re.compile(r"^4\.11\s+TIME$", re.I), "#### 4.11 Time"),
    (re.compile(r"^4\.11A\s+SETTING", re.I), "##### 4.11A Setting the time"),
    (re.compile(r"^4\.11[68]\s+SETTING", re.I), "##### 4.11B Setting the alarm"),
    (re.compile(r"^4\.12\s+MOTION$", re.I), "#### 4.12 Motion"),
    (re.compile(r"^4\.12A\s+MOVING", re.I), "##### 4.12A Moving arms, wrist, and claw"),
    (re.compile(r"^4\.12B\s+MOVING", re.I), "##### 4.12B Moving forward and backward"),
    (re.compile(r"^4\.12C\s+TURNING$", re.I), "##### 4.12C Turning"),
    (re.compile(r"^4\.13\s+SPEECH$", re.I), "#### 4.13 Speech"),
    (re.compile(r"^4\.13A\s+PROGRAMMED", re.I), "##### 4.13A Programmed phrases"),
    (re.compile(r"^4\.13B\s+PROGRAMMED", re.I), "##### 4.13B Pre-programmed words"),
    (re.compile(r"^4\.14\s+MUSIC$", re.I), "#### 4.14 Music"),
    (re.compile(r"^4\.14A\s+PROGRAMMED", re.I), "##### 4.14A Pre-programmed songs"),
    (re.compile(r"^4\.14B\s+USING", re.I), "##### 4.14B Using the controller as a musical instrument"),
    (re.compile(r"^4\.14C?\s+OCTAVE", re.I), "##### 4.14C Octave shift"),
    (re.compile(r"^4\.2\s+PROGRAM MODE$", re.I), "### 4.2 Program mode"),
    (re.compile(r"^4\.21\s+PROGRAMMING ARM", re.I), "#### 4.21 Programming arm movements"),
    (re.compile(r"^4\.22\s+PROGRAMMING.*WRIST", re.I), "#### 4.22 Programming wrist movements"),
    (re.compile(r"^4\.23\s+PROGRAMMING.*CLAW", re.I), "#### 4.23 Programming claw movements"),
    (re.compile(r"^4\.24\s+PROGRAMMING.*BODY", re.I), "#### 4.24 Programming body movements"),
    (re.compile(r"^4\.25\s+PROGRAMMING", re.I), "#### 4.25 Programming the headlamp"),
    (re.compile(r"^4\.26\s+PROGRAMMING", re.I), "#### 4.26 Programming a delay"),
    (re.compile(r"^4\.27\s+CLEARING PROGRAM STEPS$", re.I), "#### 4.27 Clearing program steps"),
    (re.compile(r"^4\.28\s+CLEARING AN", re.I), "#### 4.28 Clearing an entire program"),
    (re.compile(r"^4\.29\s+RUNNING", re.I), "#### 4.29 Running a program"),
    (re.compile(r"^4\.3\s+LEARN MODE$", re.I), "### 4.3 Learn mode"),
    (re.compile(r"^4\.31\s+USING", re.I), "#### 4.31 Using the learn mode"),
    (re.compile(r"^4\.32\s+CLEARING A", re.I), "#### 4.32 Clearing a program step"),
    (re.compile(r"^4\.33\s+CLEARING AN", re.I), "#### 4.33 Clearing an entire program"),
    (re.compile(r"^4\.4\s+EXECUTE MODE$", re.I), "### 4.4 Execute mode"),
    (re.compile(r"^4\.41\s+EXECUTING", re.I), "#### 4.41 Executing a program"),
    (re.compile(r"^4\.42\s+STOPPING", re.I), "#### 4.42 Stopping program execution"),
    (re.compile(r"^4\.43\s+PAUSING", re.I), "#### 4.43 Pausing during program execution"),
    (re.compile(r"^4\.5\s+GAME", re.I), "### 4.5 Game-playing mode"),
    (re.compile(r"^4\.51\s+MOON", re.I), "#### 4.51 Moon Ball™"),
    (re.compile(r"^4\.52\s+FORCE", re.I), "#### 4.52 Force Field™"),
    (re.compile(r"^4\.6\s+OTHER", re.I), "### 4.6 Other operating modes"),
    (re.compile(r"^4\.61\s+PROGRAM\s+SONG", re.I), "#### 4.61 Program song mode"),
    (re.compile(r"^4\.61A\s+PROGRAMMING", re.I), "##### 4.61A Programming rests"),
    (re.compile(r"^4\.61B\s+OCTAVE", re.I), "##### 4.61B Octave shift"),
    (re.compile(r"^4\.61C\s+PLAYING", re.I), "##### 4.61C Playing a programmed song"),
    (re.compile(r"^4\.61D\s+CLEARING", re.I), "##### 4.61D Clearing mistakes"),
    (re.compile(r"^4\.61E\s+CLEARING", re.I), "##### 4.61E Clearing an entire song"),
    (re.compile(r"^4\.62\s+LEARN SONG", re.I), "#### 4.62 Learn song mode"),
    (re.compile(r"^4\.62A\s+MAXX", re.I), "##### 4.62A Maxx's music memory"),
    (re.compile(r"^4\.62B\s+OCTAVE", re.I), "##### 4.62B Octave shift"),
    (re.compile(r"^4\.62C\s+PLAYING", re.I), "##### 4.62C Playing a learned song"),
    (re.compile(r"^4\.63\s+PROGRAM SPEECH", re.I), "#### 4.63 Program speech mode"),
    (re.compile(r"^4\.63A\s+ENTERING", re.I), "##### 4.63A Entering pauses"),
    (re.compile(r"^4\.63B\s+CLEARING", re.I), "##### 4.63B Clearing words"),
    (re.compile(r"^5\.\s+ADVANCED", re.I), "## 5. Advanced features"),
    (re.compile(r"^5\.1\s+AUTO", re.I), "### 5.1 Auto-execute"),
    (re.compile(r"^5\.2\s+PARALLEL", re.I), "### 5.2 Parallel program execution"),
    (re.compile(r"^5\.21\s+STARTING", re.I), "#### 5.21 Starting a parallel program"),
    (re.compile(r"^5\.22\s+RETURNING", re.I), "#### 5.22 Returning to the serial execution mode"),
    (re.compile(r"^5\.3\s+POWER", re.I), "### 5.3 Power-down"),
    (re.compile(r"^5\.31\s+AUTOMATIC", re.I), "#### 5.31 Automatic power-down"),
    (re.compile(r"^5\.32\s+MANUAL", re.I), "#### 5.32 Manual power-down"),
    (re.compile(r"^5\.4\s+PROGRAM EDITING$", re.I), "### 5.4 Program editing"),
    (re.compile(r"^5\.41\s+USING", re.I), "#### 5.41 Using the edit mode"),
    (re.compile(r"^5\.42\s+PAUSING$", re.I), "#### 5.42 Pausing"),
    (re.compile(r"^5\.5\s+CONTROLLABLE", re.I), "### 5.5 Controllable wrist/arm coupling"),
    (re.compile(r"^5\.6\s+FORWARD", re.I), "### 5.6 Forward and backward program execution"),
    (re.compile(r"^5\.7\s+VERBAL", re.I), "### 5.7 Verbal prompts"),
    (re.compile(r"^5\.8\s+VOLUME", re.I), "### 5.8 Volume adjustment"),
    (re.compile(r"^A\.\s+TECHNICAL", re.I), "## Appendix A — Technical data/specifications"),
    (re.compile(r"^B\.\s+GLOSSARY OF TECHNICAL", re.I), "## Appendix B — Glossary of technical terms"),
    (re.compile(r"^C\.\s+PRE-PROGRAMMED\s+SONGS", re.I), "## Appendix C — Pre-programmed songs"),
    (re.compile(r"^D\.\s+PRE-PROGRAMMED\s+PHRASES", re.I), "## Appendix D — Pre-programmed phrases"),
    (re.compile(r"^E\.\s+GLOSSARY OF", re.I), "## Appendix E — Glossary of prompts and display messages"),
    (re.compile(r"^F\.\s+SPEECH CODES", re.I), "## Appendix F — Speech codes"),
    (re.compile(r"^G\.?\s+ROUTINE", re.I), "## Appendix G — Routine maintenance"),
    (re.compile(r"^H\.\s+NORMAL", re.I), "## Appendix H — Normal operating conditions"),
    (re.compile(r"^I\.\s+CAUTIONS$", re.I), "## I. Cautions"),
    (re.compile(r"^J\.\s+SUMMARY", re.I), "## Appendix J — Summary of controller functions and special keys"),
    (re.compile(r"^K\.\s+F\.?C\.?C", re.I), "## Appendix K — FCC warning on Class B products"),
)

SKIP_LINE_RE = re.compile(
    r"^(?:"
    r"PROMPT FOR|"
    r"PROGRAM STEP|"
    r"DISPLAY OFF|DISPLAY ON|"
    r"INDICATOR|"
    r"CHARGER$|POWER SWITCH$|"
    r"STORAGE COMPARTMENT$|"
    r"LED POWER INDICATOR$|"
    r"MASTER PHOTO|"
    r"DALEK|DALSX|LAI CNS|L\.CONTENTS|"
    r"^[A-Z]{1,3}$|"
    r"^[\W_]{1,6}$|"
    r"^\d+\.\s+[A-Z]{2,}\s*$"
    r")",
    re.I,
)

BULLET_START_RE = re.compile(r"^[\d®•\*#]\s*[\.\)]\s+")
KEY_RE = re.compile(r"(?<![`])<([^>]+)>(?!`)")
SECTION_NUM_RE = re.compile(r"^(\d+\.\d+[A-Z]?)\s+")


def canonicalize_key(raw: str) -> str:
    """Map OCR controller-key variants to factory names."""
    key = raw.strip()
    key = re.sub(r"\s+", " ", key)
    key = key.replace("PO'WER", "POWER").replace("CPOWER", "POWER")
    key = re.sub(r"[®™]+", "", key)
    key = re.sub(r"^—+", "", key)
    key = re.sub(r"[*]+$", "", key)
    key = re.sub(r"[=>]+$", "", key).strip()
    key = re.sub(r"\bDRIVE\s+DRIVE\b", "DRIVE", key, flags=re.I)

    rules: tuple[tuple[str, str], ...] = (
        (r"^C?POWER\s*/\s*STOP", "POWER/STOP"),
        (r"^CENTER", "ENTER"),
        (r"^\{\.?ENTER", "ENTER"),
        (r"^FORW\s*ARD", "DRIVE FORWARD"),
        (r"^DRIVE\s+FORWARD", "DRIVE FORWARD"),
        (r"^REVERSE$", "DRIVE REVERSE"),
        (r"^DRIVE\s+REVERSE", "DRIVE REVERSE"),
        (r"^DRIVE\s+BACK(?:\s*WARD)?", "DRIVE REVERSE"),
        (r"^TURN\s+LEFT", "DRIVE LEFT"),
        (r"^DRIVE\s+LEFT", "DRIVE LEFT"),
        (r"^TURN\s+RIGHT", "DRIVE RIGHT"),
        (r"^DRIVE\s+RIGHT", "DRIVE RIGHT"),
        (r"^WRIST\s+UP", "WRIST UP"),
        (r"^WRIST\s+DOWN", "WRIST DOWN"),
        (r"^ARMS\s+UP", "ARMS UP"),
        (r"^ARMS\s+DOWN", "ARMS DOWN"),
        (r"^CLAW\s+ROTATE", "CLAW ROTATE"),
        (r"^CLAW\s+CLOSE\s*/\s*OPEN", "CLAW CLOSE/OPEN"),
        (r"^LAMP(?:\s+ON\s*/\s*OFF)?", "LAMP ON/OFF"),
        (r"^HOME", "HOME"),
        (r"^NOTE\s+REST\s*/\s*WAIT", "NOTE REST/WAIT"),
        (r"^WAIT", "NOTE REST/WAIT"),
        (r"^SHIFT\s+OCTAVE", "SHIFT OCTAVE"),
        (r"^OCTAVE", "SHIFT OCTAVE"),
        (r"^BACK\s*STEP", "MOTION"),
        (r"^BACKSTEP", "MOTION"),
        (r"^CLEAR", "CLEAR"),
        (r"^ENTER", "ENTER"),
        (r"^SONG", "SONG"),
        (r"^CLOCK", "CLOCK"),
        (r"^SPEECH", "SPEECH"),
        (r"^MOTION", "MOTION"),
        (r"^GAME", "GAME"),
        (r"^PROGRAM", "PROGRAM"),
        (r"^LEARN", "LEARN"),
        (r"^EXECUTE", "EXECUTE"),
        (r"^CSONG", "SONG"),
    )

    for pattern, canonical in rules:
        if re.match(pattern, key, re.I):
            return canonical

    if re.fullmatch(r"\d", key):
        return key

    return key.upper() if key.isascii() and key.isalpha() else key


def load_outline(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def load_page_text(page: int, text_dir: Path) -> str:
    path = text_dir / f"page-{page:02d}.txt"
    if not path.is_file():
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def apply_replacements(text: str) -> str:
    for pattern, repl in REPLACEMENTS:
        text = re.sub(pattern, repl, text)
    return text


def normalize_keys(text: str) -> str:
    """Canonicalize controller key names and wrap in backticks for pandoc PDF."""

    def fix_key(match: re.Match[str]) -> str:
        canonical = canonicalize_key(match.group(1))
        # Backticks prevent pandoc from treating <DRIVE LEFT> as raw HTML.
        return f"`<{canonical}>`"

    return KEY_RE.sub(fix_key, text)


def maybe_heading(line: str) -> str | None:
    stripped = line.strip()
    if not stripped or len(stripped) > 80:
        return None
    for pattern, heading in HEADING_PATTERNS:
        if pattern.search(stripped):
            return heading
    if SECTION_NUM_RE.match(stripped) and stripped.isupper():
        return f"### {stripped.title()}"
    return None


def is_noise_line(line: str) -> bool:
    stripped = line.strip()
    if not stripped:
        return False
    if SKIP_LINE_RE.match(stripped):
        return True
    if len(stripped) <= 2 and not stripped.isdigit():
        return True
    if re.fullmatch(r"[\W\d]+", stripped):
        return True
    return False


def format_line(line: str) -> str | None:
    line = apply_replacements(line)
    line = normalize_keys(line)
    line = wrap_display_codes(line)
    stripped = line.strip()
    if not stripped:
        return ""
    if is_noise_line(stripped):
        return None
    heading = maybe_heading(stripped)
    if heading:
        return f"\n{heading}\n"
    if stripped.startswith("NOTE:") or stripped.startswith("IMPORTANT:"):
        return f"\n> {stripped}\n"
    if BULLET_START_RE.match(stripped):
        body = BULLET_START_RE.sub("", stripped, count=1).strip()
        return f"- {body}"
    return stripped


def split_page_text(text: str, marker: str | None, take: str | None) -> str:
    if not marker or marker not in text:
        return text
    before, after = text.split(marker, 1)
    if take == "before_split":
        return before
    if take == "from_split":
        return marker + after
    return text


def page_figure_md(page: int, caption: str, pages_dir: Path) -> str:
    rel = f"Sources/pages/page-{page:02d}.png"
    return f"\n![{caption}]({rel})\n"


FIGURE_ONLY_PAGES = frozenset({4, 5, 8, 43, 44, 45, 46, 47})


def pages_to_markdown(
    pages: list[int],
    *,
    text_dir: Path,
    pages_dir: Path,
    figures: list[dict],
    page_splits: dict[str, str] | None,
    take: str | None,
    seen_pages: set[tuple[int, str | None]],
) -> str:
    figure_pages = {f["page"]: f["caption"] for f in figures}
    blocks: list[str] = []

    for page in pages:
        marker = (page_splits or {}).get(str(page))
        visit_key = (page, take if marker else None)
        if visit_key in seen_pages:
            continue
        seen_pages.add(visit_key)

        if page in figure_pages:
            blocks.append(page_figure_md(page, figure_pages[page], pages_dir))

        if page in FIGURE_ONLY_PAGES and page in figure_pages:
            continue

        raw = load_page_text(page, text_dir)
        if marker:
            raw = split_page_text(raw, marker, take)
        if not raw.strip():
            continue

        paragraph: list[str] = []
        for line in raw.splitlines():
            formatted = format_line(line)
            if formatted is None:
                continue
            if formatted == "":
                if paragraph:
                    blocks.append("\n".join(paragraph))
                    paragraph = []
                continue
            if formatted.startswith("\n#"):
                if paragraph:
                    blocks.append("\n".join(paragraph))
                    paragraph = []
                blocks.append(formatted.strip())
                continue
            if formatted.startswith("\n>"):
                if paragraph:
                    blocks.append("\n".join(paragraph))
                    paragraph = []
                blocks.append(formatted.strip())
                continue
            paragraph.append(formatted)
        if paragraph:
            blocks.append("\n".join(paragraph))

    body = "\n\n".join(blocks)
    body = re.sub(r"\n{3,}", "\n\n", body)
    return body.strip()


def chapter_preamble(filename: str) -> str:
    extras = {
        "02-Setup.md": (
            "Cross-reference: factory keypad labels and matrix map in "
            "[`Transmitter/remote-keypad.md`](../../Transmitter/remote-keypad.md).\n\n"
        ),
        "04-Immediate-Mode.md": (
            "The five factory operating modes correspond to the internal ROM mode "
            "table documented in "
            "[`Docs/Technical/05-Cartridge-Bootstrap-and-Internal-ROM.md`]"
            "(../Technical/05-Cartridge-Bootstrap-and-Internal-ROM.md).\n\n"
        ),
        "09-Compliance.md": (
            "Archival scan of the factory booklet: "
            "[`Chassis/Manual/MaxxSteeleManual.pdf`](../../Chassis/Manual/MaxxSteeleManual.pdf). "
            "Programmer quick reference: "
            "[`Chassis/Manual/MaxxSteeleReferenceGuide.pdf`]"
            "(../../Chassis/Manual/MaxxSteeleReferenceGuide.pdf).\n\n"
        ),
    }
    return extras.get(filename, "")


def write_chapter(
    ch: dict,
    *,
    manual_dir: Path,
    text_dir: Path,
    pages_dir: Path,
    seen_pages: set[tuple[int, str | None]],
) -> Path:
    body = pages_to_markdown(
        ch["pages"],
        text_dir=text_dir,
        pages_dir=pages_dir,
        figures=ch.get("figures", []),
        page_splits=ch.get("page_splits"),
        take=ch.get("take"),
        seen_pages=seen_pages,
    )
    preamble = chapter_preamble(ch["file"])
    content = f"# {ch['title']}\n\n{preamble}{body}\n"
    out = manual_dir / ch["file"]
    out.write_text(content, encoding="utf-8")
    return out


def write_readme(manual_dir: Path, outline: dict) -> Path:
    chapter_links = "\n".join(
        f"{i}. [{ch['title'].replace('Chapter ', '').split(' — ', 1)[-1]}]({ch['file']})"
        for i, ch in enumerate(outline["chapters"], start=1)
    )
    text = f"""# Maxx Steele User Manual

Community edition of the 1984 CBS Toys / Ideal **Electronic Maxx Steele® Personal Robot** factory *User's Guide to Operations*, transcribed from [`Chassis/Manual/MaxxSteeleManual.pdf`](../../Chassis/Manual/MaxxSteeleManual.pdf).

**PDF (latest):** [`Maxx-Steele-User-Manual.pdf`](Maxx-Steele-User-Manual.pdf) — rebuilt when `.md` or cover images change ([`tools/build_user_manual_pdf.py`](../../tools/build_user_manual_pdf.py) locally; GitHub Actions on push to `main`).

## What this guide covers

| Document | Audience | Focus |
|----------|----------|-------|
| **This guide** | Owners | Setup, remote control, five operating modes, games, maintenance |
| [`Docs/Technical/`](../Technical/) | Programmers | Bytecode, ROM, I/O, cartridge authoring |
| [`Docs/Mechanical/`](../Mechanical/) | Repairers | Disassembly, reassembly, chassis photos |
| [`Chassis/Manual/MaxxSteeleReferenceGuide.pdf`](../../Chassis/Manual/MaxxSteeleReferenceGuide.pdf) | Owners | Six-page factory programmer quick reference |

## How to use this manual

{chapter_links}

## Editing and building

**Source of truth:** the chapter `.md` files in this folder. Edit them directly, then rebuild the PDF:

```bash
python3 tools/build_user_manual_pdf.py
```

GitHub Actions runs the same build on push when `Docs/User/**` changes.

## Bootstrap from factory PDF (optional)

Archival scan: [`Chassis/Manual/MaxxSteeleManual.pdf`](../../Chassis/Manual/MaxxSteeleManual.pdf). To re-seed markdown from that PDF (overwrites chapter text from OCR):

```bash
python3 tools/bootstrap_user_manual_from_pdf.py
```

Or step-by-step: `extract_manual_pages.py` → `ocr_manual_pages.py` → `gen_user_manual_chapters.py --from-sources`.

| Path | Description |
|------|-------------|
| [`Sources/pages/`](Sources/pages/) | Page scans (figures + OCR input; optional bootstrap) |
| [`Sources/text/`](Sources/text/) | Per-page OCR text (bootstrap input only) |
| [`Sources/outline.json`](Sources/outline.json) | Page-to-chapter map for the generator |

For keypad matrix names and faceplate art, see [`Transmitter/remote-keypad.md`](../../Transmitter/remote-keypad.md).
"""
    out = manual_dir / "README.md"
    out.write_text(text, encoding="utf-8")
    return out


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--manual", type=Path, default=MANUAL_DIR)
    ap.add_argument("--outline", type=Path, default=OUTLINE)
    ap.add_argument(
        "--from-sources",
        action="store_true",
        help="Overwrite chapter .md from OCR (destructive; bootstrap only)",
    )
    ap.add_argument(
        "--include-readme",
        action="store_true",
        help="Also overwrite README.md (only with --from-sources)",
    )
    args = ap.parse_args(argv)

    if not args.from_sources:
        print(
            "No action taken. Chapter markdown is edited directly under Docs/User/.\n"
            "To rebuild the PDF: python3 tools/build_user_manual_pdf.py\n"
            "To re-generate chapters from OCR: add --from-sources\n"
            "Full bootstrap from factory PDF: python3 tools/bootstrap_user_manual_from_pdf.py",
            file=sys.stderr,
        )
        return 0

    root = project_root()
    manual_dir = args.manual if args.manual.is_absolute() else root / args.manual
    outline_path = args.outline if args.outline.is_absolute() else root / args.outline
    text_dir = manual_dir / "Sources/text"
    pages_dir = manual_dir / "Sources/pages"

    if not outline_path.is_file():
        print(f"error: missing {outline_path}", file=sys.stderr)
        return 1
    if not text_dir.is_dir() or not any(text_dir.glob("page-*.txt")):
        print(
            f"error: no OCR text in {text_dir} — run tools/ocr_manual_pages.py first",
            file=sys.stderr,
        )
        return 1

    outline = load_outline(outline_path)
    seen_pages: set[tuple[int, str | None]] = set()
    for ch in outline["chapters"]:
        path = write_chapter(
            ch,
            manual_dir=manual_dir,
            text_dir=text_dir,
            pages_dir=pages_dir,
            seen_pages=seen_pages,
        )
        print(path.relative_to(root))

    if args.include_readme:
        readme = write_readme(manual_dir, outline)
        print(readme.relative_to(root))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())