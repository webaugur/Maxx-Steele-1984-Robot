# DataSheets

Third-party component datasheets for the Maxx Steele robot. **Overrides** the usual folder naming rules: this top-level folder is always `DataSheets/` (not under individual modules).

Do not duplicate datasheet PDFs elsewhere in the archive — link here from module READMEs and docs.

**Filenames** use `OEM-PartNumber` only (no board refdes prefixes). Shell-safe: `[A-Za-z0-9._-]`.

## Index

| File | OEM / part | Refdes | Schematic | Used by |
|------|------------|--------|-----------|---------|
| [`National-COP411L.pdf`](National-COP411L.pdf) | National COP411L 4-bit MCU | U1 | [`Transmitter-27MHz.sch`](../Transmitter/KiCAD/Transmitter-27MHz.sch) | [`Transmitter/`](../Transmitter/), [`transmitter-bom.md`](../Transmitter/transmitter-bom.md) |
| [`National-COP420.pdf`](National-COP420.pdf) | National COP420 family MCU | | — | [`Mainboard/`](../Mainboard/), [`Chassis/`](../Chassis/) |
| [`National-COPS-Programming-Manual-Feb85.pdf`](National-COPS-Programming-Manual-Feb85.pdf) | National COPS programming manual | | — | COP400 family |
| [`MOS-6502.pdf`](MOS-6502.pdf) | MOS 6502 CPU | | — | [`Mainboard/`](../Mainboard/), [`Chassis/Firmware/`](../Chassis/Firmware/) |
| [`Maxx-Steele-CPU-Pinout.pdf`](Maxx-Steele-CPU-Pinout.pdf) | Robot CPU pinout (PDF, project doc) | | — | [`Mainboard/`](../Mainboard/) |
| [`Maxx-Steele-CPU-Pinout.xlsx`](Maxx-Steele-CPU-Pinout.xlsx) | Robot CPU pinout (spreadsheet, project doc) | | — | [`Mainboard/`](../Mainboard/) |
| [`Mitsubishi-KM2365.pdf`](Mitsubishi-KM2365.pdf) | Mitsubishi KM2365 EPROM (cartridge ROM) | U400 | — | [`Cartridge/`](../Cartridge/), [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) |
| [`Mitsubishi-M6116.pdf`](Mitsubishi-M6116.pdf) | Mitsubishi M6116 static RAM | U401 | — | [`Mainboard/`](../Mainboard/) |
| [`National-COP41xL-Display-Motors.pdf`](National-COP41xL-Display-Motors.pdf) | National COP41xL display & motor driver | U500 | — | [`Face/`](../Face/), [`Mainboard/`](../Mainboard/) |
| [`Eurotechnique-ET9420.pdf`](Eurotechnique-ET9420.pdf) | Eurotechnique ET9420 speech synthesizer | | — | [`Face/`](../Face/) |
| [`Thompson-ET9420-Options.pdf`](Thompson-ET9420-Options.pdf) | Thompson ET9420 options / application note | | — | [`Face/`](../Face/) |
| [`Sanyo-LC3100.pdf`](Sanyo-LC3100.pdf) | Sanyo LC3100 speech chip | | — | [`Face/`](../Face/) |
| [`Sanyo-LC3100-2.pdf`](Sanyo-LC3100-2.pdf) | Sanyo LC3100 (alternate scan) | | — | [`Face/`](../Face/) |
| [`Sanyo-LC8100.pdf`](Sanyo-LC8100.pdf) | Sanyo LC8100 speech chip | | — | [`Face/`](../Face/) |
| [`Sanyo-LC81196.pdf`](Sanyo-LC81196.pdf) | Sanyo LC81196 / LC8100 family | | — | [`Face/`](../Face/) |

**Refdes** — schematic designator when known from a KiCad `.sch` or the Yahoo Groups archive filenames (`U400`, `U401`, `U500`). Blank when not yet traced on a schematic.

**Schematic** — hyperlink to the KiCad `.sch` that references the part. `—` until the mainboard/face/cartridge schematics are digitized (raster sources: [`Mainboard/Schematic/`](../Mainboard/Schematic/)).

## Adding datasheets

1. Place new third-party PDFs here using `OEM-PartNumber.pdf` (shell-safe — see [naming conventions](../README.md#naming-conventions)).
2. Add a row to the index table with **Refdes** and **Schematic** columns when known.
3. Add a **Datasheets** section to the relevant module README; do not duplicate PDFs under modules.

---

<small>

**Archival notice.** These materials are preserved to support repair, restoration, reverse engineering, and lawful operation of legacy Maxx Steele hardware and its obsolete or difficult-to-obtain components. They are archived on the understanding that documenting how a product you own works — for interoperability, maintenance, and non-commercial study — may qualify as fair use under U.S. copyright law (17 U.S.C. § 107) and analogous doctrines elsewhere. Many of the parts documented here are no longer manufactured or readily sourced; access to original manufacturer documentation is often unavailable through normal commercial channels.

This repository does not assert ownership of these documents. If you are a copyright or trademark holder and believe that inclusion of a specific file is not permitted, please contact the maintainer. Upon notice, we will review the concern promptly and, where appropriate, remove or replace the material with an alternative reference (e.g. a different public datasheet, a summary, or a link to an authorized source). Nothing in this notice constitutes legal advice.

</small>