#!/usr/bin/env python3
"""Convert legacy EESchema v4 .sch files to KiCad 10 native .kicad_sch + .kicad_pro."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import uuid
from dataclasses import dataclass, field
from pathlib import Path

from kicad_sym_extract import KICAD_SYM_DIR, extract_symbol_block

ROOT = Path(__file__).resolve().parents[1]
COP41XL_SYM = ROOT / "libraries/cop41xl.kicad_sym"
KICAD_PRO_TEMPLATE = Path(
    "/usr/share/kicad/template/KiCad_MR_diagrams_small_parts/mr_diagrams_small_parts.kicad_pro"
)

# Legacy transform matrix -> (rotation_deg, mirror_axis or None). First match wins (KiCad behavior).
TRANSFORM_TABLE: list[tuple[tuple[int, int, int, int], tuple[int, str | None]]] = [
    ((1, 0, 0, -1), (0, None)),
    ((0, -1, -1, 0), (90, None)),
    ((1, 0, 0, 1), (180, None)),
    ((0, 1, 1, 0), (270, None)),
    ((0, -1, 1, 0), (90, "x")),
    ((1, 0, 0, -1), (180, "x")),  # unreachable if normal 0 matched first; kept for parity
    ((0, 1, -1, 0), (270, "x")),
]

LIB_ID_REMAP = {
    "OPL_Resistor:DIP-RES-4.7K-5%-1_4W_PR-D2.3XL6.5MM_": "Device:R_US",
    "Connector:Conn_01x02_Male": "Connector:Conn_01x02_Pin",
    "Connector:Conn_01x14_Male": "Connector:Conn_01x14_Pin",
    "Device:CP": "Device:C_Polarized",
}

PAPER_SIZES = {
    "A4": "A4",
    "A3": "A3",
    "A2": "A2",
    "A1": "A1",
    "A0": "A0",
    "USLetter": "USLetter",
    "USLegal": "USLegal",
}


@dataclass
class LegacyComponent:
    lib_id: str
    ref: str
    fields: dict[int, str]
    field_pos: dict[int, tuple[float, float, int]]
    x: int
    y: int
    transform: tuple[int, int, int, int]
    unit: int = 1


@dataclass
class LegacySchematic:
    paper: str
    title: str
    date: str
    rev: str
    company: str
    comments: list[str]
    components: list[LegacyComponent] = field(default_factory=list)
    wires: list[tuple[int, int, int, int]] = field(default_factory=list)
    labels: list[tuple[str, int, int, int]] = field(default_factory=list)
    junctions: list[tuple[int, int]] = field(default_factory=list)
    no_connects: list[tuple[int, int]] = field(default_factory=list)
    bus_entries: list[tuple[int, int, int, int]] = field(default_factory=list)


def uid(project: str, seed: str) -> str:
    return str(uuid.uuid5(uuid.NAMESPACE_DNS, f"maxx-steele/{project}/{seed}"))


def mm(units: int) -> float:
    return units / 100.0


def decode_transform(values: tuple[int, int, int, int]) -> tuple[int, str | None]:
    for key, result in TRANSFORM_TABLE:
        if values == key:
            return result
    return (0, None)


def extract_symbol_from_file(lib_path: Path, lib_id: str) -> str:
    lib_name, sym_name = lib_id.split(":", 1)
    text = lib_path.read_text()
    needle = f'(symbol "{sym_name}"'
    start = text.find(needle)
    if start < 0:
        raise ValueError(f"{sym_name} not found in {lib_path}")
    depth = 0
    end = start
    for i, ch in enumerate(text[start:], start):
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                end = i + 1
                break
    block = text[start:end]
    return block.replace(f'(symbol "{sym_name}"', f'(symbol "{lib_id}"', 1)


def get_lib_symbol_block(lib_id: str) -> str:
    lib_name = lib_id.split(":", 1)[0]
    if lib_name == "cop41xl":
        return extract_symbol_from_file(COP41XL_SYM, lib_id)
    return extract_symbol_block(lib_id)


def pin_numbers_for_lib(lib_id: str) -> list[str]:
    block = get_lib_symbol_block(lib_id)
    nums = re.findall(r'\(number "([^"]+)"', block)
    seen: list[str] = []
    for n in nums:
        if n not in seen:
            seen.append(n)
    return seen


def parse_legacy_sch(path: Path) -> LegacySchematic:
    lines = path.read_text().splitlines()
    sch = LegacySchematic(paper="A4", title="", date="", rev="", company="", comments=["", "", "", ""])

    i = 0
    while i < len(lines):
        line = lines[i].strip()
        if line.startswith("$Descr"):
            parts = line.split()
            sch.paper = PAPER_SIZES.get(parts[1], parts[1])
            i += 1
            continue
        if line.startswith("Title "):
            sch.title = line.split(" ", 1)[1].strip().strip('"')
        elif line.startswith("Date "):
            sch.date = line.split(" ", 1)[1].strip().strip('"')
        elif line.startswith("Rev "):
            sch.rev = line.split(" ", 1)[1].strip().strip('"')
        elif line.startswith("Comp "):
            sch.company = line.split(" ", 1)[1].strip().strip('"')
        elif line.startswith("Comment") and line[7].isdigit():
            idx = int(line[7]) - 1
            sch.comments[idx] = line.split(" ", 1)[1].strip().strip('"')
        elif line == "$Comp":
            comp_lines = []
            i += 1
            while i < len(lines) and lines[i].strip() != "$EndComp":
                comp_lines.append(lines[i])
                i += 1
            sch.components.append(_parse_component(comp_lines))
        elif line == "Wire Wire Line":
            i += 1
            coords = list(map(int, lines[i].split()))
            sch.wires.append((coords[0], coords[1], coords[2], coords[3]))
        elif line.startswith("Text Label "):
            parts = line.split()
            x, y, orient = int(parts[2]), int(parts[3]), int(parts[4])
            i += 1
            text = lines[i].strip()
            sch.labels.append((text, x, y, orient))
        elif line.startswith("Connection ~"):
            parts = line.split()
            sch.junctions.append((int(parts[2]), int(parts[3])))
        elif line.startswith("NoConn ~"):
            parts = line.split()
            sch.no_connects.append((int(parts[2]), int(parts[3])))
        elif line == "Entry Wire Line":
            i += 1
            coords = list(map(int, lines[i].split()))
            sch.bus_entries.append((coords[0], coords[1], coords[2], coords[3]))
        i += 1

    return sch


def _parse_component(comp_lines: list[str]) -> LegacyComponent:
    lib_line = next(l for l in comp_lines if l.startswith("L "))
    _, lib_id, ref = lib_line.split(maxsplit=2)

    pos_line = next(l for l in comp_lines if l.startswith("P "))
    x, y = map(int, pos_line.split()[1:3])

    tab_lines = [l for l in comp_lines if l.startswith("\t")]
    if len(tab_lines) < 2:
        raise ValueError(f"Component {ref} missing transform lines")
    tvals = tuple(map(int, tab_lines[-1].split()))
    while len(tvals) < 4:
        tvals = (*tvals, 0)

    fields: dict[int, str] = {}
    field_pos: dict[int, tuple[float, float, int]] = {}
    for cl in comp_lines:
        if not cl.startswith("F "):
            continue
        m = re.match(
            r'F (\d+) "([^"]*)"\s+.\s+(-?\d+)\s+(-?\d+)\s+(\d+)\s+(\d+)',
            cl,
        )
        if not m:
            m = re.match(r'F (\d+) "([^"]*)"', cl)
            if not m:
                continue
            idx = int(m.group(1))
            fields[idx] = m.group(2)
            continue
        idx = int(m.group(1))
        fields[idx] = m.group(2)
        field_pos[idx] = (mm(int(m.group(3))), mm(int(m.group(4))), int(m.group(6)))

    return LegacyComponent(
        lib_id=LIB_ID_REMAP.get(lib_id, lib_id),
        ref=ref,
        fields=fields,
        field_pos=field_pos,
        x=x,
        y=y,
        transform=tvals,  # type: ignore[arg-type]
    )


def label_justify(orient: int) -> tuple[int, str]:
    if orient == 2:
        return 0, "right bottom"
    if orient == 1:
        return 90, "left bottom"
    if orient == 3:
        return 270, "left bottom"
    return 0, "left bottom"


def build_kicad_sch(sch: LegacySchematic, project: str, root_uuid: str) -> str:
    lib_ids = sorted({c.lib_id for c in sch.components})
    lib_blocks = []
    pin_cache: dict[str, list[str]] = {}
    for lib_id in lib_ids:
        block = get_lib_symbol_block(lib_id)
        lib_blocks.append("\t" + block.replace("\n", "\n\t"))
        pin_cache[lib_id] = pin_numbers_for_lib(lib_id)

    out: list[str] = [
        "(kicad_sch",
        "\t(version 20250114)",
        '\t(generator "maxx-steele-tools")',
        '\t(generator_version "1.0")',
        f'\t(uuid "{root_uuid}")',
        f'\t(paper "{sch.paper}")',
        "\t(title_block",
        f'\t\t(title "{sch.title}")',
        f'\t\t(date "{sch.date}")',
        f'\t\t(rev "{sch.rev}")',
        f'\t\t(company "{sch.company}")',
    ]
    for n, comment in enumerate(sch.comments, start=1):
        out.append(f'\t\t(comment {n} "{comment}")')
    out.extend(["\t)", "\t(lib_symbols", *lib_blocks, "\t)"])

    for comp in sch.components:
        rotation, mirror = decode_transform(comp.transform)
        ref = comp.fields.get(0, comp.ref)
        value = comp.fields.get(1, "")
        footprint = comp.fields.get(2, "")
        datasheet = comp.fields.get(3, "~")
        sym_uid = uid(project, f"sym/{ref}")

        out.append("\t(symbol")
        out.append(f'\t\t(lib_id "{comp.lib_id}")')
        out.append(f"\t\t(at {mm(comp.x)} {mm(comp.y)} {rotation})")
        if mirror:
            out.append(f"\t\t(mirror {mirror})")
        out.append("\t\t(unit 1)")
        out.append("\t\t(exclude_from_sim no)")
        out.append("\t\t(in_bom yes)")
        out.append("\t\t(on_board yes)")
        out.append("\t\t(dnp no)")
        out.append(f'\t\t(uuid "{sym_uid}")')

        for prop_name, idx in [("Reference", 0), ("Value", 1), ("Footprint", 2), ("Datasheet", 3)]:
            val = comp.fields.get(idx, "")
            if prop_name == "Reference":
                val = ref
            if prop_name == "Datasheet" and not val:
                val = "~"
            if idx in comp.field_pos:
                px, py, ang = comp.field_pos[idx]
                out.append(f'\t\t(property "{prop_name}" "{val}"')
                out.append(f"\t\t\t(at {px} {py} {ang})")
            else:
                out.append(f'\t\t(property "{prop_name}" "{val}"')
                out.append(f"\t\t\t(at {mm(comp.x)} {mm(comp.y)} 0)")
            hide = "yes" if prop_name in ("Footprint", "Datasheet") and val in ("", "~") else None
            out.append("\t\t\t(effects (font (size 1.27 1.27))" + (f" (hide yes)" if hide else "") + ")")
            out.append("\t\t)")

        for extra_idx in sorted(k for k in comp.fields if k >= 4):
            val = comp.fields[extra_idx]
            out.append(f'\t\t(property "{val}" "{val}"')
            out.append(f"\t\t\t(at {mm(comp.x)} {mm(comp.y)} 0)")
            out.append("\t\t\t(effects (font (size 1.27 1.27)) (hide yes))")
            out.append("\t\t)")

        for pin in pin_cache[comp.lib_id]:
            out.append(f'\t\t(pin "{pin}"')
            out.append(f'\t\t\t(uuid "{uid(project, f"pin/{ref}/{pin}")}")')
            out.append("\t\t)")

        out.extend(
            [
                "\t\t(instances",
                f'\t\t\t(project "{project}"',
                f'\t\t\t\t(path "/{root_uuid}"',
                f'\t\t\t\t\t(reference "{ref}")',
                "\t\t\t\t\t(unit 1)",
                "\t\t\t\t)",
                "\t\t\t)",
                "\t\t)",
                "\t)",
            ]
        )

    for x1, y1, x2, y2 in sch.wires:
        out.extend(
            [
                "\t(wire",
                "\t\t(pts",
                f"\t\t\t(xy {mm(x1)} {mm(y1)})",
                f"\t\t\t(xy {mm(x2)} {mm(y2)})",
                "\t\t)",
                "\t\t(stroke (width 0) (type default))",
                f'\t\t(uuid "{uid(project, f"wire/{x1}/{y1}/{x2}/{y2}")}")',
                "\t)",
            ]
        )

    for text, x, y, orient in sch.labels:
        angle, justify = label_justify(orient)
        out.extend(
            [
                "\t(label",
                f'\t\t"{text}"',
                f"\t\t(at {mm(x)} {mm(y)} {angle})",
                f"\t\t(effects (font (size 1.27 1.27)) (justify {justify}))",
                f'\t\t(uuid "{uid(project, f"label/{text}/{x}/{y}")}")',
                "\t)",
            ]
        )

    for x, y in sch.junctions:
        out.extend(
            [
                "\t(junction",
                f"\t\t(at {mm(x)} {mm(y)})",
                "\t\t(diameter 0)",
                f'\t\t(uuid "{uid(project, f"junc/{x}/{y}")}")',
                "\t)",
            ]
        )

    for x, y in sch.no_connects:
        out.extend(
            [
                "\t(no_connect",
                f"\t\t(at {mm(x)} {mm(y)})",
                f'\t\t(uuid "{uid(project, f"nc/{x}/{y}")}")',
                "\t)",
            ]
        )

    for x1, y1, x2, y2 in sch.bus_entries:
        dx = mm(x2 - x1)
        dy = mm(y2 - y1)
        mag = 2.54
        sx = mag if dx >= 0 else -mag
        sy = mag if dy >= 0 else -mag
        if abs(dx) < 0.01:
            sx = 0
        if abs(dy) < 0.01:
            sy = 0
        out.extend(
            [
                "\t(bus_entry",
                f"\t\t(at {mm(x1)} {mm(y1)})",
                f"\t\t(size {sx} {sy})",
                "\t\t(stroke (width 0.1524) (type solid))",
                f'\t\t(uuid "{uid(project, f"bus/{x1}/{y1}/{x2}/{y2}")}")',
                "\t)",
            ]
        )

    out.extend(
        [
            "\t(sheet_instances",
            f'\t\t(path "/{root_uuid}"',
            '\t\t\t(page "1")',
            "\t\t)",
            "\t)",
            "\t(embedded_fonts no)",
            ")",
            "",
        ]
    )
    return "\n".join(out)


def build_kicad_pro(project: str, root_uuid: str) -> str:
    template = json.loads(KICAD_PRO_TEMPLATE.read_text())
    template["meta"]["filename"] = f"{project}.kicad_pro"
    template["sheets"] = [[root_uuid, "Root"]]
    return json.dumps(template, indent=2) + "\n"


def upgrade_project(legacy_sch: Path, export_pdf: bool = True) -> None:
    project = legacy_sch.stem
    out_dir = legacy_sch.parent
    root_uuid = uid(project, "root")
    parsed = parse_legacy_sch(legacy_sch)

    kicad_sch = out_dir / f"{project}.kicad_sch"
    kicad_pro = out_dir / f"{project}.kicad_pro"
    kicad_sch.write_text(build_kicad_sch(parsed, project, root_uuid))
    kicad_pro.write_text(build_kicad_pro(project, root_uuid))

    if export_pdf:
        pdf = out_dir / f"{project}-schematic.pdf"
        subprocess.run(
            ["kicad-cli", "sch", "export", "pdf", str(kicad_sch), "-o", str(pdf)],
            check=True,
            capture_output=True,
        )
        print(f"Wrote {pdf}")

    print(f"Wrote {kicad_sch}")
    print(f"Wrote {kicad_pro}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("legacy_sch", type=Path, help="Legacy .sch file to upgrade")
    parser.add_argument("--no-pdf", action="store_true", help="Skip PDF export")
    args = parser.parse_args()

    if not COP41XL_SYM.exists():
        subprocess.run(
            [
                "kicad-cli",
                "sym",
                "upgrade",
                str(ROOT / "libraries/cop41xl.lib"),
                "-o",
                str(COP41XL_SYM),
            ],
            check=True,
        )

    upgrade_project(args.legacy_sch.resolve(), export_pdf=not args.no_pdf)


if __name__ == "__main__":
    main()