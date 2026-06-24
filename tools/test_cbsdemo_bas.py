#!/usr/bin/env python3
"""Verify CBS Demo MaxxBAS translation matches factory program bytes."""

from __future__ import annotations

import unittest
from pathlib import Path

from maxx_rom import CART_SIZE
from project_paths import project_root
from tinybasic_maxx import PROG_OFF, PHRASE_OFF, compile_source, load_tables_from_reference

CBSDEMO_BAS = project_root() / "Cartridge/Examples/CBSDemo/Firmware/Basic/cbsdemo.bas"
CBSDEMO_532 = project_root() / "Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532"


class TestCbsDemoBas(unittest.TestCase):
    def test_program_table_matches_factory_rom(self) -> None:
        self.assertTrue(CBSDEMO_BAS.is_file())
        self.assertTrue(CBSDEMO_532.is_file())
        src = CBSDEMO_BAS.read_text()
        phrase, music = load_tables_from_reference(CBSDEMO_532)
        image = compile_source(
            src,
            copyright_key="cbs",
            phrase_table=phrase,
            music_table=music,
        )
        factory = CBSDEMO_532.read_bytes()
        self.assertEqual(image[PROG_OFF:PHRASE_OFF], factory[PROG_OFF:PHRASE_OFF])

    def test_full_image_matches_with_factory_tables(self) -> None:
        src = CBSDEMO_BAS.read_text()
        phrase, music = load_tables_from_reference(CBSDEMO_532)
        image = compile_source(
            src,
            copyright_key="cbs",
            phrase_table=phrase,
            music_table=music,
        )
        factory = CBSDEMO_532.read_bytes()
        self.assertEqual(len(image), CART_SIZE)
        self.assertEqual(image, factory)


if __name__ == "__main__":
    unittest.main()