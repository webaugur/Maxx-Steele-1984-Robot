# Maxx-Steele-1984-Robot

Information, code, and modifications for the 1984 Maxx Steele robot from Ideal (CBS Toys). This repository replaces the archive from the now-closed Yahoo Groups.

The intent is to allow people who own, repair, or modify their robots to share information. Feel free to request to fold your own additions back into the repository.

Much of the information in this archive has unknown ownership or uses licensed names and artwork. Everything here is believed to be appropriately used under Fair Use. If you are a copyright or trademark owner, please contact the maintainer so that appropriate use may be agreed upon.

## Naming conventions

| Scope | Rule | Examples |
|-------|------|----------|
| **Top-level hardware modules** | `CamelCase` (no spaces or hyphens) | `Transmitter/`, `Mainboard/`, `PaddleMirror/`, `Cartridge/` |
| **Top-level shared folders** | `lowercase` | `tools/`, `libraries/` |
| **Third-party datasheets** | **`DataSheets/`** (overrides lowercase rule) | Vendor PDFs as `OEM-PartNumber.pdf`; see [`DataSheets/README.md`](DataSheets/README.md) |
| **Programmer's reference** | **`ProgrammersGuide/`** (overrides lowercase rule) | C64-style programming manual; see [`ProgrammersGuide/README.md`](ProgrammersGuide/README.md) |
| **Subfolders inside modules** | `CamelCase` | `Photos/`, `Firmware/`, `KiCAD/`, `PCBoard/`, `Model3D/`, `Photos/Product/`, `Photos/ReverseEngineering/` |
| **Subfolders inside shared folders** | `lowercase` | `tools/rfcap/`, `tools/rfcap/captures/` |
| **All filenames** | shell-safe: `[A-Za-z0-9._-]` only | `Remote-Front.jpg`, `maxx-song-1.wma`, `Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf` |

Multi-word module or folder names run together (`PaddleMirror`, not `Paddle Mirror`). Acronyms keep familiar casing inside CamelCase (`KiCAD`, `PCBoard`).

**Filenames** must be shell-safe: only ASCII letters, digits, dot (`.`), underscore (`_`), and hyphen (`-`). No spaces or other punctuation. Use hyphens between words (`Remote-Front.jpg`, `maxx-song-1.wma`). Check or fix names with:

```bash
python3 tools/sanitize_filenames.py          # dry-run
python3 tools/sanitize_filenames.py --apply  # git mv fixes
```

## Repository layout

### Hardware modules

Each module has a [`KiCAD/`](Transmitter/KiCAD/) subfolder for schematic/PCB work and shares [`libraries/`](libraries/) for symbols.

| Module | Status | KiCad project |
|--------|--------|---------------|
| [`Transmitter/`](Transmitter/) | Active — [architecture](Transmitter/transmitter-architecture.md), [BOM](Transmitter/transmitter-bom.md) | [`Transmitter/KiCAD/Transmitter-27MHz.pro`](Transmitter/KiCAD/Transmitter-27MHz.pro) |
| [`Receiver/`](Receiver/) | Active | [`Receiver/KiCAD/Receiver-27MHz.pro`](Receiver/KiCAD/Receiver-27MHz.pro) |
| [`Power/`](Power/) | Planned | `Power/KiCAD/` |
| [`Mainboard/`](Mainboard/) | Planned | `Mainboard/KiCAD/` |
| [`Face/`](Face/) | Planned | `Face/KiCAD/` |
| [`Cartridge/`](Cartridge/) | ROM, firmware, [programming manual](Cartridge/PROGRAMMING.md), [programmer's guide](ProgrammersGuide/README.md) | `Cartridge/KiCAD/` (planned) |
| [`PaddleMirror/`](PaddleMirror/) | Archive | `PaddleMirror/KiCAD/` (planned) |
| [`Chassis/`](Chassis/) | Body, manuals, photos, internal ROM | `Chassis/KiCAD/` (mechanical, planned) |

### Tools & shared assets

| Path | Description |
|------|-------------|
| [`ProgrammersGuide/`](ProgrammersGuide/) | Programmer's reference (bytecode, opcodes, I/O, appendices) — [`ProgrammersGuide/README.md`](ProgrammersGuide/README.md) |
| [`DataSheets/`](DataSheets/) | Third-party component datasheets (indexed in [`DataSheets/README.md`](DataSheets/README.md)) |
| [`tools/`](tools/) | Cartridge ROM tools, [`rfcap/`](tools/rfcap/) GNU Radio OOK flowgraphs, [`sanitize_filenames.py`](tools/sanitize_filenames.py) |
| [`libraries/`](libraries/) | Shared KiCad symbol libraries |

## Paths

Scripts resolve **project-relative** paths from the repository root (`tools/project_paths.py`). KiCad modules use `${KIPRJMOD}/../../libraries/` for shared symbols.

## Quick commands

```bash
python3 tools/maxx_rom.py disasm "Cartridge/Firmware/Binary/MAXXCART.532"

# KiCad: Transmitter/KiCAD/Transmitter-27MHz.pro
```

## Related repositories

- [cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library) — upstream COP400 KiCad symbols