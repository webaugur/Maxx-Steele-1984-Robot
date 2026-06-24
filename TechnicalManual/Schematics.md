# Schematic Diagram Index

The Commodore 64 Programmer's Reference included a fold-out schematic. This index points to equivalent materials in the Maxx Steele archive.

---

## Main logic board

| Asset | Path | Status |
|-------|------|--------|
| Raster schematic (enhanced) | [`Mainboard/Schematic/Maxx_Steele_Schematic_enh-v1.1.png`](../Mainboard/Schematic/Maxx_Steele_Schematic_enh-v1.1.png) | Available |
| SVG (v1.1) | [`Mainboard/Schematic/v1.1/Maxx_Steele_Schematic.svg`](../Mainboard/Schematic/v1.1/Maxx_Steele_Schematic.svg) | Available |
| KiCad project | [`Mainboard/KiCAD/`](../Mainboard/KiCAD/) | Planned — digitization in progress |

6502 CPU, RAM (M6116), cartridge interface, speech and motor glue.

---

## Transmitter (remote)

| Asset | Path | Status |
|-------|------|--------|
| KiCad schematic + PCB | [`Transmitter/KiCAD/Transmitter-27MHz.kicad_pro`](../Transmitter/KiCAD/Transmitter-27MHz.kicad_pro) | Active |
| RE notes | [`Transmitter/ReverseEngineering/`](../Transmitter/ReverseEngineering/) | Available |
| BOM | [`Transmitter/transmitter-bom.md`](../Transmitter/transmitter-bom.md) | Available |

COP411L (U1), 455 kHz clock, 27 MHz OOK stage.

---

## Receiver (robot RF)

| Asset | Path | Status |
|-------|------|--------|
| KiCad schematic | [`Receiver/KiCAD/Receiver-27MHz.kicad_pro`](../Receiver/KiCAD/Receiver-27MHz.kicad_pro) | Schematic only |
| PCB layout | — | Not yet in archive |

---

## Cartridge

| Asset | Path | Status |
|-------|------|--------|
| KiCad (CBSDemo) | [`Cartridge/Examples/CBSDemo/KiCAD/CBSDemo.kicad_pro`](../Cartridge/Examples/CBSDemo/KiCAD/CBSDemo.kicad_pro) | Active (rev 0.1) |
| KiCad (UltraMaxx) | [`Cartridge/Examples/UltraMaxx/KiCAD/`](../Cartridge/Examples/UltraMaxx/KiCAD/) (same PCB as CBSDemo) | Active |
| PicoROM (UltraMaxx U1) | [`Cartridge/Examples/UltraMaxx/PICOROM.md`](../Cartridge/Examples/UltraMaxx/PICOROM.md) | Active |

---

## Face / speech / display

| Asset | Path | Status |
|-------|------|--------|
| KiCad | [`Face/KiCAD/`](../Face/KiCAD/) | Planned |
| Datasheets | [`DataSheets/`](../DataSheets/) | LC3100, LC8100, ET9420, COP41xL |

---

## Power module

| Asset | Path | Status |
|-------|------|--------|
| KiCad | [`Power/KiCAD/`](../Power/KiCAD/) | Planned |

---

## Paddle mirror accessory

| Asset | Path | Status |
|-------|------|--------|
| Photo | [`PaddleMirror/Photos/`](../PaddleMirror/Photos/) | Available |
| Schematic | — | **Needed** |

---

## Chip cross-reference

Indexed datasheets with refdes where known: [`DataSheets/README.md`](../DataSheets/README.md).