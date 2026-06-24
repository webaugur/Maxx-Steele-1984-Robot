# DataSheets

Third-party component datasheets for the Maxx Steele robot. **Overrides** the usual folder naming rules: this top-level folder is always `DataSheets/` (not under individual modules).

Do not duplicate datasheet PDFs elsewhere in the archive — link here from module READMEs and docs.

## Index

| File | Component / topic | Used by |
|------|-------------------|---------|
| [`COP411L.pdf`](COP411L.pdf) | National COP411L 4-bit MCU | [`Transmitter/`](../Transmitter/), [`docs/transmitter-bom.md`](../docs/transmitter-bom.md) |
| [`COP420-Microcontroller.pdf`](COP420-Microcontroller.pdf) | COP420 family MCU | [`Mainboard/`](../Mainboard/), [`Chassis/`](../Chassis/) |
| [`424410284-001A_COPSProgrammingManual_Feb85.pdf`](424410284-001A_COPSProgrammingManual_Feb85.pdf) | National COPS programming manual | COP400 family |
| [`NCR-65xx-Datasheet.pdf`](NCR-65xx-Datasheet.pdf) | MOS 6502 CPU | [`Mainboard/`](../Mainboard/), [`Chassis/Firmware/`](../Chassis/Firmware/) |
| [`Maxx-Steele-CPU-Pinout.pdf`](Maxx-Steele-CPU-Pinout.pdf) | Robot CPU pinout (PDF) | [`Mainboard/`](../Mainboard/) |
| [`Maxx-Steele-CPU-Pinout.xlsx`](Maxx-Steele-CPU-Pinout.xlsx) | Robot CPU pinout (spreadsheet) | [`Mainboard/`](../Mainboard/) |
| [`U400-KM2365-ROM.pdf`](U400-KM2365-ROM.pdf) | KM2365 EPROM (cartridge ROM) | [`Cartridge/`](../Cartridge/), [`docs/PROGRAMMING.md`](../docs/PROGRAMMING.md) |
| [`U401-M6116-RAM.pdf`](U401-M6116-RAM.pdf) | M6116 static RAM | [`Mainboard/`](../Mainboard/) |
| [`U500-COP41xL-Display-and-Motors.pdf`](U500-COP41xL-Display-and-Motors.pdf) | COP41xL display & motor driver | [`Face/`](../Face/), [`Mainboard/`](../Mainboard/) |
| [`U500-Eurotechnique-ET9420-Datasheet.pdf`](U500-Eurotechnique-ET9420-Datasheet.pdf) | Eurotechnique ET9420 | [`Face/`](../Face/) |
| [`U500-Thompson-ET9420-Options.pdf`](U500-Thompson-ET9420-Options.pdf) | Thompson ET9420 options | [`Face/`](../Face/) |
| [`LC3100.pdf`](LC3100.pdf) | Sanyo LC3100 speech chip | [`Face/`](../Face/) |
| [`LC3100-Sanyo.pdf`](LC3100-Sanyo.pdf) | Sanyo LC3100 (alternate) | [`Face/`](../Face/) |
| [`LC8100.pdf`](LC8100.pdf) | Sanyo LC8100 speech chip | [`Face/`](../Face/) |
| [`Sanyo-LC81196-datasheet-lc8100.pdf`](Sanyo-LC81196-datasheet-lc8100.pdf) | Sanyo LC81196 / LC8100 family | [`Face/`](../Face/) |

## Adding datasheets

1. Place new third-party PDFs here (shell-safe filenames — see [naming conventions](../README.md#naming-conventions)).
2. Add a row to the index table above with links to related modules and docs.
3. Link from the module README `Datasheets/` pointer (if any), not the other way around.