#!/usr/bin/env python3
"""Generate CBSDemo cartridge KiCad schematic (legacy EESchema v4).

Reverse-engineered from Cartridge/Photos/maxxcard.jpg.  Provisional net
assignments are documented in trace-worksheet.md.
"""

from __future__ import annotations

import hashlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT_DIR = ROOT / "Cartridge/Examples/CBSDemo/KiCAD"
OUT_SCH = OUT_DIR / "CBSDemo.sch"


def comp_uuid(seed: str) -> str:
    return hashlib.md5(seed.encode()).hexdigest()[:8].upper()


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
    uid = comp_uuid(f"{ref}:{x}:{y}")
    fp = footprint or "~"
    lines = [
        "$Comp",
        f"L {lib} {ref}",
        f"U 1 1 {uid}",
        f"P {x} {y}",
        f'F 0 "{ref}" H {x} {y - 150} 50  0000 {field_orient} CNN',
        f'F 1 "{value}" H {x} {y + 150} 50  0000 {field_orient} CNN',
        f'F 2 "{fp}" H {x} {y} 50  0001 C CNN',
        f'F 3 "{datasheet}" H {x} {y} 50  0001 C CNN',
        f"\t1    {x} {y}",
    ]
    mirror = "0" if rot in (0, 180) else "1"
    lines.append(f"\t{mirror}    {rot % 360}    0    -1  ")
    lines.append("$EndComp")
    return "\n".join(lines)


def pwr(ref: str, value: str, x: int, y: int) -> str:
    uid = comp_uuid(f"pwr:{ref}:{x}:{y}")
    return "\n".join(
        [
            "$Comp",
            f"L power:{value} {ref}",
            f"U 1 1 {uid}",
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
    uid = comp_uuid(f"cap:{ref}")
    return "\n".join(
        [
            "$Comp",
            f"L Device:C_Small {ref}",
            f"U 1 1 {uid}",
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


def build() -> str:
    parts: list[str] = []
    wires: list[str] = []
    labels: list[str] = []

    j1_x, j1_y = 1200, 3600
    u1_x, u1_y = 3600, 2200
    u3_x, u3_y = 5600, 2200
    u2_x, u2_y = 7600, 2200

    parts.extend(
        [
            comp(
                "Connector:Conn_01x44_Pin",
                "J1",
                "MaxxCard_Edge_44x2.54mm",
                j1_x,
                j1_y,
                footprint="Connector:Conn_01x44_Pin",
            ),
            comp(
                "Memory_EPROM:27C512",
                "U1",
                "27C512",
                u1_x,
                u1_y,
                footprint="Package_DIP:DIP-28_W15.24mm",
                datasheet="${KIPRJMOD}/../../../../DataSheets/Mitsubishi-KM2365.pdf",
            ),
            comp(
                "Connector:Conn_01x24_Pin",
                "U3",
                "5085-TBD",
                u3_x,
                u3_y,
                footprint="Package_DIP:DIP-24_W15.24mm",
            ),
            comp(
                "74xx:74HC14",
                "U2",
                "74HC14?",
                u2_x,
                u2_y,
                footprint="Package_DIP:DIP-14_W7.62mm",
            ),
        ]
    )

    for ref, x, y in [("C1", 3300, 1700), ("C2", 5300, 1700), ("C3", 7300, 1700)]:
        parts.append(cap(ref, x, y))

    parts.extend([pwr("#PWR01", "GND", 1200, 5200), pwr("#PWR02", "+5V", 1600, 5200)])

    rail_y = 1400
    gnd_y = 3200
    bus_x_j = 2000
    bus_x_u1 = 3200
    bus_x_u3 = 5400
    bus_x_u2 = 7400

    wires.extend(
        [
            wire(1600, 5200, 1600, rail_y),
            wire(1600, rail_y, 8200, rail_y),
            wire(1200, 5200, 1200, gnd_y),
            wire(1200, gnd_y, 8200, gnd_y),
        ]
    )

    for cx in (3300, 5300, 7300):
        wires.extend([wire(cx, rail_y, cx, 1700), wire(cx, 1900, cx, gnd_y)])

    # Address bus: one labeled trunk J1 -> U1 -> U3
    for i in range(16):
        y = 4000 - i * 100
        wires.extend(
            [
                wire(bus_x_j, y, bus_x_u1, y),
                wire(bus_x_u1, y, bus_x_u3, y),
            ]
        )
        labels.append(label(f"A{i}", bus_x_j + 50, y))

    # Data bus: J1 -> U1
    for i in range(8):
        y = 5600 + i * 100
        wires.append(wire(bus_x_j, y, bus_x_u1, y))
        labels.append(label(f"D{i}", bus_x_j + 50, y))

    # Control trunk (provisional)
    for net, y in [("/CE", 2400), ("/OE", 2500), ("PHI2", 5450), ("R/W", 5550)]:
        wires.extend(
            [
                wire(bus_x_j, y, bus_x_u2, y),
                wire(bus_x_u2, y, bus_x_u1, y),
            ]
        )
        labels.append(label(net, bus_x_j + 50, y))

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

    body = "\n".join(parts + wires + labels)
    return header + "\n" + body + "\n$EndSCHEMATC\n"


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    OUT_SCH.write_text(build())
    print(f"Wrote {OUT_SCH}")


if __name__ == "__main__":
    main()