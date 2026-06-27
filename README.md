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
| **Community manuals** | **`Docs/`** + CamelCase subfolders | [`Docs/User/`](Docs/User/), [`Docs/Technical/`](Docs/Technical/), [`Docs/Mechanical/`](Docs/Mechanical/) |
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
| [`Cartridge/`](Cartridge/) | ROM, firmware, [programming manual](Cartridge/PROGRAMMING.md), [technical manual](Docs/Technical/README.md) | [`CBSDemo.kicad_pro`](Cartridge/Examples/CBSDemo/KiCAD/CBSDemo.kicad_pro) (active); UltraMaxx planned |
| [`PaddleMirror/`](PaddleMirror/) | Archive | `PaddleMirror/KiCAD/` (planned) |
| [`Chassis/`](Chassis/) | Body, manuals, photos | `Chassis/KiCAD/` (mechanical, planned) |

### Documentation & tools

| Path | Description |
|------|-------------|
| [`Docs/`](Docs/) | Community manuals — [index](Docs/README.md) |
| [`Docs/User/`](Docs/User/) | Owner manual (setup, operation, games) — [PDF](Docs/User/Maxx-Steele-User-Manual.pdf) |
| [`Docs/Technical/`](Docs/Technical/) | Technical manual (bytecode, ROM, I/O) — [PDF](Docs/Technical/Maxx-Steele-Technical-Manual.pdf) |
| [`Docs/Mechanical/`](Docs/Mechanical/) | Mechanical manual (disassembly, photos) — [PDF](Docs/Mechanical/Maxx-Steele-Mechanical-Manual.pdf) |
| [`DataSheets/`](DataSheets/) | Third-party component datasheets (indexed in [`DataSheets/README.md`](DataSheets/README.md)) |
| [`tools/`](tools/) | [`tools/bin/`](tools/bin/) shell commands (`maxx`, `maxx-rom`, `picorom-cart`), [`maxxbas/`](tools/maxxbas/) Rust library + `maxx simulate`, Python modules, KiCad helpers, [`rfcap/`](tools/rfcap/) GNU Radio OOK flowgraphs |
| [`libraries/`](libraries/) | Shared KiCad symbol libraries |

## Paths

Scripts resolve **project-relative** paths from the repository root (`tools/project_paths.py`). KiCad modules use `${KIPRJMOD}/../../libraries/` for shared symbols.

## Quick commands

```bash
export PATH="$(pwd)/tools/bin:$PATH"

maxx compile Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas
maxx list Cartridge/Examples/UltraMaxx/Firmware/Binary/hello.532 --json
maxx upload Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas --device maxx_cart --dry-run
maxx simulate Cartridge/Examples/UltraMaxx/Firmware/Binary/hello.532 --gui

# KiCad 10: open Transmitter/KiCAD/Transmitter-27MHz.kicad_pro
```

See [`tools/bin/README.md`](tools/bin/README.md) for all commands.

## Related repositories

- [cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library) — upstream COP400 KiCad symbols