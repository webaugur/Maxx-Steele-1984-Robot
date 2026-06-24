#!/usr/bin/env python3
"""Verify UltraMaxx MaxxBAS translation matches community ROM image."""

from __future__ import annotations

import unittest
from pathlib import Path

from maxx_rom import CART_SIZE
from project_paths import project_root
from tinybasic_maxx import PROG_OFF, PHRASE_OFF, compile_source, load_tables_from_reference

ULTRAMAXX_BAS = project_root() / "Cartridge/Examples/UltraMaxx/Firmware/Basic/ultramaxx.bas"
ULTRAMAXX_532 = project_root() / "Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532"
CBSDEMO_532 = project_root() / "Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532"


class TestUltraMaxxBas(unittest.TestCase):
    def test_matches_ultramaxx_rom(self) -> None:
        src = ULTRAMAXX_BAS.read_text()
        phrase, music = load_tables_from_reference(ULTRAMAXX_532)
        image = compile_source(
            src,
            copyright_key="ultramaxx",
            phrase_table=phrase,
            music_table=music,
        )
        factory = ULTRAMAXX_532.read_bytes()
        self.assertEqual(image, factory)

    def test_only_copyright_differs_from_cbs(self) -> None:
        src = ULTRAMAXX_BAS.read_text()
        phrase, music = load_tables_from_reference(ULTRAMAXX_532)
        image = compile_source(
            src,
            copyright_key="ultramaxx",
            phrase_table=phrase,
            music_table=music,
        )
        cbs = CBSDEMO_532.read_bytes()
        self.assertEqual(len(image), CART_SIZE)
        self.assertEqual(image[0:2], cbs[0:2])
        self.assertNotEqual(image[2:19], cbs[2:19])
        self.assertEqual(image[19:], cbs[19:])


if __name__ == "__main__":
    unittest.main()