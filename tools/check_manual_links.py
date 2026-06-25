#!/usr/bin/env python3
"""Report repo file path references in manual markdown that lack hyperlinks."""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
MANUAL_DIRS = (REPO / "TechnicalManual", REPO / "MechanicalManual")

PATH_RE = re.compile(
    r"(?<![(\[`])"
    r"(?:\.\./)?"
    r"(?:tools|Cartridge|Chassis|DataSheets|Transmitter|Receiver|Mainboard|"
    r"MechanicalManual|TechnicalManual|Power|PaddleMirror)/"
    r"[\w./-]+"
    r"|"
    r"(?:\.\./)?Face/(?:KiCAD|Photos|README\.md)"
    r"[\w./-]*"
    r"|"
    r"(?:maxx_internal_ROM|maxx_demo_ROM_532)\.dsm"
    r"|"
    r"(?:cover-front|cover-rear)\.jpg"
    r"|"
    r"Maxx-Steele-(?:Technical|Mechanical)-Manual\.pdf"
    r"|"
    r"build_(?:technical|mechanical)_manual_pdf\.py"
    r"|"
    r"tools/maxx_rom\.py"
    r"|"
    r"tools/maxx(?:bas)?(?:/[\w.-]*)?"
    r"|"
    r"Chassis/Photos/Disassembly/IMG_\d+\.JPG"
    r"|"
    r"Sources/Maxx-Steele-Disassembly-Guide\.docx"
)

LINK_RE = re.compile(r"\[[^\]]*\]\([^)]+\)")


def strip_code_fences(text: str) -> str:
    return re.sub(r"```.*?```", "", text, flags=re.DOTALL)


def line_has_link(line: str, path: str) -> bool:
    for m in LINK_RE.finditer(line):
        target = m.group(0)
        if path in target or Path(path).name in target:
            return True
    # Bare markdown images: ![alt](Photos/IMG_2116.JPG)
    if f"]({path})" in line or f"]({Path(path).name})" in line:
        return True
    return False


def scan_manual(manual_dir: Path) -> list[str]:
    issues: list[str] = []
    for md in sorted(manual_dir.glob("*.md")):
        raw = md.read_text(encoding="utf-8")
        prose = strip_code_fences(raw)
        for lineno, line in enumerate(prose.splitlines(), start=1):
            if "http://" in line or "https://" in line:
                continue
            for m in PATH_RE.finditer(line):
                path = m.group(0).lstrip("./")
                if line_has_link(line, path):
                    continue
                issues.append(
                    f"{md.relative_to(REPO)}:{lineno}: {path!r} — {line.strip()[:100]}"
                )
    return issues


def main() -> int:
    issues: list[str] = []
    for manual_dir in MANUAL_DIRS:
        if manual_dir.is_dir():
            issues.extend(scan_manual(manual_dir))

    if issues:
        print("Unlinked repo path references in manuals:")
        for item in issues:
            print(f"  {item}")
        return 1

    print("OK: all detected repo path references in manual prose are hyperlinked.")
    return 0


if __name__ == "__main__":
    sys.exit(main())