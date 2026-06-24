#!/usr/bin/env python3
"""Generate CBSDemo cartridge KiCad schematics (legacy + KiCad 10 native).

Reverse-engineered from Cartridge/Photos/maxxcard.jpg.  Provisional net
assignments are documented in trace-worksheet.md.
"""

from __future__ import annotations

import hashlib
import json
import uuid
from pathlib import Path

from kicad_sym_extract import extract_symbol_block

ROOT = Path(__file__).resolve().parents[1]
OUT_DIR = ROOT / "Cartridge/Examples/CBSDemo/KiCAD"
OUT_LEGACY = OUT_DIR / "CBSDemo.sch"
OUT_KICAD_SCH = OUT_DIR / "CBSDemo.kicad_sch"
OUT_KICAD_PRO = OUT_DIR / "CBSDemo.kicad_pro"
PROJECT = "CBSDemo"
ROOT_UUID = str(uuid.uuid5(uuid.NAMESPACE_DNS, "maxx-steele/CBSDemo/cartridge"))


def comp_uuid(seed: str) -> str:
    return hashlib.md5(seed.encode()).hexdigest()[:8].upper()


def uid(seed: str) -> str:
    return str(uuid.uuid5(uuid.NAMESPACE_DNS, f"maxx-steele/CBSDemo/{seed}"))


def mm(legacy_units: int) -> float:
    """Legacy EESchema v4 coords are in 0.01 mm."""
    return legacy_units / 100.0


def comp(
    lib: str,
    ref: str,
    value: str,
    x: int,
    y: int,
    rot: int = 0,
    footprint: str = "",
    datasheet: str = "~",
    field_orient: str = "C",
) -> str:
    lines = [
        "$Comp",
        f"L {lib} {ref}",
        f"U 1 1 {comp_uuid(f'{ref}:{x}:{y}')}",
        f"P {x} {y}",
        f'F 0 "{ref}" H {x} {y - 150} 50  0000 {field_orient} CNN',
        f'F 1 "{value}" H {x} {y + 150} 50  0000 {field_orient} CNN',
        f'F 2 "{footprint or "~"}" H {x} {y} 50  0001 C CNN',
        f'F 3 "{datasheet}" H {x} {y} 50  0001 C CNN',
        f"\t1    {x} {y}",
    ]
    mirror = "0" if rot in (0, 180) else "1"
    lines.append(f"\t{mirror}    {rot % 360}    0    -1  ")
    lines.append("$EndComp")
    return "\n".join(lines)


def pwr(ref: str, value: str, x: int, y: int) -> str:
    return "\n".join(
        [
            "$Comp",
            f"L power:{value} {ref}",
            f"U 1 1 {comp_uuid(f'pwr:{ref}:{x}:{y}')}",
            f"P {x} {y}",
            f'F 0 "{ref}" H {x} {y - 250} 50  0001 C CNN',
            f'F 1 "{value}" H {x + 5} {y - 173} 50  0000 C CNN',
            f'F 2 "" H {x} {y} 50  0001 C CNN',
            f'F 3 "" H {x} {y} 50  0001 C CNN',
            f"\t1    {x} {y}",
            "\t1    0    0    -1  ",
            "$EndComp",
        ]
    )


def cap(ref: str, x: int, y: int) -> str:
    return "\n".join(
        [
            "$Comp",
            f"L Device:C_Small {ref}",
            f"U 1 1 {comp_uuid(f'cap:{ref}')}",
            f"P {x} {y}",
            f'F 0 "{ref}" H {x + 92} {y + 46} 50  0000 L CNN',
            f'F 1 "0.1uF" H {x + 92} {y - 45} 50  0000 L CNN',
            f'F 2 "Capacitor_THT:C_Disc_D4.3mm_W1.9mm_P5.00mm" H {x} {y} 50  0001 C CNN',
            f'F 3 "~" H {x} {y} 50  0001 C CNN',
            f"\t1    {x} {y}",
            "\t1    0    0    -1  ",
            "$EndComp",
        ]
    )


def wire(x1: int, y1: int, x2: int, y2: int) -> str:
    return f"Wire Wire Line\n\t{x1} {y1} {x2} {y2}"


def label(text: str, x: int, y: int, orient: int = 0) -> str:
    return f"Text Label {x} {y} {orient}    50   ~ 0\n{text}"


def design() -> dict:
    """Shared component and connectivity layout (legacy 0.01 mm units)."""
    j1_x, j1_y = 1200, 3600
    u1_x, u1_y = 3600, 2200
    u3_x, u3_y = 5600, 2200
    u2_x, u2_y = 7600, 2200
    rail_y = 1400
    gnd_y = 3200
    bus_x_j = 2000
    bus_x_u1 = 3200
    bus_x_u3 = 5400
    bus_x_u2 = 7400

    components = [
        ("Connector:Conn_01x44_Pin", "J1", "MaxxCard_Edge_44x2.54mm", j1_x, j1_y, "Connector:Conn_01x44_Pin",
         "~", 44),
        ("Memory_EPROM:27C512", "U1", "27C512", u1_x, u1_y, "Package_DIP:DIP-28_W15.24mm",
         "${KIPRJMOD}/../../../../DataSheets/Mitsubishi-KM2365.pdf", 28),
        ("Connector:Conn_01x24_Pin", "U3", "5085-TBD", u3_x, u3_y, "Package_DIP:DIP-24_W15.24mm", "~", 24),
        ("74xx:74HC14", "U2", "74HC14?", u2_x, u2_y, "Package_DIP:DIP-14_W7.62mm", "~", 14),
    ]
    for ref, x, y in [("C1", 3300, 1700), ("C2", 5300, 1700), ("C3", 7300, 1700)]:
        components.append(("Device:C_Small", ref, "0.1uF", x, y,
                           "Capacitor_THT:C_Disc_D4.3mm_W1.9mm_P5.00mm", "~", 2))
    components.append(("power:GND", "#PWR01", "GND", 1200, 5200, "", "", 1))
    components.append(("power:+5V", "#PWR02", "+5V", 1600, 5200, "", "", 1))

    wires = [
        (1600, 5200, 1600, rail_y),
        (1600, rail_y, 8200, rail_y),
        (1200, 5200, 1200, gnd_y),
        (1200, gnd_y, 8200, gnd_y),
    ]
    for cx in (3300, 5300, 7300):
        wires.extend([(cx, rail_y, cx, 1700), (cx, 1900, cx, gnd_y)])

    labels: list[tuple[str, int, int]] = []
    for i in range(16):
        y = 4000 - i * 100
        wires.extend([(bus_x_j, y, bus_x_u1, y), (bus_x_u1, y, bus_x_u3, y)])
        labels.append((f"A{i}", bus_x_j + 50, y))
    for i in range(8):
        y = 5600 + i * 100
        wires.append((bus_x_j, y, bus_x_u1, y))
        labels.append((f"D{i}", bus_x_j + 50, y))
    for net, y in [("/CE", 2400), ("/OE", 2500), ("PHI2", 5450), ("R/W", 5550)]:
        wires.extend([(bus_x_j, y, bus_x_u2, y), (bus_x_u2, y, bus_x_u1, y)])
        labels.append((net, bus_x_j + 50, y))

    return {"components": components, "wires": wires, "labels": labels}


def build_legacy(d: dict) -> str:
    parts = []
    for lib_id, ref, value, x, y, fp, ds, _pins in d["components"]:
        if lib_id.startswith("power:"):
            parts.append(pwr(ref, value, x, y))
        elif lib_id == "Device:C_Small":
            parts.append(cap(ref, x, y))
        else:
            parts.append(comp(lib_id, ref, value, x, y, footprint=fp, datasheet=ds))

    body = "\n".join(
        parts
        + [wire(*w) for w in d["wires"]]
        + [label(text, x, y) for text, x, y in d["labels"]]
    )
    header = "\n".join(
        [
            "EESchema Schematic File Version 4",
            "EELAYER 30 0",
            "EELAYER END",
            "$Descr A3 16535 11693",
            "encoding utf-8",
            "Sheet 1 1",
            "Title \"Maxx Steele CBSDemo Cartridge\"",
            "Date \"2026-06-24\"",
            "Rev \"0.1\"",
            "Comp \"Maxx-Steele-1984-Robot\"",
            "Comment1 \"Reverse-engineered from maxxcard.jpg\"",
            "Comment2 \"Provisional netlist - see KiCAD/trace-worksheet.md\"",
            "Comment3 \"\"",
            "Comment4 \"\"",
            "$EndDescr",
        ]
    )
    return header + "\n" + body + "\n$EndSCHEMATC\n"


def kicad_symbol_instance(
    lib_id: str,
    ref: str,
    value: str,
    x: int,
    y: int,
    footprint: str,
    datasheet: str,
    pin_count: int,
) -> list[str]:
    sym_uid = uid(f"sym/{ref}")
    lines = [
        "\t(symbol",
        f'\t\t(lib_id "{lib_id}")',
        f"\t\t(at {mm(x)} {mm(y)} 0)",
        "\t\t(unit 1)",
        "\t\t(exclude_from_sim no)",
        "\t\t(in_bom yes)",
        "\t\t(on_board yes)",
        "\t\t(dnp no)",
        f'\t\t(uuid "{sym_uid}")',
        f'\t\t(property "Reference" "{ref}"',
        f"\t\t\t(at {mm(x)} {mm(y - 150)} 0)",
        "\t\t\t(effects (font (size 1.27 1.27)))",
        "\t\t)",
        f'\t\t(property "Value" "{value}"',
        f"\t\t\t(at {mm(x)} {mm(y + 150)} 0)",
        "\t\t\t(effects (font (size 1.27 1.27)))",
        "\t\t)",
        f'\t\t(property "Footprint" "{footprint}"',
        f"\t\t\t(at {mm(x)} {mm(y)} 0)",
        "\t\t\t(effects (font (size 1.27 1.27)) (hide yes))",
        "\t\t)",
        f'\t\t(property "Datasheet" "{datasheet}"',
        f"\t\t\t(at {mm(x)} {mm(y)} 0)",
        "\t\t\t(effects (font (size 1.27 1.27)) (hide yes))",
        "\t\t)",
    ]
    for pin in range(1, pin_count + 1):
        lines.append(f'\t\t(pin "{pin}"')
        lines.append(f'\t\t\t(uuid "{uid(f"pin/{ref}/{pin}")}")')
        lines.append("\t\t)")
    lines.extend(
        [
            "\t\t(instances",
            f'\t\t\t(project "{PROJECT}"',
            f'\t\t\t\t(path "/{ROOT_UUID}"',
            f'\t\t\t\t\t(reference "{ref}")',
            "\t\t\t\t\t(unit 1)",
            "\t\t\t\t)",
            "\t\t\t)",
            "\t\t)",
            "\t)",
        ]
    )
    return lines


def build_kicad_sch(d: dict) -> str:
    lib_ids = sorted({c[0] for c in d["components"]})
    lib_blocks = []
    for lib_id in lib_ids:
        block = extract_symbol_block(lib_id)
        lib_blocks.append("\t" + block.replace("\n", "\n\t"))

    lines = [
        "(kicad_sch",
        "\t(version 20250114)",
        '\t(generator "maxx-steele-tools")',
        '\t(generator_version "1.1")',
        f'\t(uuid "{ROOT_UUID}")',
        '\t(paper "A3")',
        "\t(title_block",
        '\t\t(title "Maxx Steele CBSDemo Cartridge")',
        '\t\t(date "2026-06-24")',
        '\t\t(rev "0.1")',
        '\t\t(comment 1 "Reverse-engineered from maxxcard.jpg")',
        '\t\t(comment 2 "Provisional netlist - see KiCAD/trace-worksheet.md")',
        "\t)",
        "\t(lib_symbols",
        *lib_blocks,
        "\t)",
    ]

    for lib_id, ref, value, x, y, fp, ds, pin_count in d["components"]:
        lines.extend(kicad_symbol_instance(lib_id, ref, value, x, y, fp, ds, pin_count))

    for x1, y1, x2, y2 in d["wires"]:
        lines.extend(
            [
                "\t(wire",
                "\t\t(pts",
                f"\t\t\t(xy {mm(x1)} {mm(y1)})",
                f"\t\t\t(xy {mm(x2)} {mm(y2)})",
                "\t\t)",
                "\t\t(stroke (width 0) (type default))",
                f'\t\t(uuid "{uid(f"wire/{x1}/{y1}/{x2}/{y2}")}")',
                "\t)",
            ]
        )

    for text, x, y in d["labels"]:
        lines.extend(
            [
                "\t(label",
                f'\t\t"{text}"',
                f"\t\t(at {mm(x)} {mm(y)} 0)",
                "\t\t(effects (font (size 1.27 1.27)) (justify left bottom))",
                f'\t\t(uuid "{uid(f"label/{text}/{x}/{y}")}")',
                "\t)",
            ]
        )

    lines.extend(
        [
            "\t(sheet_instances",
            f'\t\t(path "/{ROOT_UUID}"',
            '\t\t\t(page "1")',
            "\t\t)",
            "\t)",
            "\t(embedded_fonts no)",
            ")",
        ]
    )
    return "\n".join(lines) + "\n"


def build_kicad_pro() -> str:
    template = json.loads(
        (Path("/usr/share/kicad/template/KiCad_MR_diagrams_small_parts/mr_diagrams_small_parts.kicad_pro").read_text())
    )
    template["sheets"] = [[ROOT_UUID, "Root"]]
    return json.dumps(template, indent=2) + "\n"


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    layout = design()
    OUT_LEGACY.write_text(build_legacy(layout))
    OUT_KICAD_SCH.write_text(build_kicad_sch(layout))
    OUT_KICAD_PRO.write_text(build_kicad_pro())
    pdf = OUT_DIR / "CBSDemo-schematic.pdf"
    import subprocess

    subprocess.run(
        ["kicad-cli", "sch", "export", "pdf", str(OUT_KICAD_SCH), "-o", str(pdf)],
        check=True,
        capture_output=True,
    )
    print(f"Wrote {OUT_LEGACY}")
    print(f"Wrote {OUT_KICAD_SCH}")
    print(f"Wrote {OUT_KICAD_PRO}")
    print(f"Wrote {pdf}")


if __name__ == "__main__":
    main()