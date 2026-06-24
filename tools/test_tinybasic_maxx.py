#!/usr/bin/env python3
"""Tests for MaxxBAS compiler (tools/tinybasic_maxx.py)."""

from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from maxx_rom import CART_SIZE, CartImage, validate_cart
from project_paths import project_root
from tinybasic_maxx import (
    MAX_PROG_BYTES,
    CompileError,
    compile_source,
    parse_source,
    program_bytes,
)

HELLO_BAS = project_root() / "Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas"


class TestMaxxBasParser(unittest.TestCase):
    def test_minimal_program(self) -> None:
        src = "DELAY 1\nEND\n"
        prog = parse_source(src)
        self.assertEqual(len(prog), 2)
        self.assertEqual(prog[0].opcode, 0x0C)
        self.assertEqual(prog[0].operand, 1)
        self.assertEqual(prog[-1].as_bytes(), b"\xff\xff")

    def test_line_numbers_and_comments(self) -> None:
        src = "10 DELAY 2  REM wait\n# full-line comment\n20 END\n"
        prog = parse_source(src)
        self.assertEqual(len(prog), 2)
        self.assertEqual(prog[0].operand, 2)

    def test_lamp_on_off(self) -> None:
        prog = parse_source("LAMP ON\nLAMP OFF\nEND\n")
        self.assertEqual(prog[0].as_bytes(), b"\x0a\x01")
        self.assertEqual(prog[1].as_bytes(), b"\x0a\x00")

    def test_say_and_speak(self) -> None:
        prog = parse_source("SAY 0\nSPEAK 63\nEND\n")
        self.assertEqual(prog[0].as_bytes(), b"\x83\x00")
        self.assertEqual(prog[1].as_bytes(), b"\x82\x3f")

    def test_implicit_end(self) -> None:
        prog = parse_source("HOME\n")
        self.assertEqual(prog[-1].opcode, 0xFF)

    def test_unknown_statement(self) -> None:
        with self.assertRaises(CompileError):
            parse_source("GOTO 10\nEND\n")

    def test_operand_range(self) -> None:
        with self.assertRaises(CompileError):
            parse_source("DELAY 300\nEND\n")

    def test_program_size_limit(self) -> None:
        lines = [f"DELAY {i % 10}\n" for i in range(MAX_PROG_BYTES // 2)]
        lines.append("END\n")
        with self.assertRaises(CompileError):
            program_bytes(parse_source("".join(lines)))


class TestMaxxBasEmit(unittest.TestCase):
    def test_hello_bas_compiles_and_validates(self) -> None:
        self.assertTrue(HELLO_BAS.is_file(), f"missing {HELLO_BAS}")
        image = compile_source(HELLO_BAS.read_text())
        self.assertEqual(len(image), CART_SIZE)
        cart = CartImage(data=image)
        issues = validate_cart(cart)
        self.assertEqual(issues, [], f"validation failed: {issues}")

    def test_round_trip_write(self) -> None:
        src = HELLO_BAS.read_text()
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp) / "hello.532"
            out.write_bytes(compile_source(src))
            cart = CartImage.load(out)
            self.assertEqual(validate_cart(cart), [])


if __name__ == "__main__":
    unittest.main()