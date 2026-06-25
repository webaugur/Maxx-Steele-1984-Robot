#!/usr/bin/env python3
"""Tests for manual PDF markdown preprocessing."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from manual_pdf import preprocess_markdown  # noqa: E402


class PreprocessMarkdownTests(unittest.TestCase):
    def test_technical_manual_basename_link(self) -> None:
        manual_dir = Path(__file__).resolve().parents[1] / "TechnicalManual"
        text = (
            "Matrix wiring refs: "
            "[`keyboard-matrix-reference-1.png`]"
            "(../Transmitter/Photos/ReverseEngineering/keyboard-matrix-reference-1.png)."
        )
        out = preprocess_markdown(text, {}, manual_dir=manual_dir)
        self.assertNotIn("`keyboard-matrix-reference-1.png` (`Transmitter/", out)
        self.assertIn("![keyboard-matrix-reference-1.png]", out)

    def test_technical_manual_cover_stays_textual(self) -> None:
        manual_dir = Path(__file__).resolve().parents[1] / "TechnicalManual"
        text = "Covers: [`cover-front.jpg`](cover-front.jpg)."
        out = preprocess_markdown(text, {}, manual_dir=manual_dir)
        self.assertNotIn("![cover-front.jpg]", out)
        self.assertIn("`cover-front.jpg`", out)

    def test_mechanical_manual_embeds_chassis_photo(self) -> None:
        manual_dir = Path(__file__).resolve().parents[1] / "MechanicalManual"
        text = "![IMG_2116](../Chassis/Photos/Disassembly/IMG_2116.JPG)"
        out = preprocess_markdown(text, {}, manual_dir=manual_dir)
        self.assertIn("![IMG_2116]", out)
        self.assertIn("Chassis/Photos/Disassembly/IMG_2116.JPG", out)

    def test_mechanical_manual_repo_link_no_suffix(self) -> None:
        manual_dir = Path(__file__).resolve().parents[1] / "MechanicalManual"
        text = "See [`Mainboard/`](../Mainboard/) for PCB work."
        out = preprocess_markdown(text, {}, manual_dir=manual_dir)
        self.assertIn("`Mainboard/`", out)
        self.assertNotIn("`Mainboard/` (`Mainboard/`)", out)


if __name__ == "__main__":
    unittest.main()