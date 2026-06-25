#!/usr/bin/env python3
"""Generate MechanicalManual/Hardware-Assets.csv — hardware media inventory."""

from __future__ import annotations

import csv
import re
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
OUT = REPO / "MechanicalManual" / "Hardware-Assets.csv"

RASTER_EXT = {".jpg", ".jpeg", ".png", ".gif", ".webp", ".JPG", ".JPEG", ".PNG"}
VECTOR_EXT = {".svg", ".SVG"}
MEDIA_EXT = RASTER_EXT | VECTOR_EXT | {".wma", ".pdf", ".64", ".532", ".dsm"}

# (section title, section blurb) keyed by Chassis/Photos/<folder> or Chassis/Artwork
PHOTO_SECTIONS: dict[str, tuple[str, str]] = {
    "Photos/Disassembly": (
        "Workshop teardown sequence",
        "Primary disassembly walkthrough IMG_2116 through IMG_2131",
    ),
    "Photos/cyberia-makerspace": (
        "Cyberia Makerspace workshop session",
        "Timestamp photos and hero shot from teardown session",
    ),
    "Photos/autopsy": (
        "Internal autopsy",
        "Opened chassis mainboard base arm spring and backoff views",
    ),
    "Photos/mint-vintage-toys": (
        "Exterior and collection",
        "Mint vintage toy listing exterior and detail shots",
    ),
    "Photos/case": (
        "Case and packaging",
        "Retail case press and promotional photos",
    ),
    "Artwork": (
        "Body artwork",
        "Vector logos and body graphics",
    ),
}

AUTOPSY_HINTS: dict[str, str] = {
    "Arm-Balance-Spring.jpg": "Arm balance spring detail",
    "Base-and-battery-mount.jpg": "Base shell and battery mount",
    "Main-Board.jpg": "Internal mainboard top view",
    "Main-Board-1.jpg": "Internal mainboard alternate angle",
    "PIC00024.jpg": "Internal chassis component detail",
    "back-off-arms-off.jpg": "Rear view arms removed",
    "backoff.jpg": "Rear backoff shell view",
}

MINT_HINTS: dict[str, str] = {
    "maxx_bottom.jpg": "Robot underside exterior",
    "maxx_in_packing.jpg": "Robot in original packing foam",
    "maxxsteal7.jpg": "Full robot exterior collection shot",
}

TUNE_DESCRIPTIONS: dict[int, str] = {
    1: "PLAY 1 opcode $81 01 Immediate mode tune internal ROM tune table tune 1",
    2: "PLAY 2 opcode $81 02 Learn mode tune internal ROM tune table tune 2",
    3: "PLAY 3 opcode $81 03 Program mode tune internal ROM tune table tune 3",
    4: "PLAY 4 opcode $81 04 Execute mode tune internal ROM tune table tune 4",
    5: "PLAY 5 opcode $81 05 Game mode tune internal ROM tune table tune 5",
    6: "PLAY 6 opcode $81 06 Reveille internal ROM tune table tune 6 factory demo",
    7: "PLAY 7 opcode $81 07 Power-down tune internal ROM tune table tune 7",
    8: "PLAY 8 opcode $81 08 Taps internal ROM tune table tune 8",
}

SCAN_ROOTS = (
    "Chassis",
    "Transmitter",
    "Receiver",
    "Mainboard",
    "Cartridge",
    "PaddleMirror",
)

SKIP_REL = {
    "Cartridge/Model3D/Pasted image.png",
    "Chassis/Firmware/Assembly/maxx_internal_ROM.dsm.pdf",
    "Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm.pdf",
}


def rel(path: Path) -> str:
    return path.relative_to(REPO).as_posix()


def no_commas(text: str) -> str:
    return text.replace(",", " ").replace("  ", " ").strip()


def teardown_step(name: str) -> str | None:
    m = re.match(r"IMG_(\d{4})(?:-1)?\.(?:jpg|JPG)$", name)
    if not m:
        return None
    num = int(m.group(1))
    if 2116 <= num <= 2131:
        step = num - 2115
        return f"Teardown frame {num} step {step} of 16"
    return None


def chassis_photo_description(path: Path) -> str:
    parts = path.relative_to(REPO / "Chassis")
    key = "/".join(parts.parts[:-1])
    name = path.name
    section, blurb = PHOTO_SECTIONS.get(key, ("Chassis photo", "Chassis reference image"))

    if key == "Photos/autopsy" and name in AUTOPSY_HINTS:
        return no_commas(f"{AUTOPSY_HINTS[name]} — {blurb}")

    if key == "Photos/mint-vintage-toys" and name in MINT_HINTS:
        return no_commas(f"{MINT_HINTS[name]} — {blurb}")

    if key == "Artwork":
        stem = path.stem.replace("Maxx-Steele-", "").replace("-", " ")
        return no_commas(f"{stem} SVG body artwork")

    step = teardown_step(name)
    if step and key == "Photos/Disassembly":
        return no_commas(f"{step} — {blurb}")

    if re.match(r"2012-", name):
        return no_commas(f"Cyberia workshop timestamp photo — {blurb}")

    if name == "Cyberia-Maxx-Steele.jpg":
        return no_commas(f"Cyberia Makerspace hero photo — {blurb}")

    return no_commas(f"{section} — {name}")


def assign_manual(path: Path) -> str:
    r = rel(path)

    if r.startswith("Chassis/Sounds/"):
        return "TechnicalManual"

    if r.startswith("Chassis/Firmware/"):
        return "TechnicalManual"

    if r.startswith("Chassis/References/"):
        return "MechanicalManual|TechnicalManual"

    if r.startswith("Chassis/Manual/"):
        return "MechanicalManual|TechnicalManual"

    if r.startswith("Chassis/Photos/autopsy/"):
        if "Main-Board" in path.name or "Board" in path.name:
            return "MechanicalManual|TechnicalManual"
        return "MechanicalManual"

    if r.startswith("Chassis/"):
        return "MechanicalManual"

    if r.startswith("Mainboard/Schematic/"):
        return "TechnicalManual"

    if r.startswith("Transmitter/Photos/ReverseEngineering/"):
        return "TechnicalManual"

    if r.startswith("Transmitter/Photos/Product/"):
        return "MechanicalManual|TechnicalManual"

    if r.startswith("Transmitter/ReverseEngineering/"):
        return "TechnicalManual"

    if r.startswith("Transmitter/Stickers/"):
        return "MechanicalManual"

    if r.startswith("Transmitter/KiCAD/") and path.suffix == ".pdf":
        return "TechnicalManual"

    if r.startswith("Receiver/KiCAD/") and path.suffix == ".pdf":
        return "TechnicalManual"

    if r.startswith("Cartridge/Photos/"):
        return "MechanicalManual|TechnicalManual"

    if r.startswith("Cartridge/Examples/") and (
        path.suffix in {".532", ".dsm"} or "KiCAD/reference" in r or path.name.endswith("-schematic.pdf")
    ):
        return "TechnicalManual"

    if r.startswith("PaddleMirror/"):
        return "MechanicalManual"

    return "MechanicalManual|TechnicalManual"


def describe(path: Path) -> str:
    r = rel(path)
    name = path.name

    if r.startswith("Chassis/Sounds/maxx-song-"):
        n = int(name.replace("maxx-song-", "").replace(".wma", ""))
        return TUNE_DESCRIPTIONS[n]

    if r == "Chassis/Sounds/maxx-greet-5.wma":
        return (
            "Speech greeting sample opcode mapping unverified "
            "(not PLAY 5 see ROM phrase table $10-$20 in maxx_internal_ROM.dsm)"
        )

    if r == "Chassis/Firmware/Binary/Maxxrom.64":
        return "Internal 8 KB 6502 ROM binary image ($0000-$1FFF)"

    if r.endswith("maxx_internal_ROM.dsm"):
        return "Annotated internal ROM disassembly primary Technical Manual source"

    if r.endswith("maxx_demo_ROM_532.dsm"):
        return "Annotated factory CBS demo cartridge ROM disassembly"

    if r.endswith("ultramaxx_ROM_532.dsm"):
        return "Annotated UltraMaxx cartridge ROM disassembly (CBSDemo fork)"

    if name.endswith(".532"):
        stem = path.stem
        return f"4 KB cartridge EPROM image at $A000 ({stem})"

    if r == "Chassis/Manual/MaxxSteeleManual.pdf":
        return "Factory owner manual operation and keypad (not ROM layout)"

    if r == "Chassis/Manual/MaxxSteeleReferenceGuide.pdf":
        return "Factory reference guide modes speech and programming overview"

    if r.startswith("Chassis/References/"):
        stem = path.stem.replace("-", " ")
        return f"Third-party workshop or article PDF — {stem}"

    if r.startswith("Chassis/Photos/") or r.startswith("Chassis/Artwork/"):
        return chassis_photo_description(path)

    if r == "Mainboard/Schematic/Maxx_Steele_Schematic.jpg":
        return "Mainboard raster schematic scan original"

    if r == "Mainboard/Schematic/Maxx_Steele_Schematic_enh-v1.1.png":
        return "Mainboard raster schematic enhanced v1.1"

    if r == "Mainboard/Schematic/v1.1/Maxx_Steele_Schematic.svg":
        return "Mainboard vector schematic SVG v1.1"

    if r == "Transmitter/Photos/Product/Remote-Front.jpg":
        return "Remote transmitter front product photo keypad labels"

    if r == "Transmitter/Photos/Product/Remote-Front.svg":
        return "Remote front keypad diagram with labeled keys"

    if r == "Transmitter/Photos/Product/Remote-Back.jpg":
        return "Remote transmitter rear product photo"

    if r == "Transmitter/Photos/Product/Maxx-Transmitter-Front.jpg":
        return "Transmitter product hero front angle"

    if r == "Transmitter/Photos/Product/Remote-Sticker-FCC.jpg":
        return "FCC ID and regulatory sticker on remote rear"

    if r == "Transmitter/Photos/ReverseEngineering/keyboard-matrix-reference-1.png":
        return "Keypad matrix wiring reference diagram 1"

    if r == "Transmitter/Photos/ReverseEngineering/keyboard-matrix-reference-2.png":
        return "Keypad matrix wiring reference diagram 2"

    if r == "Transmitter/Photos/ReverseEngineering/handwritten-bom.jpg":
        return "Handwritten BOM notes from transmitter PCB"

    if r == "Transmitter/Photos/ReverseEngineering/mcu-board-pcb.jpg":
        return "Transmitter MCU board PCB photo"

    if "trace-overlay" in name or "schematic-trace" in name:
        return f"Transmitter PCB trace overlay — {name}"

    if r == "Transmitter/ReverseEngineering/Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf":
        return "Transmitter reverse engineering notes COP keypad and RF"

    if r.startswith("Transmitter/Stickers/"):
        return f"Remote case sticker artwork — {path.stem.replace('-', ' ')}"

    if r.endswith("Transmitter-27MHz-schematic.pdf"):
        return "Transmitter 27 MHz KiCad schematic export"

    if r.endswith("Receiver-27MHz-schematic.pdf"):
        return "Receiver 27 MHz KiCad schematic export"

    if r == "Cartridge/Photos/maxxcard.jpg":
        return "Demo cartridge PCB photo 22 fingers per face 44 contacts total"

    if "KiCAD/reference" in r:
        return f"Cartridge PCB reference crop for KiCad RE — {name}"

    if r.endswith("CBSDemo-schematic.pdf"):
        return "CBSDemo cartridge KiCad schematic export"

    if r == "PaddleMirror/Photos/ab226.jpg":
        return "Paddle mirror accessory product photo"

    return no_commas(f"Hardware asset — {r}")


def iter_assets() -> list[Path]:
    found: list[Path] = []
    for root_name in SCAN_ROOTS:
        root = REPO / root_name
        if not root.is_dir():
            continue
        for path in sorted(root.rglob("*")):
            if not path.is_file():
                continue
            if path.suffix not in MEDIA_EXT:
                continue
            r = rel(path)
            if r in SKIP_REL:
                continue
            found.append(path)
    return found


def main() -> None:
    rows: list[tuple[str, str, str]] = []
    for path in iter_assets():
        rows.append((rel(path), assign_manual(path), describe(path)))

    rows.sort(key=lambda r: r[0].lower())

    with OUT.open("w", newline="", encoding="utf-8") as f:
        w = csv.writer(f, lineterminator="\n")
        w.writerow(["filename", "MechanicalManual|TechnicalManual", "description"])
        w.writerows(rows)

    print(f"wrote {OUT.relative_to(REPO)} ({len(rows)} assets)")


if __name__ == "__main__":
    main()