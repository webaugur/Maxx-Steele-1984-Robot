#!/usr/bin/env python3
"""Optional one-shot bootstrap: factory PDF → page scans → OCR → chapter markdown.

Use only when re-seeding from Chassis/Manual/MaxxSteeleManual.pdf. Normal edits
should change UserManual/*.md directly and run build_user_manual_pdf.py.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

from project_paths import project_root


def run(script: str, *args: str) -> None:
    cmd = [sys.executable, str(project_root() / "tools" / script), *args]
    print("+", " ".join(cmd), flush=True)
    subprocess.run(cmd, check=True)


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--pdf",
        type=Path,
        default=Path("Chassis/Manual/MaxxSteeleManual.pdf"),
        help="Factory manual PDF (default: Chassis/Manual/MaxxSteeleManual.pdf)",
    )
    ap.add_argument("--dpi", type=int, default=300)
    ap.add_argument(
        "--include-readme",
        action="store_true",
        help="Also regenerate UserManual/README.md from the generator template",
    )
    ap.add_argument(
        "--yes",
        action="store_true",
        help="Skip confirmation prompt",
    )
    args = ap.parse_args(argv)

    print(
        "This overwrites chapter markdown under UserManual/ from OCR.\n"
        "Hand-edited prose will be lost unless you have committed or backed it up.",
        file=sys.stderr,
    )
    if not args.yes:
        answer = input("Continue? [y/N] ").strip().lower()
        if answer not in {"y", "yes"}:
            print("Aborted.", file=sys.stderr)
            return 1

    pdf_args = ["--pdf", str(args.pdf), "--dpi", str(args.dpi)]
    run("extract_manual_pages.py", *pdf_args)
    run("ocr_manual_pages.py")

    gen_args = ["--from-sources"]
    if args.include_readme:
        gen_args.append("--include-readme")
    run("gen_user_manual_chapters.py", *gen_args)

    print(
        "\nBootstrap complete. Review diffs, edit chapters as needed, then:\n"
        "  python3 tools/build_user_manual_pdf.py",
        flush=True,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())