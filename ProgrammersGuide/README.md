# Maxx Steele Programmer's Reference Guide

This guide describes how to program the 1984 CBS Toys / Ideal **Maxx Steele** robot at the software level: bytecode motion programs, speech, music, cartridge images, and the 6502 internal ROM that interprets them.

It is modeled on the structure of the [Commodore 64 Programmer's Reference Guide](https://archive.org/details/c64-programmer-ref) — numbered chapters, appendices, a quick-reference card, and a schematic index — adapted to Maxx Steele hardware.

**PDF (latest):** [`Maxx-Steele-Programmers-Reference.pdf`](Maxx-Steele-Programmers-Reference.pdf) — rebuilt automatically when chapter `.md` files change (`python3 tools/build_programmers_guide_pdf.py` locally; GitHub Actions on push to `main`).

## What this guide covers

| Document | Audience | Focus |
|----------|----------|-------|
| **This guide** | Programmers, reverse engineers | Bytecode language, memory map, I/O, internal ROM hooks |
| [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) | Cartridge authors | Step-by-step cart layout, tools, demo walkthrough |
| [`Chassis/Manual/`](../Chassis/Manual/) | Owners | Factory user / reference manuals (operation, not ROM layout) |

Primary sources: R. Wind disassemblies ([`maxx_internal_ROM.dsm`](../Chassis/Firmware/Assembly/maxx_internal_ROM.dsm), [`maxx_demo_ROM_532.dsm`](../Cartridge/Firmware/Assembly/maxx_demo_ROM_532.dsm)), [`tools/maxx_rom.py`](../tools/maxx_rom.py).

---

## How to use this reference guide

**If you are new to Maxx programming**, read in order:

1. [Chapter 1 — Bytecode programming rules](01-Bytecode-Programming-Rules.md)
2. [Chapter 2 — Opcode vocabulary](02-Opcode-Vocabulary.md)
3. [Chapter 3 — Motion and display](03-Programming-Motion-and-Display.md)
4. Work through the demo program in [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) §5

**If you are writing cartridges**, add:

5. [Chapter 5 — Cartridge bootstrap and internal ROM](05-Cartridge-Bootstrap-and-Internal-ROM.md)
6. [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) §9 (authoring workflow)

**If you are reverse engineering hardware**, use:

- [Chapter 6 — Input/output guide](06-Input-Output-Guide.md)
- [Schematics](Schematics.md)
- Annotated `.dsm` listings under [`Chassis/Firmware/`](../Chassis/Firmware/) and [`Cartridge/Firmware/`](../Cartridge/Firmware/)

Keep [Quick reference](Quick-Reference.md) and [Appendices](Appendices.md) open while coding.

---

## Applications guide

| Application | Mode (`$0D`) | What you need |
|-------------|--------------|---------------|
| Remote control (immediate) | 0 | Remote keypad; opcodes executed live |
| Learn key sequences | 1 | Records into program RAM |
| Enter program steps | 2 | Keypad maps to opcodes via `$E6B5` table |
| Run stored program | 3 | Bytecode at `$0200`, terminator `FF FF` |
| Factory demo | 3 + cart | [`MAXXCART.532`](../Cartridge/Firmware/Binary/MAXXCART.532) bootstrap |
| Custom cartridge | 3 + cart | 4 KB EPROM image; see Ch 5 + cartridge manual |
| Speech / music authoring | any | Ch 4; phrase tables in cart or RAM |
| Repair / RE | — | Ch 6, Schematics, [`DataSheets/`](../DataSheets/) |

---

## Table of contents

| Chapter | Title |
|---------|-------|
| — | [Introduction](README.md) (this file) |
| 1 | [Bytecode programming rules](01-Bytecode-Programming-Rules.md) |
| 2 | [Opcode vocabulary](02-Opcode-Vocabulary.md) |
| 3 | [Programming motion and display](03-Programming-Motion-and-Display.md) |
| 4 | [Programming speech and music](04-Programming-Speech-and-Music.md) |
| 5 | [Cartridge bootstrap and internal ROM](05-Cartridge-Bootstrap-and-Internal-ROM.md) |
| 6 | [Input/output guide](06-Input-Output-Guide.md) |
| A–L | [Appendices](Appendices.md) |
| — | [Quick reference card](Quick-Reference.md) |
| — | [Schematic diagram index](Schematics.md) |

---

## Firmware images in this archive

| Image | Path |
|-------|------|
| Internal 8 KB ROM | [`Chassis/Firmware/Binary/Maxxrom.64`](../Chassis/Firmware/Binary/Maxxrom.64) |
| Demo cartridge (4 KB) | [`Cartridge/Firmware/Binary/MAXXCART.532`](../Cartridge/Firmware/Binary/MAXXCART.532) |

Tools: [`tools/maxx_rom.py`](../tools/maxx_rom.py) — disassemble, validate, template, opcode export.