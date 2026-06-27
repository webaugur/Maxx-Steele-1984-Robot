#!/usr/bin/env python3
"""Maxx Steele cartridge ROM utilities.

Decode 4KB cartridge images (e.g. CBSDemo.532, UltraMaxx.532), validate structure, and
emit human-readable program listings compatible with the R. Wind .dsm format.
"""

from __future__ import annotations

import argparse
import json
import re
import struct
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

from project_paths import resolve_from_root

COPYRIGHT_CBS = b"(c) 1985 CBS Toys"
COPYRIGHT_ULTRAMAXX = b"(c) UltraMaxx    "
COPYRIGHT_MAXXOS = b"(c) MaxxOS       "
COPYRIGHTS = (COPYRIGHT_CBS, COPYRIGHT_ULTRAMAXX, COPYRIGHT_MAXXOS)
CART_SIZE = 4096
PROG_RAM = 0x0200
PHRASE_RAM = 0x0500
MUSIC_RAM = 0x0400

# Display names from internal ROM table at $F878 (R. Wind / maxx_internal_ROM.dsm)
OPCODES: dict[int, dict] = {
    0x00: {"name": "turn_left", "display": "L", "operand": "distance"},
    0x01: {"name": "drive_forward", "display": "F", "operand": "distance"},
    0x02: {"name": "drive_reverse", "display": "b", "operand": "distance"},
    0x03: {"name": "turn_right", "display": "r", "operand": "angle"},
    0x04: {"name": "wrist_up", "display": "Uu", "operand": "value"},
    0x05: {"name": "wrist_down", "display": "Ud", "operand": "value"},
    0x06: {"name": "arms_up", "display": "Au", "operand": "value"},
    0x07: {"name": "arms_down", "display": "Ad", "operand": "value"},
    0x08: {"name": "claw_rotate", "display": "Cr", "operand": "value"},
    0x09: {"name": "claw_open_close", "display": "Cc", "operand": "0=open,1=close"},
    0x0A: {"name": "lamp", "display": "HL", "operand": "0=off,1=on"},
    0x0B: {"name": "home", "display": "init", "operand": "must be 0"},
    0x0C: {"name": "delay", "display": "d", "operand": "seconds"},
    0x0D: {"name": "song_number", "display": "Sn", "operand": "song index"},
    0x0E: {"name": "speech_rom", "display": "S", "operand": "phrase #"},
    0x0F: {"name": "speech_shift", "display": "SS", "operand": "phrase #"},
    0x10: {"name": "speech_program", "display": "PS", "operand": "phrase #"},
    0x41: {"name": "game", "display": "?", "operand": "game id"},
    0x43: {"name": "click", "display": "?", "operand": "value"},
    0x46: {"name": "motion_step", "display": "?", "operand": "value"},
    0x80: {"name": "extended_0", "display": "ext", "operand": "maps to 0x0C"},
    0x81: {"name": "play_tune", "display": "PLAY", "operand": "tune #"},
    0x82: {"name": "speak_rom", "display": "SPEE", "operand": "phrase #"},
    0x83: {"name": "speak_ram", "display": "SS", "operand": "RAM phrase #"},
    0x84: {"name": "speech_clear", "display": "CLr", "operand": "phrase slot"},
    0xFE: {"name": "marker_beg", "display": "beg", "operand": "display only"},
    0xFF: {"name": "end", "display": "End", "operand": "must be 0xFF"},
}

STATUS_BITS = {
    0x02: {
        0: "SEr (speech error)",
        1: "Ebof (enable backoff)",
        2: "PDon (power down on)",
        3: "Edof (enable drive off)",
        4: "SPon (speech on)",
        5: "reserved",
        6: "reserved",
        7: "reserved",
    },
    0x03: {
        0: "UCon (user control on)",
        1: "SPon (speech enable)",
        2: "reserved",
        3: "reserved",
        4: "Edof (execute disable?)",
        5: "reserved",
        6: "reserved",
        7: "reserved",
    },
}


@dataclass
class CartImage:
    data: bytes
    base_addr: int = 0xA000

    @classmethod
    def load(cls, path: Path, base_addr: int = 0xA000) -> CartImage:
        data = path.read_bytes()
        if len(data) != CART_SIZE:
            raise ValueError(f"expected {CART_SIZE} bytes, got {len(data)}")
        return cls(data=data, base_addr=base_addr)

    @property
    def entry_vector(self) -> int:
        return self.data[0] | (self.data[1] << 8)

    @property
    def copyright(self) -> bytes:
        # 17-byte string ending immediately before entry code at offset $13
        return self.data[2:19]

    def offset(self, addr: int) -> int:
        return addr - self.base_addr


def describe_status(byte_val: int, reg: int) -> str:
    names = []
    for bit, label in STATUS_BITS.get(reg, {}).items():
        if byte_val & (1 << bit):
            names.append(label.split()[0])
    return ", ".join(names) if names else "none"


def decode_program(data: bytes, start: int, end: int | None = None) -> list[tuple[int, int, int, str]]:
    """Return (address, opcode, operand, comment) tuples."""
    lines: list[tuple[int, int, int, str]] = []
    i = start
    limit = end if end is not None else len(data)
    while i + 1 < limit:
        op, operand = data[i], data[i + 1]
        info = OPCODES.get(op, {"name": f"unknown_{op:02X}", "display": "?", "operand": "?"})
        comment = info["name"].replace("_", " ")
        if op == 0x0C:
            comment = f"delay {operand} second{'s' if operand != 1 else ''}"
        elif op == 0x0A:
            comment = "turn light on" if operand else "turn light off"
        elif op == 0x81:
            comment = f"play tune #{operand}"
        elif op in (0x82, 0x83, 0x0E):
            comment = f"speak phrase #{operand}"
        elif op == 0xFF and operand == 0xFF:
            comment = "end"
        lines.append((i, op, operand, comment))
        if op == 0xFF and operand == 0xFF:
            break
        i += 2
    return lines


def find_program_table(data: bytes, base_addr: int) -> int | None:
    """Locate program bytecode by scanning for typical cart entry pattern."""
    # Standard Maxx carts place the program table at base+$35 (see demo cart).
    if data[0x35] != 0xFF or data[0x36] != 0xFF:
        return 0x35
    for marker in (b"\x83\x10", b"\x0c\x02", b"\x81\x00", b"\xff\xff"):
        idx = data.find(marker)
        if idx != -1 and 0x20 <= idx < 0x200:
            return idx
    return None


def parse_dsm_program_comments(path: Path) -> dict[tuple[int, int], str]:
    """Load annotated comments from an existing .dsm listing."""
    comments: dict[tuple[int, int], str] = {}
    in_prog = False
    for raw in path.read_text().splitlines():
        line = raw.strip().rstrip("\r")
        if "program table copied" in line.lower():
            in_prog = True
            continue
        if in_prog and "phrase table" in line.lower():
            break
        m = re.match(r"^[0-9A-F]{6,8}:\s*([0-9A-F]{2})\s+([0-9A-F]{2})\s*(.*)$", line, re.I)
        if m and in_prog:
            op = int(m.group(1), 16)
            operand = int(m.group(2), 16)
            text = m.group(3).strip()
            if text:
                comments[(op, operand)] = text
    return comments


def returns_to_main_loop(cart: CartImage) -> bool:
    """True if bootstrap hands off to internal ROM at $E0B6 (factory pattern)."""
    entry_off = cart.offset(cart.entry_vector)
    if entry_off < 0 or entry_off >= CART_SIZE:
        return False
    chunk = cart.data[entry_off : entry_off + 64]
    return b"\x4c\xb6\xe0" in chunk


def validate_cart(cart: CartImage) -> list[str]:
    issues: list[str] = []
    if cart.copyright not in COPYRIGHTS:
        issues.append(f"copyright mismatch: {cart.copyright!r}")
    entry_off = cart.offset(cart.entry_vector)
    if entry_off < 0 or entry_off >= CART_SIZE:
        issues.append(f"entry vector ${cart.entry_vector:04X} out of range")
    else:
        # Entry should begin with LDA #$02 / STA $02
        chunk = cart.data[entry_off : entry_off + 4]
        if chunk[:2] != b"\xa9\x02" or chunk[2:4] != b"\x85\x02":
            issues.append(
                f"unexpected entry code at ${cart.entry_vector:04X}: {chunk.hex()}"
            )
    if returns_to_main_loop(cart):
        prog_start = find_program_table(cart.data, cart.base_addr)
        if prog_start is None:
            issues.append("could not locate program table")
        else:
            prog = decode_program(cart.data, prog_start)
            if not prog or prog[-1][1:3] != (0xFF, 0xFF):
                issues.append("program table missing FF FF terminator")
    return issues


def format_listing(cart: CartImage, dsm_comments: dict | None = None) -> str:
    dsm_comments = dsm_comments or {}
    out: list[str] = []
    out.append(f"// Cartridge image @ ${cart.base_addr:04X}")
    out.append(f"// Entry vector: ${cart.entry_vector:04X}")
    out.append(f"// Copyright: {cart.copyright.decode('ascii', errors='replace')}")
    out.append("")

    prog_start = find_program_table(cart.data, cart.base_addr)
    if prog_start is None:
        out.append("// ERROR: program table not found")
        return "\n".join(out)

    addr = cart.base_addr + prog_start
    out.append(f"// Program table @ ${addr:04X} -> RAM ${PROG_RAM:04X}")
    for rom_addr, op, operand, comment in decode_program(cart.data, prog_start):
        key = (op, operand)
        if key in dsm_comments:
            comment = dsm_comments[key]
        out.append(
            f"{cart.base_addr + rom_addr:06X}: {op:02X} {operand:02X}  {comment}"
        )
    return "\n".join(out)


def emit_template(path: Path, base_addr: int = 0xA000) -> None:
    """Write a minimal valid cartridge skeleton."""
    img = bytearray(b"\xFF" * CART_SIZE)
    # Header
    entry = 0x0013  # offset from base
    struct.pack_into("<H", img, 0, entry)
    img[2 : 2 + len(COPYRIGHT_CBS)] = COPYRIGHT_CBS

    # Entry stub (matches demo cart)
    off = entry
    stub = bytes(
        [
            0xA9,
            0x02,
            0x85,
            0x02,  # LDA #$02 / STA $02
            0xA9,
            0x82,
            0x85,
            0x03,  # LDA #$82 / STA $03
            0xA2,
            0x00,  # LDX #0
            0xBD,
            0x35,
            0xA0,  # LDA $A035,X
            0x9D,
            0x00,
            0x02,  # STA $0200,X
            0xBD,
            0x81,
            0xA0,  # LDA $A081,X  phrases
            0x9D,
            0x00,
            0x05,  # STA $0500,X
            0xBD,
            0xBB,
            0xA0,  # LDA $A0BB,X  music
            0x9D,
            0x00,
            0x04,  # STA $0400,X
            0xE8,  # INX
            0xD0,
            0xEB,  # BNE
            0x4C,
            0xB6,
            0xE0,  # JMP $E0B6
        ]
    )
    img[off : off + len(stub)] = stub

    # Minimal program at $A035
    prog_off = 0x35
    program = bytes(
        [
            0x83,
            0x10,  # speak RAM phrase 0 (placeholder)
            0x0C,
            0x02,  # delay 2s
            0x01,
            0x14,  # forward
            0xFF,
            0xFF,
        ]
    )
    img[prog_off : prog_off + len(program)] = program

    # Empty phrase + music tables
    for i in range(0x40):
        img[0x81 + i] = 0xFF
    for i in range(0x10):
        img[0xBB + i] = 0x00

    path.write_bytes(img)


def export_opcode_json(path: Path) -> None:
    path.write_text(json.dumps(OPCODES, indent=2))


def main(argv: Iterable[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_dis = sub.add_parser("disasm", help="disassemble cartridge binary")
    p_dis.add_argument("cart")
    p_dis.add_argument("--base", type=lambda x: int(x, 0), default=0xA000)
    p_dis.add_argument("--compare-dsm", help="merge comments from reference .dsm")

    p_val = sub.add_parser("validate", help="validate cartridge structure")
    p_val.add_argument("cart")
    p_val.add_argument("--base", type=lambda x: int(x, 0), default=0xA000)

    p_tpl = sub.add_parser("template", help="write minimal cartridge skeleton")
    p_tpl.add_argument("output")

    p_op = sub.add_parser("opcodes", help="export opcode table JSON")
    p_op.add_argument("output")

    args = parser.parse_args(list(argv) if argv is not None else None)

    if args.cmd == "disasm":
        cart = CartImage.load(resolve_from_root(args.cart), args.base)
        comments = None
        if args.compare_dsm:
            comments = parse_dsm_program_comments(resolve_from_root(args.compare_dsm))
        print(format_listing(cart, comments))
        return 0

    if args.cmd == "validate":
        cart = CartImage.load(resolve_from_root(args.cart), args.base)
        issues = validate_cart(cart)
        if issues:
            for issue in issues:
                print(f"FAIL: {issue}")
            return 1
        print("OK: cartridge structure looks valid")
        return 0

    if args.cmd == "template":
        emit_template(resolve_from_root(args.output), args.base if hasattr(args, "base") else 0xA000)
        print(f"wrote template to {args.output}")
        return 0

    if args.cmd == "opcodes":
        export_opcode_json(resolve_from_root(args.output))
        print(f"wrote opcode table to {args.output}")
        return 0

    return 2


if __name__ == "__main__":
    sys.exit(main())