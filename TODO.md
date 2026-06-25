# TODO — Missing and incomplete archive content

Generated from an audit of empty directories, module READMEs, and KiCad project status (2026-06-23).

Each module [`README.md`](Transmitter/README.md) (and [`tools/rfcap/README.md`](tools/rfcap/README.md)) includes a **TODO** section mirrored from this file. When you check an item off, update **both** the local README and this file.

Folder and filename naming: see [Naming conventions](README.md#naming-conventions) in the root README.

## GitHub issues

Open backlog items are tracked as individual [GitHub Issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Abacklog) ([#2](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/2)–[#41](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/41)). Each open `[ ]` item below links to its issue.

| Module | Filter |
|--------|--------|
| Transmitter | [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Atransmitter+label%3Abacklog) |
| Receiver | [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Areceiver+label%3Abacklog) |
| Mainboard | [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Amainboard+label%3Abacklog) |
| Cartridge | [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Acartridge+label%3Abacklog) |
| Chassis | [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Achassis+label%3Abacklog) |
| Power | [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Apower+label%3Abacklog) |
| Face | [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Aface+label%3Abacklog) |
| PaddleMirror | [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Apaddlemirror+label%3Abacklog) |
| RFCAP / tools | [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Atools+label%3Abacklog) |

When an item is done, check it off in **three places**: the GitHub issue, this file, and the module README.

**Legend:** `[empty]` = directory has a README placeholder only (no real assets yet). `[partial]` = work started but not finished. `[planned]` = README marks module as not yet started.

---

## Summary

| Module | Status | Biggest gaps |
|--------|--------|--------------|
| Transmitter | **Active** | KiCad repair/fab; empty schematic/PCB/3D folders |
| Receiver | **Active** | Schematic only — no PCB layout or fab outputs |
| Mainboard | Partial | Raster schematics; no KiCad project |
| Cartridge | Partial | Firmware done; CBSDemo schematic rev 0.1; PCB layout TBD |
| Chassis | Mostly complete | No 3D body model or mechanical KiCad |
| Power | Planned | No KiCad project |
| Face | Planned | No KiCad project |
| PaddleMirror | Archive | No KiCad or 3D model |

---

## Active modules

### Transmitter

KiCad project: [`Transmitter/KiCAD/Transmitter-27MHz.kicad_pro`](Transmitter/KiCAD/Transmitter-27MHz.kicad_pro)

**KiCad / RE (in progress)**

- [ ] Run ERC; resolve outstanding errors/warnings ([#2](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/2))
- [ ] Confirm COP411L-PAC/N **DIP-20** footprint matches physical package ([#3](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/3))
- [ ] Sync schematic ↔ PCB (export netlist from `.kicad_sch` via `kicad-cli sch export netlist`) ([#4](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/4))
- [ ] Run DRC on layout ([#5](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/5))
- [ ] Export Gerbers / drill files for fab review ([#6](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/6))
- [ ] Align [`Transmitter/transmitter-bom.md`](Transmitter/transmitter-bom.md) with final schematic refs ([#7](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/7))

**Completely missing artifact folders** `[empty]`

- [ ] [`Transmitter/Schematic/`](Transmitter/Schematic/) — dedicated schematic scan or vector export (separate from KiCad sources) ([#8](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/8))
- [ ] [`Transmitter/PCBoard/`](Transmitter/PCBoard/) — PCB photos, fab drawings, or layout exports ([#9](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/9))
- [ ] [`Transmitter/Model3D/`](Transmitter/Model3D/) — enclosure / remote mechanical CAD ([#10](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/10))
- [x] [`Transmitter/Photos/`](Transmitter/Photos/) — product and reverse-engineering images

**Housekeeping** (dirs have content; `null.txt` is stale)

- [x] Remove [`Transmitter/Datasheets/null.txt`](Transmitter/Datasheets/null.txt) (`National-COP411L.pdf` present)
- [x] Remove [`Transmitter/Stickers/null.txt`](Transmitter/Stickers/null.txt) (SVG/PDF artwork present)
- [x] Remove [`Transmitter/ReverseEngineering/null.txt`](Transmitter/ReverseEngineering/null.txt) (ODT/DOCX/PDF notes present)

### Receiver

KiCad project: [`Receiver/KiCAD/Receiver-27MHz.kicad_pro`](Receiver/KiCAD/Receiver-27MHz.kicad_pro) — schematic only

**KiCad (in progress)**

- [ ] Create `.kicad_pcb` from schematic ([#11](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/11))
- [ ] Assign footprints for superhet / OOK strip ([#12](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/12))
- [ ] Route board; generate `.net` and sync with schematic ([#13](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/13))
- [ ] Run ERC and DRC ([#14](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/14))
- [ ] Export Gerbers ([#15](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/15))
- [ ] Add [`Receiver/KiCAD/README.md`](Receiver/KiCAD/README.md) (other modules have one) ([#16](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/16))
- [ ] Expand [`Receiver/README.md`](Receiver/README.md) with BOM/architecture cross-links (mirror Transmitter) ([#17](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/17))

**Not yet started** (no directories)

- [ ] Receiver schematic scans / photos (if separate from KiCad) ([#18](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/18))
- [ ] Receiver PCB photos / layout archive ([#19](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/19))
- [ ] Add receiver IC datasheets to [`DataSheets/`](DataSheets/) ([#20](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/20))
- [ ] Receiver 3D model ([#21](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/21))

---

## Partially started modules

### Mainboard

- [ ] Digitize [`Mainboard/Schematic/`](Mainboard/Schematic/) raster/SVG into KiCad — [`Mainboard/KiCAD/`](Mainboard/KiCAD/) is `[planned]` (README + `sym-lib-table` only) ([#22](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/22))
- [ ] PCB layout project and fabrication artifacts ([#23](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/23))
- [ ] Module-specific photos (chassis photos: [`Chassis/Photos/`](Chassis/Photos/)) ([#24](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/24))

### Cartridge

Example firmware is complete ([`Cartridge/Examples/`](Cartridge/Examples/)).

**Partial** — schematic rev 0.1 from PCB photo; PCB layout not started

- [x] [`Cartridge/Examples/CBSDemo/KiCAD/`](Cartridge/Examples/CBSDemo/KiCAD/) — CBSDemo cartridge schematic ([#25](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/25)) — PCB layout still open
- [x] [`Cartridge/Examples/UltraMaxx/KiCAD/`](Cartridge/Examples/UltraMaxx/KiCAD/) — UltraMaxx hardware pointer ([#27](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/27))
- [x] [`Cartridge/Examples/UltraMaxx/PICOROM.md`](Cartridge/Examples/UltraMaxx/PICOROM.md) — PicoROM P28 adaptation for U1 (based on CBSDemo; [wickerwaka/PicoROM](https://github.com/wickerwaka/PicoROM))
- [x] [`Cartridge/Model3D/`](Cartridge/Model3D/) — cartridge STEP assembly ([#28](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/28)) — PCB layout still TBD
- [x] [`Cartridge/Photos/`](Cartridge/Photos/) — cartridge card photo

### Chassis

Most content present (manuals, photos, firmware, datasheets, sounds). Disassembly guide: [`MechanicalManual/`](../MechanicalManual/).

**Completely missing** `[empty]`

- [ ] [`Chassis/Model3D/`](Chassis/Model3D/) — full robot body mechanical CAD ([#29](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/29))
- [ ] [`Chassis/KiCAD/`](Chassis/KiCAD/) — enclosure / mechanism drawings (README placeholder only) ([#30](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/30))

**Housekeeping**

- [x] Remove [`Chassis/Artwork/Null`](Chassis/Artwork/Null) (SVG logos present)
- [x] Remove [`Chassis/Sounds/Null.txt`](Chassis/Sounds/Null.txt) (`.wma` samples present)

---

## Planned modules (not started)

### Power `[planned]`

- [ ] KiCad project under [`Power/KiCAD/`](Power/KiCAD/) (currently README + `sym-lib-table` only) ([#31](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/31))
- [ ] Schematic source or scans ([#32](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/32))
- [ ] PCB layout and photos ([#33](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/33))
- [ ] Add power-module datasheets to [`DataSheets/`](DataSheets/) ([#34](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/34))

### Face `[planned]`

- [ ] KiCad project under [`Face/KiCAD/`](Face/KiCAD/) ([#35](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/35))
- [ ] Display / speech subsystem schematic ([#36](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/36))
- [ ] PCB layout and photos; add face/speech IC datasheets to [`DataSheets/`](DataSheets/) ([#37](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/37))

### PaddleMirror `[archive]`

**Completely missing** `[empty]`

- [ ] [`PaddleMirror/KiCAD/`](PaddleMirror/KiCAD/) — KiCad project ([#39](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/39))
- [ ] [`PaddleMirror/Model3D/`](PaddleMirror/Model3D/) — mechanical CAD ([#40](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/40))
- [x] [`PaddleMirror/Photos/`](PaddleMirror/Photos/) — accessory photo

---

## Repository hygiene

- [x] Remove orphan empty [`Accessories/`](Accessories/) (leftover after promoting Cartridge and PaddleMirror to top-level modules)
- [x] Photo policy: module photos live under each module's `Photos/` folder (removed `docs/photos/`)

---

## Tools and captures

- [ ] Annotate four empty per-button capture sidecars in [`tools/rfcap/captures/`](tools/rfcap/captures/) ([#41](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/41)):
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