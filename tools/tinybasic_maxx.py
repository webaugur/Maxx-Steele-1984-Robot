#!/usr/bin/env python3
"""MaxxBAS — compile a BASIC-like Maxx program into a 4 KB cartridge image.

MaxxBAS v1 is a line-oriented DSL that maps 1:1 to Maxx bytecode pairs. The
compiler emits the same cartridge layout as tools/maxx_rom.py emit_template()
(bootstrap stub, program @ $A035, phrase/music tables).

Statements (case-insensitive):
  DELAY n       FORWARD n     BACK n        LEFT n        RIGHT n
  LAMP ON|OFF   HOME          PLAY n        SAY n         SPEAK n
  END

Optional line numbers and comments (# or REM) are stripped. SAY emits opcode $83
(RAM phrase); SPEAK emits $82 (ROM phrase). Custom phrase bytes are not authored
in v1 — use SPEAK for built-in ROM speech.

Example:
  python3 tools/tinybasic_maxx.py compile hello.bas -o hello.532
  python3 tools/maxx_rom.py validate hello.532
"""

from __future__ import annotations

import argparse
import re
import struct
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

from maxx_rom import (
    CART_SIZE,
    COPYRIGHT_CBS,
    COPYRIGHT_ULTRAMAXX,
    CartImage,
    validate_cart,
)
from project_paths import resolve_from_root

# Cartridge layout (offsets from $A000)
ENTRY_ADDR = 0xA013
ENTRY_OFF = 0x0013
PROG_OFF = 0x35
PHRASE_OFF = 0x81
MUSIC_OFF = 0xBB
MAX_PROG_BYTES = PHRASE_OFF - PROG_OFF  # 76 bytes = 38 pairs

COPYRIGHTS = {
    "cbs": COPYRIGHT_CBS,
    "ultramaxx": COPYRIGHT_ULTRAMAXX,
}

# Fixed 6502 bootstrap (matches factory demo / emit_template)
BOOTSTRAP_STUB = bytes(
    [
        0xA9,
        0x02,
        0x85,
        0x02,
        0xA9,
        0x82,
        0x85,
        0x03,
        0xA2,
        0x00,
        0xBD,
        0x35,
        0xA0,
        0x9D,
        0x00,
        0x02,
        0xBD,
        0x81,
        0xA0,
        0x9D,
        0x00,
        0x05,
        0xBD,
        0xBB,
        0xA0,
        0x9D,
        0x00,
        0x04,
        0xE8,
        0xD0,
        0xEB,
        0x4C,
        0xB6,
        0xE0,
    ]
)


class CompileError(Exception):
    """Raised when source cannot be compiled."""


@dataclass(frozen=True)
class Instruction:
    opcode: int
    operand: int
    source_line: int
    text: str

    def as_bytes(self) -> bytes:
        return bytes((self.opcode & 0xFF, self.operand & 0xFF))


def strip_comments(line: str) -> str:
    """Remove end-of-line comments (# or REM)."""
    if "#" in line:
        line = line[: line.index("#")]
    if re.match(r"^\s*REM\b", line, re.I):
        return ""
    m = re.search(r"\s+REM\b", line, re.I)
    if m:
        line = line[: m.start()]
    return line.strip()


def parse_operand(token: str, line_no: int, stmt: str) -> int:
    try:
        value = int(token, 0)
    except ValueError as exc:
        raise CompileError(f"line {line_no}: {stmt}: expected integer operand, got {token!r}") from exc
    if not 0 <= value <= 255:
        raise CompileError(f"line {line_no}: {stmt}: operand {value} out of range 0..255")
    return value


def parse_line(raw: str, line_no: int) -> Instruction | None:
    line = strip_comments(raw)
    if not line:
        return None

    # Optional line number prefix (10 DELAY 2)
    m = re.match(r"^(\d+)\s+(.+)$", line)
    if m:
        line = m.group(2).strip()

    parts = line.split()
    if not parts:
        return None

    keyword = parts[0].upper()
    rest = parts[1:]

    if keyword == "DELAY":
        if len(rest) != 1:
            raise CompileError(f"line {line_no}: DELAY requires one operand (seconds)")
        return Instruction(0x0C, parse_operand(rest[0], line_no, "DELAY"), line_no, line)

    if keyword == "FORWARD":
        if len(rest) != 1:
            raise CompileError(f"line {line_no}: FORWARD requires one operand")
        return Instruction(0x01, parse_operand(rest[0], line_no, "FORWARD"), line_no, line)

    if keyword == "BACK":
        if len(rest) != 1:
            raise CompileError(f"line {line_no}: BACK requires one operand")
        return Instruction(0x02, parse_operand(rest[0], line_no, "BACK"), line_no, line)

    if keyword == "LEFT":
        if len(rest) != 1:
            raise CompileError(f"line {line_no}: LEFT requires one operand")
        return Instruction(0x00, parse_operand(rest[0], line_no, "LEFT"), line_no, line)

    if keyword == "RIGHT":
        if len(rest) != 1:
            raise CompileError(f"line {line_no}: RIGHT requires one operand")
        return Instruction(0x03, parse_operand(rest[0], line_no, "RIGHT"), line_no, line)

    if keyword == "LAMP":
        if len(rest) != 1 or rest[0].upper() not in ("ON", "OFF"):
            raise CompileError(f"line {line_no}: LAMP requires ON or OFF")
        return Instruction(0x0A, 1 if rest[0].upper() == "ON" else 0, line_no, line)

    if keyword == "HOME":
        if rest:
            raise CompileError(f"line {line_no}: HOME takes no operands")
        return Instruction(0x0B, 0, line_no, line)

    if keyword == "PLAY":
        if len(rest) != 1:
            raise CompileError(f"line {line_no}: PLAY requires tune number")
        return Instruction(0x81, parse_operand(rest[0], line_no, "PLAY"), line_no, line)

    if keyword == "SAY":
        if len(rest) != 1:
            raise CompileError(f"line {line_no}: SAY requires RAM phrase index")
        return Instruction(0x83, parse_operand(rest[0], line_no, "SAY"), line_no, line)

    if keyword == "SPEAK":
        if len(rest) != 1:
            raise CompileError(f"line {line_no}: SPEAK requires ROM phrase index")
        return Instruction(0x82, parse_operand(rest[0], line_no, "SPEAK"), line_no, line)

    if keyword == "END":
        if rest:
            raise CompileError(f"line {line_no}: END takes no operands")
        return Instruction(0xFF, 0xFF, line_no, line)

    raise CompileError(f"line {line_no}: unknown statement {keyword!r}")


def parse_source(text: str) -> list[Instruction]:
    program: list[Instruction] = []
    for line_no, raw in enumerate(text.splitlines(), start=1):
        insn = parse_line(raw, line_no)
        if insn is not None:
            program.append(insn)
    if not program:
        raise CompileError("empty program")
    if program[-1].opcode != 0xFF:
        program.append(Instruction(0xFF, 0xFF, 0, "END (implicit)"))
    elif program[-1].operand != 0xFF:
        raise CompileError(f"line {program[-1].source_line}: END must be sole statement or last line")
    return program


def program_bytes(program: Iterable[Instruction]) -> bytes:
    data = bytearray()
    for insn in program:
        data.extend(insn.as_bytes())
    if len(data) > MAX_PROG_BYTES:
        pairs = len(data) // 2
        raise CompileError(
            f"program too large: {pairs} pairs ({len(data)} bytes), max {MAX_PROG_BYTES // 2}"
        )
    return bytes(data)


def emit_cart(
    program: list[Instruction],
    *,
    copyright: bytes = COPYRIGHT_ULTRAMAXX,
    phrase_table: bytes | None = None,
    music_table: bytes | None = None,
) -> bytes:
    """Build a 4096-byte cartridge image."""
    if len(copyright) != 17:
        raise CompileError(f"copyright must be 17 bytes, got {len(copyright)}")

    img = bytearray(b"\xFF" * CART_SIZE)
    struct.pack_into("<H", img, 0, ENTRY_ADDR)
    img[2 : 2 + 17] = copyright
    img[ENTRY_OFF : ENTRY_OFF + len(BOOTSTRAP_STUB)] = BOOTSTRAP_STUB

    prog = program_bytes(program)
    img[PROG_OFF : PROG_OFF + len(prog)] = prog

    phrases = phrase_table if phrase_table is not None else bytes([0xFF] * (MUSIC_OFF - PHRASE_OFF))
    music = music_table if music_table is not None else bytes([0x00] * (CART_SIZE - MUSIC_OFF))

    phrase_len = min(len(phrases), MUSIC_OFF - PHRASE_OFF)
    music_len = min(len(music), CART_SIZE - MUSIC_OFF)
    img[PHRASE_OFF : PHRASE_OFF + phrase_len] = phrases[:phrase_len]
    img[MUSIC_OFF : MUSIC_OFF + music_len] = music[:music_len]

    return bytes(img)


def compile_source(
    text: str,
    *,
    copyright_key: str = "ultramaxx",
    phrase_table: bytes | None = None,
    music_table: bytes | None = None,
) -> bytes:
    program = parse_source(text)
    return emit_cart(
        program,
        copyright=COPYRIGHTS[copyright_key],
        phrase_table=phrase_table,
        music_table=music_table,
    )


def format_listing(program: list[Instruction]) -> str:
    lines = ["; MaxxBAS compiled program"]
    for insn in program:
        lines.append(f"; line {insn.source_line}: {insn.text}")
        lines.append(f"{insn.opcode:02X} {insn.operand:02X}")
    return "\n".join(lines)


def cmd_compile(args: argparse.Namespace) -> int:
    src_path = resolve_from_root(args.source, must_exist=True)
    text = src_path.read_text()
    try:
        image = compile_source(
            text,
            copyright_key=args.copyright,
        )
    except CompileError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1

    out_path = resolve_from_root(args.output) if args.output else src_path.with_suffix(".532")
    out_path.write_bytes(image)

    cart = CartImage(data=image)
    issues = validate_cart(cart)
    if issues:
        for issue in issues:
            print(f"WARN: {issue}", file=sys.stderr)
        return 1

    print(f"wrote {out_path} ({len(image)} bytes)")
    if args.listing:
        program = parse_source(text)
        print(format_listing(program))
    return 0


def cmd_check(args: argparse.Namespace) -> int:
    src_path = resolve_from_root(args.source, must_exist=True)
    try:
        program = parse_source(src_path.read_text())
        program_bytes(program)
    except CompileError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    pairs = sum(1 for _ in program)
    print(f"OK: {pairs} instructions ({pairs * 2} bytes)")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_compile = sub.add_parser("compile", help="compile .bas/.maxx source to .532")
    p_compile.add_argument("source", help="MaxxBAS source file")
    p_compile.add_argument("-o", "--output", help="output .532 path (default: same basename)")
    p_compile.add_argument(
        "--copyright",
        choices=sorted(COPYRIGHTS),
        default="ultramaxx",
        help="17-byte copyright string (default: ultramaxx)",
    )
    p_compile.add_argument("--listing", action="store_true", help="print bytecode listing to stdout")

    p_check = sub.add_parser("check", help="parse source without writing output")
    p_check.add_argument("source", help="MaxxBAS source file")

    args = parser.parse_args(argv)
    if args.cmd == "compile":
        return cmd_compile(args)
    if args.cmd == "check":
        return cmd_check(args)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())