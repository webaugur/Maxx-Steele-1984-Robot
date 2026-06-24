#!/usr/bin/env python3
"""Helpers for invoking the Rust maxx toolchain from Python scripts."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
from pathlib import Path

from project_paths import project_root

SOURCE_SUFFIXES = {".bas", ".maxx"}
MAXX = project_root() / "tools" / "maxx"
RELEASE_BIN = project_root() / "tools" / "maxxbas" / "target" / "release" / "maxx"


def is_maxxbas_source(path: Path) -> bool:
    return path.suffix.lower() in SOURCE_SUFFIXES


def maxx_binary() -> Path:
    if RELEASE_BIN.is_file():
        return RELEASE_BIN
    return MAXX


def run_maxx(argv: list[str], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    cmd = [sys.executable, str(MAXX), *argv]
    env = os.environ.copy()
    tools = str(project_root() / "tools")
    env["PATH"] = f"{tools}{os.pathsep}{env.get('PATH', '')}"
    return subprocess.run(cmd, check=check, text=True, capture_output=False, env=env)


def compile_source(
    source: Path,
    output: Path | None = None,
    *,
    copyright: str = "ultramaxx",
) -> Path:
    """Compile .bas/.maxx to .532; return output path."""
    source = source.resolve()
    if not source.is_file():
        raise FileNotFoundError(source)

    out = output or source.with_suffix(".532")
    run_maxx(
        [
            "compile",
            str(source),
            "-o",
            str(out),
            "--copyright",
            copyright,
        ]
    )
    return out


def resolve_rom_input(
    path: Path,
    *,
    copyright: str = "ultramaxx",
    output: Path | None = None,
) -> tuple[Path, tempfile.NamedTemporaryFile[bytes] | None]:
    """Return a .532 path, compiling MaxxBAS sources on demand."""
    path = path if path.is_absolute() else project_root() / path
    if not path.exists():
        raise FileNotFoundError(path)

    if is_maxxbas_source(path):
        if output is not None:
            out = compile_source(path, output, copyright=copyright)
            return out, None
        tmp = tempfile.NamedTemporaryFile(suffix=".532", delete=False)
        tmp.close()
        out = compile_source(path, Path(tmp.name), copyright=copyright)
        return out, tmp

    return path, None