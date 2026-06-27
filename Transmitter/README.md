# Transmitter

27 MHz remote control — COP411L MCU, OOK RF link.

| Path | Description |
|------|-------------|
| [`KiCAD/`](KiCAD/) | KiCad 10 schematic/PCB — open [`Transmitter-27MHz.kicad_pro`](KiCAD/Transmitter-27MHz.kicad_pro) |
| [`ReverseEngineering/`](ReverseEngineering/) | Protocol and hardware notes ([`Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf`](ReverseEngineering/Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf), ODT/DOCX sources) |
| [`Stickers/`](Stickers/) | Remote sticker artwork |
| [`Photos/`](Photos/) | Product shots and reverse-engineering images |
| [`transmitter-architecture.md`](transmitter-architecture.md) | 455 kHz IF clock, OOK protocol, signal path |
| [`transmitter-bom.md`](transmitter-bom.md) | Bill of materials |
| [`remote-keypad.md`](remote-keypad.md) | Faceplate button names and matrix map (A–Y) |
| [`tools/rfcap/`](../tools/rfcap/) | GNU Radio flowgraphs and 27 MHz OOK IQ captures |
| [GNU Radio OOK capture demo](https://www.instagram.com/p/CLOjig8nCJS/) | Screen recording of `RemoteSpectrum` plotting RF data |

Shared symbols: [`libraries/`](../libraries/).

## Datasheets

Third-party component PDFs live in [`DataSheets/`](../DataSheets/) (repository-wide index). Parts used on this module:

| File | Part |
|------|------|
| [`National-COP411L.pdf`](../DataSheets/National-COP411L.pdf) | National COP411L 4-bit MCU (U1) |

## TODO

Status: **Active** — KiCad repair/fab; empty schematic/PCB/3D folders. Full backlog: [`TODO.md`](../TODO.md#transmitter). GitHub: [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Atransmitter+label%3Abacklog).

**KiCad / RE (in progress)**

- [ ] Run ERC; resolve outstanding errors/warnings
- [ ] Confirm COP411L-PAC/N **DIP-20** footprint matches physical package
- [ ] Sync schematic ↔ PCB (export netlist from `.kicad_sch`)
- [ ] Run DRC on layout
- [ ] Export Gerbers / drill files for fab review
- [ ] Align [`transmitter-bom.md`](transmitter-bom.md) with final schematic refs

**Missing artifact folders** `[empty]`

- [ ] [`Schematic/`](Schematic/) — dedicated schematic scan or vector export (separate from KiCad)
- [ ] [`PCBoard/`](PCBoard/) — PCB photos, fab drawings, or layout exports
- [ ] [`Model3D/`](Model3D/) — enclosure / remote mechanical CAD
- [x] [`Photos/`](Photos/) — product and reverse-engineering images