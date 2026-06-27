# Maxx Steele User Manual

Community edition of the 1984 CBS Toys / Ideal **Electronic Maxx Steele® Personal Robot** factory *User's Guide to Operations*, originally bootstrapped from [`Chassis/Manual/MaxxSteeleManual.pdf`](../../Chassis/Manual/MaxxSteeleManual.pdf).

**PDF (latest):** [`Maxx-Steele-User-Manual.pdf`](Maxx-Steele-User-Manual.pdf) — rebuilt when `.md` or cover images change ([`tools/build_user_manual_pdf.py`](../../tools/build_user_manual_pdf.py) locally; GitHub Actions on push to `main`).

## What this guide covers

| Document | Audience | Focus |
|----------|----------|-------|
| **This guide** | Owners | Setup, remote control, five operating modes, games, maintenance |
| [`Docs/Technical/`](../Technical/) | Programmers | Bytecode, ROM, I/O, cartridge authoring |
| [`Docs/Mechanical/`](../Mechanical/) | Repairers | Disassembly, reassembly, chassis photos |
| [`Chassis/Manual/MaxxSteeleReferenceGuide.pdf`](../../Chassis/Manual/MaxxSteeleReferenceGuide.pdf) | Owners | Six-page factory programmer quick reference |

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

## Editing and building

**Source of truth:** the chapter `.md` files in this folder. Edit them directly, then rebuild the PDF:

```bash
python3 tools/build_user_manual_pdf.py
```

GitHub Actions runs the same build on push when `Docs/User/**` changes.

## Bootstrap from factory PDF (optional)

To re-seed chapter markdown from the archival factory PDF (overwrites chapter text from OCR):

```bash
python3 tools/bootstrap_user_manual_from_pdf.py
```

Or step-by-step: [`tools/extract_manual_pages.py`](../../tools/extract_manual_pages.py) → [`tools/ocr_manual_pages.py`](../../tools/ocr_manual_pages.py) → [`tools/gen_user_manual_chapters.py`](../../tools/gen_user_manual_chapters.py) `--from-sources`.

| Path | Description |
|------|-------------|
| [`Sources/pages/`](Sources/pages/) | Page scans (figures + OCR input; optional bootstrap) |
| [`Sources/text/`](Sources/text/) | Per-page OCR text (bootstrap input only) |
| [`Sources/outline.json`](Sources/outline.json) | Page-to-chapter map for the generator |

For keypad matrix names and faceplate art, see [`Transmitter/remote-keypad.md`](../../Transmitter/remote-keypad.md).