#!/usr/bin/env python3
"""Build and freshness checks for the Rust maxx toolchain."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

from process_util import INTERRUPTED
from project_paths import project_root

CRATE_DIR = project_root() / "tools" / "maxxbas"
RELEASE_BIN = CRATE_DIR / "target" / "release" / "maxx"
DEBUG_BIN = CRATE_DIR / "target" / "debug" / "maxx"
MANIFEST = CRATE_DIR / "Cargo.toml"
LOCKFILE = CRATE_DIR / "Cargo.lock"
SOURCE_SUFFIXES = {".rs", ".toml"}


def find_maxx_binary() -> Path | None:
    for candidate in (RELEASE_BIN, DEBUG_BIN):
        if candidate.is_file():
            return candidate
    return None


def binary_version(path: Path) -> str:
    proc = subprocess.run(
        [str(path), "--version"],
        capture_output=True,
        text=True,
        check=False,
    )
    return (proc.stdout or proc.stderr or "").strip()


def crate_source_mtime() -> float:
    newest = 0.0
    for path in CRATE_DIR.rglob("*"):
        if path.is_file() and path.suffix in SOURCE_SUFFIXES:
            newest = max(newest, path.stat().st_mtime)
    for extra in (MANIFEST, LOCKFILE):
        if extra.is_file():
            newest = max(newest, extra.stat().st_mtime)
    return newest


def needs_rebuild() -> bool:
    if not RELEASE_BIN.is_file():
        return True
    return crate_source_mtime() > RELEASE_BIN.stat().st_mtime


def build_maxx(*, release: bool = True) -> Path:
    profile = "release" if release else "debug"
    cmd = [
        "cargo",
        "build",
        f"--{profile}",
        "--manifest-path",
        str(MANIFEST),
    ]
    print("building maxx toolchain...", file=sys.stderr)
    try:
        subprocess.run(cmd, check=True)
    except KeyboardInterrupt:
        print("\nBuild interrupted.", file=sys.stderr)
        raise SystemExit(INTERRUPTED) from None
    path = RELEASE_BIN if release else DEBUG_BIN
    if not path.is_file():
        raise RuntimeError(f"build succeeded but {path} not found")
    return path


def ensure_maxx_built(*, force: bool = False) -> Path:
    """Return a release binary, rebuilding when sources are newer (or when forced)."""
    if force or needs_rebuild():
        return build_maxx(release=True)
    found = find_maxx_binary()
    if found is not None:
        return found
    return build_maxx(release=True)


def verify_maxx_built(*, force: bool = False) -> tuple[Path, str]:
    """Ensure the release binary is current; return (path, version string)."""
    binary = ensure_maxx_built(force=force)
    if needs_rebuild():
        raise RuntimeError(
            f"maxx binary is still stale after build: {binary}\n"
            "source mtime exceeds binary mtime"
        )
    version = binary_version(binary)
    if not version:
        raise RuntimeError(f"maxx binary produced no version: {binary}")
    return binary, version