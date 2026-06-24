# TODO — Missing and incomplete archive content

Generated from an audit of empty directories, module READMEs, and KiCad project status (2026-06-23).

Each module [`README.md`](Transmitter/README.md) (and [`tools/rfcap/README.md`](tools/rfcap/README.md)) includes a **TODO** section mirrored from this file. When you check an item off, update **both** the local README and this file.

Folder and filename naming: see [Naming conventions](README.md#naming-conventions) in the root README.

## GitHub issues

Yes — these items can be tracked as [GitHub Issues](https://docs.github.com/en/issues). **Issues are currently disabled** on [`webaugur/Maxx-Steele-1984-Robot`](https://github.com/webaugur/Maxx-Steele-1984-Robot); enable them under **Settings → General → Features → Issues**, then create backlog issues.

**Recommended layout** (one tracking issue per module, checklist in the issue body):

| Module | Suggested issue title | Labels |
|--------|----------------------|--------|
| Transmitter | `Transmitter backlog` | `backlog`, `transmitter` |
| Receiver | `Receiver backlog` | `backlog`, `receiver` |
| Mainboard | `Mainboard backlog` | `backlog`, `mainboard` |
| Cartridge | `Cartridge backlog` | `backlog`, `cartridge` |
| Chassis | `Chassis backlog` | `backlog`, `chassis` |
| Power | `Power backlog` | `backlog`, `power` |
| Face | `Face backlog` | `backlog`, `face` |
| PaddleMirror | `PaddleMirror backlog` | `backlog`, `paddlemirror` |
| RFCAP captures | `RFCAP capture annotations` | `backlog`, `tools` |

Copy the open checklist from each module README into the matching issue. When an item is done, check it off in **three places**: the GitHub issue, this file, and the module README. Add the issue link to the module README `## TODO` section once created.

**Legend:** `[empty]` = directory has a README placeholder only (no real assets yet). `[partial]` = work started but not finished. `[planned]` = README marks module as not yet started.

---

## Summary

| Module | Status | Biggest gaps |
|--------|--------|--------------|
| Transmitter | **Active** | KiCad repair/fab; empty schematic/PCB/3D folders |
| Receiver | **Active** | Schematic only — no PCB layout or fab outputs |
| Mainboard | Partial | Raster schematics; no KiCad project |
| Cartridge | Partial | Firmware done; no hardware CAD |
| Chassis | Mostly complete | No 3D body model or mechanical KiCad |
| Power | Planned | No KiCad project |
| Face | Planned | No KiCad project |
| PaddleMirror | Archive | No KiCad or 3D model |

---

## Active modules

### Transmitter

KiCad project: [`Transmitter/KiCAD/Transmitter-27MHz.pro`](Transmitter/KiCAD/Transmitter-27MHz.pro)

**KiCad / RE (in progress)**

- [ ] Run ERC; resolve outstanding errors/warnings
- [ ] Confirm COP411L-PAC/N **DIP-20** footprint matches physical package
- [ ] Sync schematic ↔ PCB (31 nets per current `.net`)
- [ ] Run DRC on layout
- [ ] Export Gerbers / drill files for fab review
- [ ] Align [`docs/transmitter-bom.md`](docs/transmitter-bom.md) with final schematic refs

**Completely missing artifact folders** `[empty]`

- [ ] [`Transmitter/Schematic/`](Transmitter/Schematic/) — dedicated schematic scan or vector export (separate from KiCad sources)
- [ ] [`Transmitter/PCBoard/`](Transmitter/PCBoard/) — PCB photos, fab drawings, or layout exports
- [ ] [`Transmitter/Model3D/`](Transmitter/Model3D/) — enclosure / remote mechanical CAD
- [x] [`Transmitter/Photos/`](Transmitter/Photos/) — product and reverse-engineering images

**Housekeeping** (dirs have content; `null.txt` is stale)

- [x] Remove [`Transmitter/Datasheets/null.txt`](Transmitter/Datasheets/null.txt) (`National-COP411L.pdf` present)
- [x] Remove [`Transmitter/Stickers/null.txt`](Transmitter/Stickers/null.txt) (SVG/PDF artwork present)
- [x] Remove [`Transmitter/ReverseEngineering/null.txt`](Transmitter/ReverseEngineering/null.txt) (ODT/DOCX/PDF notes present)

### Receiver

KiCad project: [`Receiver/KiCAD/Receiver-27MHz.pro`](Receiver/KiCAD/Receiver-27MHz.pro) — schematic only

**KiCad (in progress)**

- [ ] Create `.kicad_pcb` from schematic
- [ ] Assign footprints for superhet / OOK strip
- [ ] Route board; generate `.net` and sync with schematic
- [ ] Run ERC and DRC
- [ ] Export Gerbers
- [ ] Add [`Receiver/KiCAD/README.md`](Receiver/KiCAD/README.md) (other modules have one)
- [ ] Expand [`Receiver/README.md`](Receiver/README.md) with BOM/architecture cross-links (mirror Transmitter)

**Not yet started** (no directories)

- [ ] Receiver schematic scans / photos (if separate from KiCad)
- [ ] Receiver PCB photos / layout archive
- [ ] Add receiver IC datasheets to [`DataSheets/`](DataSheets/)
- [ ] Receiver 3D model

---

## Partially started modules

### Mainboard

- [ ] Digitize [`Mainboard/Schematic/`](Mainboard/Schematic/) raster/SVG into KiCad — [`Mainboard/KiCAD/`](Mainboard/KiCAD/) is `[planned]` (README + `sym-lib-table` only)
- [ ] PCB layout project and fabrication artifacts
- [ ] Module-specific photos (chassis photos: [`Chassis/Photos/`](Chassis/Photos/))

### Cartridge

Firmware is complete ([`Cartridge/Firmware/`](Cartridge/Firmware/)).

**Completely missing** `[empty]`

- [ ] [`Cartridge/KiCAD/`](Cartridge/KiCAD/) — cartridge PCB KiCad project
- [ ] [`Cartridge/Schematic/`](Cartridge/Schematic/) — schematic scan or source
- [ ] [`Cartridge/PCBoard/`](Cartridge/PCBoard/) — PCB layout / photos
- [ ] [`Cartridge/Model3D/`](Cartridge/Model3D/) — cartridge mechanical model
- [x] [`Cartridge/Photos/`](Cartridge/Photos/) — cartridge card photo

### Chassis

Most content present (manuals, photos, firmware, datasheets, sounds, disassembly).

**Completely missing** `[empty]`

- [ ] [`Chassis/Model3D/`](Chassis/Model3D/) — full robot body mechanical CAD
- [ ] [`Chassis/KiCAD/`](Chassis/KiCAD/) — enclosure / mechanism drawings (README placeholder only)

**Housekeeping**

- [x] Remove [`Chassis/Artwork/Null`](Chassis/Artwork/Null) (SVG logos present)
- [x] Remove [`Chassis/Sounds/Null.txt`](Chassis/Sounds/Null.txt) (`.wma` samples present)

---

## Planned modules (not started)

### Power `[planned]`

- [ ] KiCad project under [`Power/KiCAD/`](Power/KiCAD/) (currently README + `sym-lib-table` only)
- [ ] Schematic source or scans
- [ ] PCB layout and photos
- [ ] Add power-module datasheets to [`DataSheets/`](DataSheets/)

### Face `[planned]`

- [ ] KiCad project under [`Face/KiCAD/`](Face/KiCAD/)
- [ ] Display / speech subsystem schematic
- [ ] PCB layout and photos; add face/speech IC datasheets to [`DataSheets/`](DataSheets/)

### PaddleMirror `[archive]`

**Completely missing** `[empty]`

- [ ] [`PaddleMirror/KiCAD/`](PaddleMirror/KiCAD/) — KiCad project
- [ ] [`PaddleMirror/Model3D/`](PaddleMirror/Model3D/) — mechanical CAD
- [x] [`PaddleMirror/Photos/`](PaddleMirror/Photos/) — accessory photo

---

## Repository hygiene

- [x] Remove orphan empty [`Accessories/`](Accessories/) (leftover after promoting Cartridge and PaddleMirror to top-level modules)
- [x] Photo policy: module photos live under each module's `Photos/` folder (removed `docs/photos/`)

---

## Tools and captures

- [ ] Annotate four empty per-button capture sidecars in [`tools/rfcap/captures/`](tools/rfcap/captures/):
  - `2021.02.14.06.50.10.dat.txt`
  - `2021.02.14.06.51.12.dat.txt`
  - `2021.02.14.06.52.42.dat.txt`
  - `2021.02.14.06.54.21.dat.txt`

---

## Suggested priority

1. **Transmitter** — ERC, footprint check, schematic–PCB sync, DRC, Gerbers
2. **Receiver** — PCB layout and fab outputs
3. **Mainboard** — KiCad digitization from existing schematic art
4. **Cartridge** — hardware CAD (schematic, PCB, 3D)
5. **Chassis** — 3D body model and mechanical drawings
6. **Power / Face** — KiCad projects when boards are ready to document
7. ~~**Housekeeping**~~ — done (stale placeholders removed, `Accessories/` deleted, photo README pointers added)