"""Canonical paths for community manuals under Docs/."""

from __future__ import annotations

from pathlib import Path

from project_paths import project_root

DOCS = Path("Docs")
USER_MANUAL = DOCS / "User"
TECHNICAL_MANUAL = DOCS / "Technical"
MECHANICAL_MANUAL = DOCS / "Mechanical"

MANUAL_DIRS = (USER_MANUAL, TECHNICAL_MANUAL, MECHANICAL_MANUAL)


def manual_dir(name: str) -> Path:
    """Return a manual directory under the repo root."""
    mapping = {
        "user": USER_MANUAL,
        "technical": TECHNICAL_MANUAL,
        "mechanical": MECHANICAL_MANUAL,
    }
    try:
        return project_root() / mapping[name.lower()]
    except KeyError as exc:
        raise ValueError(f"unknown manual: {name!r}") from exc