# Maxx-Steele-1984-Robot

Information, code, and modifications for the 1984 Maxx Steele robot from Ideal (CBS Toys). This repository replaces the archive from the now-closed Yahoo Groups.

The intent is to allow people who own, repair, or modify their robots to share information. Feel free to request to fold your own additions back into the repository.

Much of the information in this archive has unknown ownership or uses licensed names and artwork. Everything here is believed to be appropriately used under Fair Use. If you are a copyright or trademark owner, please contact the maintainer so that appropriate use may be agreed upon.

## Repository layout

### Hardware modules

Each module has a [`KiCAD/`](Transmitter/KiCAD/) subfolder for schematic/PCB work and shares [`libraries/`](libraries/) for symbols.

| Module | Status | KiCad project |
|--------|--------|---------------|
| [`Transmitter/`](Transmitter/) | Active | [`Transmitter/KiCAD/Transmitter-27MHz.pro`](Transmitter/KiCAD/Transmitter-27MHz.pro) |
| [`Receiver/`](Receiver/) | Active | [`Receiver/KiCAD/Receiver-27MHz.pro`](Receiver/KiCAD/Receiver-27MHz.pro) |
| [`Power/`](Power/) | Planned | `Power/KiCAD/` |
| [`Mainboard/`](Mainboard/) | Planned | `Mainboard/KiCAD/` |
| [`Face/`](Face/) | Planned | `Face/KiCAD/` |
| [`Demo Cartridge/`](Demo%20Cartridge/) | Archive + firmware | `Demo Cartridge/KiCAD/` (planned) |
| [`Paddle Mirror/`](Paddle%20Mirror/) | Archive | `Paddle Mirror/KiCAD/` (planned) |
| [`Chassis/`](Chassis/) | Body, manuals, photos, artwork | `Chassis/KiCAD/` (mechanical, planned) |

### Tools & documentation

| Path | Description |
|------|-------------|
| [`docs/`](docs/) | RE manuals, [module photos](docs/photos/), [reference PDFs](docs/references/) |
| [`firmware/`](firmware/) | Curated ROM binaries + disassembly listings |
| [`tools/`](tools/) | Cartridge ROM disassembler / validator |
| [`rfcap/`](rfcap/) | GNU Radio OOK flowgraphs; data in [`rfcap/captures/`](rfcap/captures/) |
| [`libraries/`](libraries/) | Shared KiCad symbol libraries |

## Paths

Scripts resolve **project-relative** paths from the repository root (`tools/project_paths.py`). KiCad modules use `${KIPRJMOD}/../../libraries/` for shared symbols.

## Quick commands

```bash
python3 tools/maxx_rom.py disasm firmware/demo-cart/MAXXCART.532

# KiCad: Transmitter/KiCAD/Transmitter-27MHz.pro
```

## Related repositories

- [cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library) — upstream COP400 KiCad symbols