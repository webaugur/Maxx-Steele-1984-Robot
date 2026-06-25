#!/usr/bin/env python3
"""Report repo file path references in TechnicalManual/*.md that lack markdown hyperlinks."""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
MANUAL = REPO / "TechnicalManual"

# Repo-root-relative path shapes (not exhaustive; tuned for this tree).
PATH_RE = re.compile(
    r"(?<![(\[`])"
    r"(?:\.\./)?"
    r"(?:tools|Cartridge|Chassis|DataSheets|Transmitter|Receiver|Mainboard|"
    r"Power|PaddleMirror|Simulator)/"
    r"[\w./-]+"
    r"|"
    r"(?:\.\./)?Face/(?:KiCAD|Photos|README\.md)"
    r"[\w./-]*"
    r"|"
    r"(?:maxx_internal_ROM|maxx_demo_ROM_532)\.dsm"
    r"|"
    r"(?:cover-front|cover-rear)\.jpg"
    r"|"
    r"Maxx-Steele-Technical-Manual\.pdf"
    r"|"
    r"build_technical_manual_pdf\.py"
    r"|"
    r"tools/maxx_rom\.py"
    r"|"
    r"tools/maxx(?:bas)?(?:/[\w.-]*)?"
)

LINK_RE = re.compile(r"\[[^\]]*\]\([^)]+\)")


def strip_code_fences(text: str) -> str:
    """Remove fenced code blocks (paths inside commands are covered by companion prose)."""
    return re.sub(r"```.*?```", "", text, flags=re.DOTALL)


def line_has_link(line: str, path: str) -> bool:
    for m in LINK_RE.finditer(line):
        target = m.group(0)
        # Link text or URL contains the path (allow basename match for short refs).
        if path in target or Path(path).name in target:
            return True
    return False


def main() -> int:
    issues: list[str] = []

    for md in sorted(MANUAL.glob("*.md")):
        raw = md.read_text(encoding="utf-8")
        prose = strip_code_fences(raw)
        for lineno, line in enumerate(prose.splitlines(), start=1):
            if "http://" in line or "https://" in line:
                continue
            for m in PATH_RE.finditer(line):
                path = m.group(0).lstrip("./")
                if line_has_link(line, path):
                    continue
                issues.append(f"{md.relative_to(REPO)}:{lineno}: {path!r} — {line.strip()[:100]}")

    if issues:
        print("Unlinked repo path references in TechnicalManual:")
        for item in issues:
            print(f"  {item}")
        return 1

    print("OK: all detected repo path references in TechnicalManual prose are hyperlinked.")
    return 0


if __name__ == "__main__":
    sys.exit(main())