# Maxx Steele Technical Manual

This manual describes how to program the 1984 CBS Toys / Ideal **Maxx Steele** robot at the software level: bytecode motion programs, speech, music, cartridge images, and the 6502 internal ROM that interprets them.

It is modeled on the structure of the [Commodore 64 Programmer's Reference Guide](https://archive.org/details/c64-programmer-ref) — numbered chapters, appendices, a quick-reference card, and a schematic index — adapted to Maxx Steele hardware.

**PDF (latest):** [`Maxx-Steele-Technical-Manual.pdf`](Maxx-Steele-Technical-Manual.pdf) — front/rear covers plus chapters; rebuilt when `.md` or cover images ([`cover-front.jpg`](cover-front.jpg), [`cover-rear.jpg`](cover-rear.jpg)) change ([`tools/build_technical_manual_pdf.py`](../tools/build_technical_manual_pdf.py) locally; GitHub Actions on push to `main`).

## What this guide covers

| Document | Audience | Focus |
|----------|----------|-------|
| **This guide** | Programmers, reverse engineers | Bytecode language, memory map, I/O, internal ROM hooks |
| [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) | Cartridge authors | Step-by-step cart layout, tools, demo walkthrough |
| [`Chassis/Manual/`](../Chassis/Manual/) | Owners | Factory user / reference manuals (operation, not ROM layout) |
| [`MechanicalManual/`](../MechanicalManual/) | Repairers | Chassis disassembly, reassembly, teardown photos |

Primary sources: R. Wind disassemblies ([`maxx_internal_ROM.dsm`](../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm), [`maxx_demo_ROM_532.dsm`](../Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm)), [`tools/maxx_rom.py`](../tools/maxx_rom.py).

---

## How to use this reference guide

**If you are new to Maxx programming**, read in order:

1. [Chapter 1 — Bytecode programming rules](01-Bytecode-Programming-Rules.md)
2. [Chapter 2 — Opcode vocabulary](02-Opcode-Vocabulary.md)
3. [Chapter 3 — Motion and display](03-Programming-Motion-and-Display.md)
4. Work through the demo program in [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) §5

**If you are writing cartridges**, add:

5. [Chapter 5 — Internal ROM operating system](05-Cartridge-Bootstrap-and-Internal-ROM.md)
6. [Chapter 7 — UltraMaxx BASIC language](07-UltraMaxx-BASIC-Language.md)
7. [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) §9 (authoring workflow)

**If you are reverse engineering hardware**, use:

- [Chapter 6 — Input/output guide](06-Input-Output-Guide.md)
- [Schematics](Schematics.md)
- Annotated `.dsm` listings under [`Mainboard/Firmware/`](../Mainboard/Firmware/) and [`Cartridge/Examples/`](../Cartridge/Examples/)

Keep [Quick reference](Quick-Reference.md) and [Appendices](Appendices.md) open while coding.

---

## Applications guide

| Application | Mode (`$0D`) | What you need |
|-------------|--------------|---------------|
| Remote control (immediate) | 0 | Remote keypad; opcodes executed live |
| Learn key sequences | 1 | Records into program RAM |
| Enter program steps | 2 | Keypad maps to opcodes via `$E6B5` table |
| Run stored program | 3 | Bytecode at `$0200`, terminator `FF FF` |
| Built-in games | 4 | `JMP ($0094)` → `$F8CE`; game 1/2 at `$F8E9` / `$FAA4` |
| Factory demo | 3 + cart | [`CBSDemo.532`](../Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532) bootstrap |
| Custom cartridge | 3 + cart | 4 KB EPROM image; see Ch 5–7 + cartridge manual |
| UltraMaxx BASIC authoring | 3 + cart | Ch 7; [`tools/maxx`](../tools/maxx) `compile` → `.532` |
| Speech / music authoring | any | Ch 4; phrase tables in cart or RAM |
| Repair / RE | — | Ch 6, Schematics, [`DataSheets/`](../DataSheets/) |

---

## Firmware images in this archive

| Image | Path |
|-------|------|
| Internal 8 KB ROM | [`Mainboard/Firmware/Binary/Maxxrom.64`](../Mainboard/Firmware/Binary/Maxxrom.64) |
| Demo cartridge (4 KB) | [`Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532`](../Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532) |
| UltraMaxx cartridge (4 KB) | [`Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532`](../Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532) |

Tools: [`tools/maxx`](../tools/maxx) — compile UltraMaxx BASIC, validate, list, upload; [`tools/maxx_rom.py`](../tools/maxx_rom.py) — disassemble, template, opcode export.