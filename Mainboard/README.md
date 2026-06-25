# Mainboard

6502 main logic board for the Maxx Steele robot.

| Path | Description |
|------|-------------|
| [`Firmware/`](Firmware/) | Masked internal 6502 ROM binary and disassembly (`$E000`–`$FFFF`) |
| [`KiCAD/`](KiCAD/) | KiCad project (placeholder) |
| [`Schematic/`](Schematic/) | Raster scans, [IC inventory](Schematic/IC-Inventory.md), [MMIO pin map](Schematic/MMIO-Pin-Map.md) |

Chassis photos: [`Chassis/Photos/`](../Chassis/Photos/).

Shared KiCad symbols: [`libraries/`](../libraries/).

## TODO

Status: **Partial** — raster schematics; no KiCad project. Full backlog: [`TODO.md`](../TODO.md#mainboard). GitHub: [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Amainboard+label%3Abacklog).

- [ ] Digitize [`Schematic/`](Schematic/) raster/SVG into KiCad — [`KiCAD/`](KiCAD/) is `[planned]` (README + `sym-lib-table` only)
- [ ] PCB layout project and fabrication artifacts
- [ ] Module-specific photos (robot photos: [`Chassis/Photos/`](../Chassis/Photos/))