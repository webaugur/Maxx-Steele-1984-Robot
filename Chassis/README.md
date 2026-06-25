# Chassis

Robot body, packaging, manuals, and material not tied to a single PCB module.

| Path | Description |
|------|-------------|
| [`Manual/`](Manual/) | Original owner and reference manuals |
| [`Photos/`](Photos/) | Exterior, teardown, workshop, and collection photos (indexed in [`MechanicalManual/03-Chassis-Photos.md`](../MechanicalManual/03-Chassis-Photos.md)) |
| [`Artwork/`](Artwork/) | Logos and body artwork (listed in [`MechanicalManual/03-Chassis-Photos.md`](../MechanicalManual/03-Chassis-Photos.md)) |
| [`References/`](References/) | Third-party articles and workshop notes |
| [`Photos/Disassembly/`](Photos/Disassembly/) | Teardown workshop photos (`IMG_2116`–`IMG_2131`) |
| [`MechanicalManual/`](../MechanicalManual/) | Disassembly / reassembly manual (text + PDF) |
| [`Sounds/`](Sounds/) | Sample speech and song audio (indexed in [`MechanicalManual/Hardware-Assets.csv`](../MechanicalManual/Hardware-Assets.csv); `maxx-song-N` → `PLAY N`) |
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

Status: **Mostly complete** — no 3D body model or mechanical KiCad. Full backlog: [`TODO.md`](../TODO.md#chassis). GitHub: [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Achassis+label%3Abacklog).

**Missing** `[empty]`

- [ ] [`Model3D/`](Model3D/) — full robot body mechanical CAD
- [ ] [`KiCAD/`](KiCAD/) — enclosure / mechanism drawings (README placeholder only)