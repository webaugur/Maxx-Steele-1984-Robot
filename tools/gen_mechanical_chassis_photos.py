#!/usr/bin/env python3
"""Generate MechanicalManual/03-Chassis-Photos.md from Chassis/ image inventory."""

from __future__ import annotations

from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
CHASSIS = REPO / "Chassis"
OUT = REPO / "MechanicalManual" / "03-Chassis-Photos.md"

RASTER_EXT = {".jpg", ".jpeg", ".png", ".gif", ".webp", ".JPG", ".JPEG", ".PNG"}
VECTOR_EXT = {".svg", ".SVG"}

SECTIONS: tuple[tuple[str, str, Path], ...] = (
    (
        "Workshop teardown sequence",
        "Primary disassembly walkthrough (`IMG_2116`–`IMG_2131`).",
        CHASSIS / "Photos" / "Disassembly",
    ),
    (
        "Cyberia Makerspace — workshop session",
        "Timestamp photos and hero shot from the teardown session. Canonical `IMG_2116`–`IMG_2131` frames are in Disassembly above.",
        CHASSIS / "Photos" / "cyberia-makerspace",
    ),
    (
        "Internal autopsy",
        "Opened chassis: mainboard, base, arm balance spring, and backoff views.",
        CHASSIS / "Photos" / "autopsy",
    ),
    (
        "Exterior and collection",
        "Mint/vintage toy listing photos: exterior and detail shots. Remote photos: [`Transmitter/Photos/Product/`](../../Transmitter/Photos/Product/).",
        CHASSIS / "Photos" / "mint-vintage-toys",
    ),
    (
        "Case and packaging",
        "Retail case, press, and promotional photos.",
        CHASSIS / "Photos" / "case",
    ),
    (
        "Body artwork",
        "Vector logos and body graphics (SVG source files; raster booklet shows filenames).",
        CHASSIS / "Artwork",
    ),
)

def rel_chassis(path: Path) -> str:
    return path.relative_to(CHASSIS).as_posix()


def link(path: Path) -> str:
    return f"../Chassis/{rel_chassis(path)}"


def is_raster(path: Path) -> bool:
    return path.suffix in RASTER_EXT


def is_vector(path: Path) -> bool:
    return path.suffix in VECTOR_EXT


def list_images(folder: Path) -> list[Path]:
    if not folder.is_dir():
        return []
    return sorted(
        p
        for p in folder.iterdir()
        if p.is_file() and (is_raster(p) or is_vector(p))
    )


def emit_image(path: Path) -> list[str]:
    href = link(path)
    name = path.name
    if is_raster(path):
        return [f"![{name}]({href})", ""]
    return [
        f"- [`{name}`]({href}) — vector artwork (open in repo for full SVG)",
    ]


def main() -> None:
    lines = [
        "# Chapter 3 — Chassis photo reference",
        "",
        "All photos and artwork under [`Chassis/`](../Chassis/), grouped by source folder.",
        "",
    ]

    for title, blurb, folder in SECTIONS:
        images = list_images(folder)
        lines.append(f"## {title}")
        lines.append("")
        lines.append(f"{blurb} Archive: [`Chassis/{rel_chassis(folder)}/`](../Chassis/{rel_chassis(folder)}/).")
        lines.append("")
        if not images:
            lines.append("_No images in this folder._")
            lines.append("")
            continue
        for path in images:
            lines.extend(emit_image(path))
        lines.append("")

    lines.append("---")
    lines.append("")
    lines.append("**Previous:** [Chapter 2 — Reassembly](02-Reassembly.md)")
    lines.append("")

    OUT.write_text("\n".join(lines), encoding="utf-8")
    total = sum(len(list_images(folder)) for _, _, folder in SECTIONS)
    print(f"wrote {OUT.relative_to(REPO)} ({total} images)")


if __name__ == "__main__":
    main()