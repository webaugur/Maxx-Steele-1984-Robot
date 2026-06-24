"""Resolve paths relative to the Maxx Steele repository root."""

from __future__ import annotations

import os
from pathlib import Path

_MARKER_DIRS = ("docs", "tools", "Chassis")


def project_root(start: Path | str | None = None) -> Path:
    """Return the repository root directory."""
    path = Path(start or __file__).resolve()
    if path.is_file():
        path = path.parent

    for directory in (path, *path.parents):
        if (directory / ".git").is_dir():
            return directory
        if all((directory / name).is_dir() for name in _MARKER_DIRS):
            return directory

    raise RuntimeError("Could not locate Maxx Steele project root")


def resolve_from_root(path: Path | str, *, must_exist: bool = False) -> Path:
    """Expand a path from project root when it is not already absolute."""
    candidate = Path(path)
    if candidate.is_absolute():
        resolved = candidate
    else:
        resolved = project_root() / candidate
    if must_exist and not resolved.exists():
        raise FileNotFoundError(resolved)
    return resolved


def as_posix(path: Path | str) -> str:
    """Return a forward-slash path string suitable for GNU Radio variables."""
    return Path(path).as_posix()


def ensure_dir(path: Path | str) -> Path:
    """Create a directory (including parents) and return its path."""
    directory = Path(path)
    directory.mkdir(parents=True, exist_ok=True)
    return directory


def capture_dir() -> Path:
    """GNU Radio IQ capture output directory."""
    return ensure_dir(project_root() / "tools" / "rfcap" / "captures")


def capture_prefix() -> str:
    """Prefix for new capture files, with trailing separator."""
    return as_posix(capture_dir()) + "/"


def resolve_capture_path(path: Path | str) -> str:
    """Resolve a capture file path for read/write (absolute or project-relative)."""
    candidate = Path(path)
    if candidate.is_absolute():
        return as_posix(candidate)
    if candidate.parts[:2] == ("tools", "rfcap"):
        return as_posix(resolve_from_root(candidate))
    if candidate.parts and candidate.parts[0] == "rfcap":
        return as_posix(resolve_from_root(Path("tools") / candidate))
    return as_posix(resolve_from_root(Path("tools/rfcap/captures") / candidate))