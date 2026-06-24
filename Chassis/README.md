# Chassis

Robot body, packaging, manuals, and material not tied to a single PCB module.

| Path | Description |
|------|-------------|
| [`Manual/`](Manual/) | Original owner and reference manuals |
| [`Photos/`](Photos/) | Exterior, teardown, workshop, and collection photos |
| [`Artwork/`](Artwork/) | Logos and body artwork |
| [`References/`](References/) | Third-party articles and workshop notes |
| [`Disassembly/`](Disassembly/) | Disassembly guide and photos |
| [`Sounds/`](Sounds/) | Sample speech and song audio |
| [`Firmware/`](Firmware/) | Internal 6502 ROM binary and disassembly |
| [`Model3D/`](Model3D/) | Mechanical CAD (placeholder) |
| [`KiCAD/`](KiCAD/) | Mechanical / enclosure CAD (placeholder) |

Module-specific hardware lives in [`Transmitter/`](../Transmitter/), [`Receiver/`](../Receiver/), [`Mainboard/`](../Mainboard/), [`Face/`](../Face/), [`Power/`](../Power/), [`Cartridge/`](../Cartridge/), and [`PaddleMirror/`](../PaddleMirror/).

## Datasheets

Third-party component PDFs live in [`DataSheets/`](../DataSheets/) (repository-wide index). Parts referenced from chassis / internal ROM work:

| File | Part |
|------|------|
| [`MOS-6502.pdf`](../DataSheets/MOS-6502.pdf) | MOS 6502 CPU |
| [`National-COP420.pdf`](../DataSheets/National-COP420.pdf) | National COP420 family MCU |

## TODO

Status: **Mostly complete** — no 3D body model or mechanical KiCad. Full backlog: [`TODO.md`](../TODO.md#chassis).

**Missing** `[empty]`

- [ ] [`Model3D/`](Model3D/) — full robot body mechanical CAD
- [ ] [`KiCAD/`](KiCAD/) — enclosure / mechanism drawings (README placeholder only)