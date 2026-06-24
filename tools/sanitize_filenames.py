#!/usr/bin/env python3
"""Rename repository files to shell-safe names (no spaces or special characters).

Allowed characters in basenames: ASCII letters, digits, dot, underscore, hyphen.

Usage:
    python3 tools/sanitize_filenames.py              # dry-run
    python3 tools/sanitize_filenames.py --apply      # git mv renames
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

from project_paths import project_root

SAFE_PATTERN = re.compile(r"^[a-zA-Z0-9._-]+$")
PARENS_NUM = re.compile(r"\((\d+)\)")
SKIP_DIRS = {".git", "__pycache__"}


def sanitize_basename(name: str) -> str:
    """Return a shell-safe filename; preserves extension."""
    path = Path(name)
    suffixes = "".join(path.suffixes)
    if suffixes:
        stem = name[: -len(suffixes)]
    else:
        stem = name
        suffixes = ""

    stem = stem.replace("&", "and")
    stem = PARENS_NUM.sub(r"-\1", stem)
    stem = re.sub(r"[^\w.-]", "-", stem, flags=re.ASCII)
    stem = re.sub(r"\s+", "-", stem)
    stem = re.sub(r"-+", "-", stem)
    stem = stem.strip("-.")

    if not stem:
        raise ValueError(f"cannot sanitize filename: {name!r}")

    return stem + suffixes


def needs_rename(path: Path) -> bool:
    return not SAFE_PATTERN.match(path.name)


def iter_files(root: Path) -> list[Path]:
    files: list[Path] = []
    for entry in root.rglob("*"):
        if entry.is_file() and not any(part in SKIP_DIRS for part in entry.parts):
            files.append(entry)
    return sorted(files)


def plan_renames(root: Path) -> list[tuple[Path, Path]]:
    planned: list[tuple[Path, Path]] = []
    targets: dict[Path, Path] = {}

    for src in iter_files(root):
        if not needs_rename(src):
            continue
        dst = src.with_name(sanitize_basename(src.name))
        if src == dst:
            continue
        if dst.exists() and dst != src:
            raise RuntimeError(f"collision: {src} -> {dst} (target exists)")
        if dst in targets and targets[dst] != src:
            raise RuntimeError(f"collision: {src} and {targets[dst]} -> {dst}")
        targets[dst] = src
        planned.append((src, dst))

    return planned


def git_mv(src: Path, dst: Path) -> None:
    dst.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(["git", "mv", str(src), str(dst)], check=True, cwd=project_root())


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="perform git mv renames")
    args = parser.parse_args()

    root = project_root()
    planned = plan_renames(root)
    if not planned:
        print("All filenames are already shell-safe.")
        return 0

    print(f"{'APPLY' if args.apply else 'DRY-RUN'}: {len(planned)} rename(s)")
    for src, dst in planned:
        rel_src = src.relative_to(root)
        rel_dst = dst.relative_to(root)
        print(f"  {rel_src} -> {rel_dst}")

    if args.apply:
        for src, dst in planned:
            git_mv(src, dst)

    return 0


if __name__ == "__main__":
    sys.exit(main())