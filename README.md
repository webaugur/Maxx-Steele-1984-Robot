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
| **Technical manual** | **`TechnicalManual/`** (overrides lowercase rule) | C64-style programming manual; see [`TechnicalManual/README.md`](TechnicalManual/README.md) |
| **Mechanical manual** | **`MechanicalManual/`** (overrides lowercase rule) | Disassembly / reassembly guide; see [`MechanicalManual/README.md`](MechanicalManual/README.md) |
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
| [`Transmitter/`](Transmitter/) | Active — [architecture](Transmitter/transmitter-architecture.md), [BOM](Transmitter/transmitter-bom.md) | [`Transmitter/KiCAD/Transmitter-27MHz.kicad_pro`](Transmitter/KiCAD/Transmitter-27MHz.kicad_pro) |
| [`Receiver/`](Receiver/) | Active | [`Receiver/KiCAD/Receiver-27MHz.kicad_pro`](Receiver/KiCAD/Receiver-27MHz.kicad_pro) |
| [`Power/`](Power/) | Planned | `Power/KiCAD/` |
| [`Mainboard/`](Mainboard/) | Planned | `Mainboard/KiCAD/` |
| [`Face/`](Face/) | Planned | `Face/KiCAD/` |
| [`Cartridge/`](Cartridge/) | ROM, firmware, [programming manual](Cartridge/PROGRAMMING.md), [technical manual](TechnicalManual/README.md) | [`CBSDemo.kicad_pro`](Cartridge/Examples/CBSDemo/KiCAD/CBSDemo.kicad_pro) (active); UltraMaxx planned |
| [`PaddleMirror/`](PaddleMirror/) | Archive | `PaddleMirror/KiCAD/` (planned) |
| [`Chassis/`](Chassis/) | Body, manuals, photos, internal ROM | `Chassis/KiCAD/` (mechanical, planned) |

### Tools & shared assets

| Path | Description |
|------|-------------|
| [`TechnicalManual/`](TechnicalManual/) | Technical manual (bytecode, opcodes, I/O, appendices) — [README](TechnicalManual/README.md), [PDF](TechnicalManual/Maxx-Steele-Technical-Manual.pdf) |
| [`MechanicalManual/`](MechanicalManual/) | Mechanical manual (disassembly, reassembly, teardown photos) — [README](MechanicalManual/README.md), [PDF](MechanicalManual/Maxx-Steele-Mechanical-Manual.pdf) |
| [`DataSheets/`](DataSheets/) | Third-party component datasheets (indexed in [`DataSheets/README.md`](DataSheets/README.md)) |
| [`Simulator/`](Simulator/) | Unified robot simulator (patches, docs); run via `maxx simulate` |
| [`tools/`](tools/) | [`tools/bin/`](tools/bin/) shell commands (`maxx`, `maxx-rom`, `picorom-cart`), [`maxxbas/`](tools/maxxbas/) Rust library, Python modules, KiCad helpers, [`rfcap/`](tools/rfcap/) GNU Radio OOK flowgraphs |
| [`libraries/`](libraries/) | Shared KiCad symbol libraries |

## Paths

Scripts resolve **project-relative** paths from the repository root (`tools/project_paths.py`). KiCad modules use `${KIPRJMOD}/../../libraries/` for shared symbols.

## Quick commands

```bash
export PATH="$(pwd)/tools/bin:$PATH"

maxx compile Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas
maxx list Cartridge/Examples/UltraMaxx/Firmware/Binary/hello.532 --json
maxx upload Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas --device maxx_cart --dry-run

# KiCad 10: open Transmitter/KiCAD/Transmitter-27MHz.kicad_pro
```

See [`tools/bin/README.md`](tools/bin/README.md) for all commands.

## Related repositories

- [cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library) — upstream COP400 KiCad symbols