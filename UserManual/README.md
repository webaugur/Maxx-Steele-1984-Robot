# Maxx Steele User Manual

Community edition of the 1984 CBS Toys / Ideal **Electronic Maxx Steele® Personal Robot** factory *User's Guide to Operations*, transcribed from [`Chassis/Manual/MaxxSteeleManual.pdf`](../Chassis/Manual/MaxxSteeleManual.pdf).

**PDF (latest):** [`Maxx-Steele-User-Manual.pdf`](Maxx-Steele-User-Manual.pdf) — rebuilt when `.md` or cover images change ([`tools/build_user_manual_pdf.py`](../tools/build_user_manual_pdf.py) locally; GitHub Actions on push to `main`).

## What this guide covers

| Document | Audience | Focus |
|----------|----------|-------|
| **This guide** | Owners | Setup, remote control, five operating modes, games, maintenance |
| [`TechnicalManual/`](../TechnicalManual/) | Programmers | Bytecode, ROM, I/O, cartridge authoring |
| [`MechanicalManual/`](../MechanicalManual/) | Repairers | Disassembly, reassembly, chassis photos |
| [`Chassis/Manual/MaxxSteeleReferenceGuide.pdf`](../Chassis/Manual/MaxxSteeleReferenceGuide.pdf) | Owners | Six-page factory programmer quick reference |

## How to use this manual

1. [Getting started](01-Getting-Started.md)
2. [Setup](02-Setup.md)
3. [Starting Maxx](03-Operations.md)
4. [Immediate mode](04-Immediate-Mode.md)
5. [Program, learn, and execute modes](05-Program-Learn-Execute.md)
6. [Games and other operating modes](06-Games-And-Other-Modes.md)
7. [Advanced features](07-Advanced-Features.md)
8. [Appendices A–J](08-Appendices.md)
9. [Cautions, FCC, and stabilizers](09-Compliance.md)

## Source materials

| Path | Description |
|------|-------------|
| [`Sources/pages/`](Sources/pages/) | 300 DPI page scans extracted from the factory PDF |
| [`Sources/text/`](Sources/text/) | Tesseract OCR per page (used to bootstrap chapter text) |
| [`Sources/outline.json`](Sources/outline.json) | Page-to-chapter map |

Regenerate chapter markdown after updating scans or OCR:

```bash
python3 tools/extract_manual_pages.py
python3 tools/ocr_manual_pages.py
python3 tools/gen_user_manual_chapters.py
python3 tools/build_user_manual_pdf.py
```

For keypad matrix names and faceplate art, see [`Transmitter/remote-keypad.md`](../Transmitter/remote-keypad.md).
