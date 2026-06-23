# Maxx-Steele-1984-Robot

Information, code, and modifications for the 1984 Maxx Steele robot from Ideal (CBS Toys). This repository replaces the archive from the now-closed Yahoo Groups.

The intent is to allow people who own, repair, or modify their robots to share information. Feel free to request to fold your own additions back into the repository.

Much of the information in this archive has unknown ownership or uses licensed names and artwork. Everything here is believed to be appropriately used under Fair Use. If you are a copyright or trademark owner, please contact the maintainer so that appropriate use may be agreed upon.

## Repository layout

### Original archive

| Path | Description |
|------|-------------|
| [`Robot/`](Robot/) | Robot body — electronics, firmware, photos, schematics |
| [`Receiver/`](Receiver/) | Robot 27 MHz receiver module — KiCad project |
| [`Transmitter/`](Transmitter/) | 27 MHz remote — photos, RE notes, datasheets, KiCad project |
| [`Accessories/`](Accessories/) | Demo cartridge and other accessories |
| [`Manual/`](Manual/) | Original manuals |
| [`References/`](References/) | Third-party articles and workshop notes |
| [`Artwork/`](Artwork/) | Logos and artwork |

### Reverse-engineering additions

| Path | Description |
|------|-------------|
| [`libraries/`](libraries/) | Shared [cop41xl](https://github.com/webaugur/cop41xl-kicad-library) KiCad MCU library |
| [`firmware/`](firmware/) | Curated ROM binaries + disassembly listings |
| [`docs/`](docs/) | [Cartridge programming manual](docs/PROGRAMMING.md), [transmitter BOM](docs/transmitter-bom.md), [transmitter architecture / 455 kHz IF clock](docs/transmitter-architecture.md) |
| [`tools/`](tools/) | Cartridge ROM disassembler / validator |
| [`sch/`](sch/) | Schematic photos, PCB images, handwritten BOM |
| [`rfcap/`](rfcap/) | GNU Radio 27 MHz OOK flowgraphs; [per-file catalog](rfcap/README.md) (`.dat` IQ captures are local-only, not in git) |
| [`references/`](references/) | Local PDF copies of key datasheets and RE notes |

## Quick commands

```bash
# Disassemble demo cartridge
python3 tools/maxx_rom.py disasm firmware/demo-cart/MAXXCART.532

# Open KiCad transmitter project
# Transmitter/KiCAD/Transmitter-27MHz.pro
```

## Related repositories

- [cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library) — COP400-series KiCad symbols