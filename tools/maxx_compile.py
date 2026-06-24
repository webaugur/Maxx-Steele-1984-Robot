#!/usr/bin/env python3
"""Compile MaxxBAS — thin wrapper around the unified maxx toolchain.

Prefer:  python3 tools/maxx compile SOURCE [-o OUT]
Legacy:  python3 tools/maxx_compile.py SOURCE [-o OUT]
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

from project_paths import project_root


def main(argv: list[str] | None = None) -> int:
    maxx = project_root() / "tools" / "maxx"
    args = list(argv) if argv is not None else sys.argv[1:]
    return subprocess.call([sys.executable, str(maxx), "compile", *args])


if __name__ == "__main__":
    raise SystemExit(main())