#!/usr/bin/env python3
"""Tests for technical manual PDF markdown preprocessing."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from build_technical_manual_pdf import preprocess_markdown  # noqa: E402


class PreprocessMarkdownTests(unittest.TestCase):
    def setUp(self) -> None:
        self.manual_dir = Path(__file__).resolve().parents[1] / "TechnicalManual"

    def test_basename_repo_link_drops_long_path_suffix(self) -> None:
        text = (
            "Matrix wiring refs: "
            "[`keyboard-matrix-reference-1.png`]"
            "(../Transmitter/Photos/ReverseEngineering/keyboard-matrix-reference-1.png)."
        )
        out = preprocess_markdown(text, {}, manual_dir=self.manual_dir)
        self.assertNotIn("`keyboard-matrix-reference-1.png` (`Transmitter/", out)
        self.assertIn("![keyboard-matrix-reference-1.png]", out)

    def test_cover_images_stay_textual(self) -> None:
        text = "Covers: [`cover-front.jpg`](cover-front.jpg)."
        out = preprocess_markdown(text, {}, manual_dir=self.manual_dir)
        self.assertNotIn("![cover-front.jpg]", out)
        self.assertIn("`cover-front.jpg`", out)

    def test_large_table_image_stays_textual(self) -> None:
        text = (
            "| Raster | "
            "[`Mainboard/Schematic/Maxx_Steele_Schematic_enh-v1.1.png`]"
            "(../Mainboard/Schematic/Maxx_Steele_Schematic_enh-v1.1.png) |"
        )
        out = preprocess_markdown(text, {}, manual_dir=self.manual_dir)
        self.assertNotIn("![Mainboard/Schematic", out)
        self.assertIn("`Mainboard/Schematic/Maxx_Steele_Schematic_enh-v1.1.png`", out)


if __name__ == "__main__":
    unittest.main()