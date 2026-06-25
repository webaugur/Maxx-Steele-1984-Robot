# Maxx Steele Mechanical Manual

This manual describes how to **disassemble and reassemble** the 1984 CBS Toys / Ideal **Maxx Steele** robot chassis for repair, inspection, and maintenance.

It is a community edition of a workshop guide originally written by **Mike McCarty** (preserved in [`Sources/Maxx-Steele-Disassembly-Guide.docx`](Sources/Maxx-Steele-Disassembly-Guide.docx)).

**PDF (latest):** [`Maxx-Steele-Mechanical-Manual.pdf`](Maxx-Steele-Mechanical-Manual.pdf) — rebuilt when `.md` or cover images change ([`tools/build_mechanical_manual_pdf.py`](../tools/build_mechanical_manual_pdf.py) locally; GitHub Actions on push to `main`).

## What this guide covers

| Document | Audience | Focus |
|----------|----------|-------|
| **This guide** | Owners, repairers | Case opening, head module, stabilizers, reassembly |
| [`TechnicalManual/`](../TechnicalManual/) | Programmers, reverse engineers | Bytecode, ROM, I/O, cartridge authoring |
| [`UserManual/`](../UserManual/) | Owners | Setup, operation, games, maintenance (markdown + PDF) |
| [`Chassis/Manual/`](../Chassis/Manual/) | Owners | Archival factory PDF scans |
| [`Chassis/Photos/`](../Chassis/Photos/) | Reference | All chassis photos (embedded in Ch 3 by folder) |
| [`Chassis/Artwork/`](../Chassis/Artwork/) | Reference | Body logos and graphics (listed in Ch 3) |

## Before you start

- **Power off** and **remove the battery** before opening the chassis.
- Work on a soft surface; lay Maxx **face down** when removing the back shell.
- **Do not strain** the wires between the back shell and the body (remote holder, display, RF).
- Keep screws with their hardware (stabilizers, case, head plate).

Module-level PCB work is documented under [`Mainboard/`](../Mainboard/), [`Face/`](../Face/), [`Receiver/`](../Receiver/), [`Power/`](../Power/), and [`Transmitter/`](../Transmitter/).

## How to use this manual

1. [Chapter 1 — Disassembly](01-Disassembly.md)
2. [Chapter 2 — Reassembly](02-Reassembly.md)
3. [Chapter 3 — Chassis photo reference](03-Chassis-Photos.md) — all [`Chassis/Photos/`](../Chassis/Photos/) and [`Chassis/Artwork/`](../Chassis/Artwork/) images by folder

Regenerate the photo chapter after adding chassis images:

```bash
python3 tools/gen_mechanical_chassis_photos.py
```

Hardware media index (photos, sounds, firmware, schematics): [`Hardware-Assets.csv`](Hardware-Assets.csv). Regenerate with `python3 tools/gen_hardware_assets_csv.py`. Manual classifications are under human review ([#42](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/42)).

For programming and electrical reference after reassembly, see [`TechnicalManual/README.md`](../TechnicalManual/README.md).