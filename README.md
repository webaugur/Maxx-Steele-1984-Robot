# Maxx-Steele-1984-Robot

Information, code, and modifications for the 1984 Maxx Steele robot from Ideal (CBS Toys). This repository replaces the archive from the now-closed Yahoo Groups.

The intent is to allow people who own, repair, or modify their robots to share information. Feel free to request to fold your own additions back into the repository.

Much of the information in this archive has unknown ownership or uses licensed names and artwork. Everything here is believed to be appropriately used under Fair Use. If you are a copyright or trademark owner, please contact the maintainer so that appropriate use may be agreed upon.

## Repository layout

| Path | Description |
|------|-------------|
| **Archive** | |
| [`Robot/`](Robot/) | Robot body — electronics, firmware, schematics |
| [`Transmitter/`](Transmitter/) | Remote — stickers, RE notes, datasheets |
| [`Accessories/`](Accessories/) | Demo cartridge, paddle mirror |
| [`Manual/`](Manual/) | Original manuals |
| [`References/`](References/) | Third-party articles and workshop notes |
| [`Artwork/`](Artwork/) | Logos and artwork |
| **Active work** | |
| [`kicad/`](kicad/) | KiCad projects (transmitter, receiver) + symbol libraries |
| [`docs/`](docs/) | RE manuals, BOM, architecture notes, [photos](docs/photos/), [reference PDFs](docs/references/) |
| [`firmware/`](firmware/) | Curated ROM binaries + disassembly listings |
| [`tools/`](tools/) | Cartridge ROM disassembler / validator |
| [`rfcap/`](rfcap/) | GNU Radio OOK flowgraphs; IQ data in [`rfcap/captures/`](rfcap/captures/) |

## Paths

Scripts resolve **project-relative** paths from the repository root (`tools/project_paths.py`). KiCad projects use `${KIPRJMOD}/../libraries/` for shared symbols.

## Quick commands

```bash
# Disassemble demo cartridge
python3 tools/maxx_rom.py disasm firmware/demo-cart/MAXXCART.532

# Open KiCad transmitter project
# kicad/transmitter/Transmitter-27MHz.pro
```

## Related repositories

- [cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library) — upstream COP400 KiCad symbols