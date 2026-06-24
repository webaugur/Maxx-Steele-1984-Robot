# Transmitter

27 MHz remote control — COP411L MCU, OOK RF link.

| Path | Description |
|------|-------------|
| [`KiCAD/`](KiCAD/) | KiCad schematic/PCB — [`Transmitter-27MHz.pro`](KiCAD/Transmitter-27MHz.pro) |
| [`ReverseEngineering/`](ReverseEngineering/) | Protocol and hardware notes |
| [`Stickers/`](Stickers/) | Remote sticker artwork |
| [`Datasheets/`](Datasheets/) | Pointer to [`DataSheets/`](../DataSheets/) (e.g. [`COP411L.pdf`](../DataSheets/COP411L.pdf)) |
| [`Photos/`](Photos/) | Product shots and reverse-engineering images |

Shared symbols: [`libraries/`](../libraries/).  
RE docs: [`docs/transmitter-architecture.md`](../docs/transmitter-architecture.md), [`docs/transmitter-bom.md`](../docs/transmitter-bom.md).

## TODO

Status: **Active** — KiCad repair/fab; empty schematic/PCB/3D folders. Full backlog: [`TODO.md`](../TODO.md#transmitter).

**KiCad / RE (in progress)**

- [ ] Run ERC; resolve outstanding errors/warnings
- [ ] Confirm COP411L-PAC/N **DIP-20** footprint matches physical package
- [ ] Sync schematic ↔ PCB (31 nets per current `.net`)
- [ ] Run DRC on layout
- [ ] Export Gerbers / drill files for fab review
- [ ] Align [`docs/transmitter-bom.md`](../docs/transmitter-bom.md) with final schematic refs

**Missing artifact folders** `[empty]`

- [ ] [`Schematic/`](Schematic/) — dedicated schematic scan or vector export (separate from KiCad)
- [ ] [`PCBoard/`](PCBoard/) — PCB photos, fab drawings, or layout exports
- [ ] [`Model3D/`](Model3D/) — enclosure / remote mechanical CAD
- [x] [`Photos/`](Photos/) — product and reverse-engineering images