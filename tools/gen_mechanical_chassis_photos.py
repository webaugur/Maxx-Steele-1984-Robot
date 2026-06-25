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
        "Cyberia Makerspace — additional angles",
        "Workshop session variants and timestamps. Canonical `IMG_*` frames are in the teardown sequence above.",
        CHASSIS / "Photos" / "cyberia-makerspace",
    ),
    (
        "Internal autopsy",
        "Opened chassis: mainboard, base, arm balance spring, and backoff views.",
        CHASSIS / "Photos" / "autopsy",
    ),
    (
        "Exterior and collection",
        "Mint/vintage toy listing photos: exterior, remote, and detail shots.",
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

SKIP_NAMES = frozenset(
    name
    for name in (
        [f"Copy-of-IMG_{n}.JPG" for n in range(2116, 2132)]
        + [f"IMG_{n}.JPG" for n in range(2116, 2132)]
    )
)


def rel_chassis(path: Path) -> str:
    return path.relative_to(CHASSIS).as_posix()


def link(path: Path) -> str:
    return f"../Chassis/{rel_chassis(path)}"


def is_raster(path: Path) -> bool:
    return path.suffix in RASTER_EXT


def is_vector(path: Path) -> bool:
    return path.suffix in VECTOR_EXT


def list_images(folder: Path, *, cyberia: bool) -> list[Path]:
    if not folder.is_dir():
        return []
    files = sorted(
        p
        for p in folder.iterdir()
        if p.is_file() and (is_raster(p) or is_vector(p))
    )
    if cyberia:
        files = [p for p in files if p.name not in SKIP_NAMES]
    return files


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
        "Duplicate `Copy-of-IMG_*` exports and second copies of the canonical `IMG_2116`–`IMG_2131` "
        "frames in `cyberia-makerspace/` are omitted here (they match "
        "[`Chassis/Photos/Disassembly/`](../Chassis/Photos/Disassembly/)).",
        "",
    ]

    for title, blurb, folder in SECTIONS:
        cyberia = folder.name == "cyberia-makerspace"
        images = list_images(folder, cyberia=cyberia)
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
    total = sum(
        len(list_images(folder, cyberia=folder.name == "cyberia-makerspace"))
        for _, _, folder in SECTIONS
    )
    print(f"wrote {OUT.relative_to(REPO)} ({total} images)")


if __name__ == "__main__":
    main()