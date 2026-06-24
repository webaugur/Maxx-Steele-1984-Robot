# TODO — Missing and incomplete archive content

Generated from an audit of empty directories, module READMEs, and KiCad project status (2026-06-23).

**Legend:** `[empty]` = directory has a README placeholder only (no real assets yet). `[partial]` = work started but not finished. `[planned]` = README marks module as not yet started.

---

## Summary

| Module | Status | Biggest gaps |
|--------|--------|--------------|
| Transmitter | **Active** | KiCad repair/fab; empty schematic/PCB/3D/photos folders |
| Receiver | **Active** | Schematic only — no PCB layout or fab outputs |
| Mainboard | Partial | Raster schematics; no KiCad project |
| Demo Cartridge | Partial | Firmware done; no hardware CAD |
| Chassis | Mostly complete | No 3D body model or mechanical KiCad |
| Power | Planned | No KiCad project |
| Face | Planned | No KiCad project |
| Paddle Mirror | Archive | No KiCad or 3D model |

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
- [ ] [`Transmitter/PC-Board/`](Transmitter/PC-Board/) — PCB photos, fab drawings, or layout exports
- [ ] [`Transmitter/3D-Model/`](Transmitter/3D-Model/) — enclosure / remote mechanical CAD
- [x] [`Transmitter/Photos/`](Transmitter/Photos/) — README pointer to [`docs/photos/transmitter/`](docs/photos/transmitter/)

**Housekeeping** (dirs have content; `null.txt` is stale)

- [x] Remove [`Transmitter/Datasheets/null.txt`](Transmitter/Datasheets/null.txt) (`COP411L.pdf` present)
- [x] Remove [`Transmitter/Stickers/null.txt`](Transmitter/Stickers/null.txt) (SVG/PDF artwork present)
- [x] Remove [`Transmitter/Reverse-Engineering/null.txt`](Transmitter/Reverse-Engineering/null.txt) (ODT/DOCX/PDF notes present)

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
- [ ] Receiver datasheets folder
- [ ] Receiver 3D model

---

## Partially started modules

### Mainboard

- [ ] Digitize [`Mainboard/Schematic/`](Mainboard/Schematic/) raster/SVG into KiCad — [`Mainboard/KiCAD/`](Mainboard/KiCAD/) is `[planned]` (README + `sym-lib-table` only)
- [ ] PCB layout project and fabrication artifacts
- [ ] Module-specific photos (chassis photos: [`Chassis/Photos/`](Chassis/Photos/))

### Demo Cartridge

Firmware is complete ([`Demo Cartridge/Firmware/`](Demo Cartridge/Firmware/)).

**Completely missing** `[empty]`

- [ ] [`Demo Cartridge/KiCAD/`](Demo Cartridge/KiCAD/) — cartridge PCB KiCad project
- [ ] [`Demo Cartridge/Schematic/`](Demo Cartridge/Schematic/) — schematic scan or source
- [ ] [`Demo Cartridge/PC-Board/`](Demo Cartridge/PC-Board/) — PCB layout / photos
- [ ] [`Demo Cartridge/3D-Model/`](Demo Cartridge/3D-Model/) — cartridge mechanical model
- [x] [`Demo Cartridge/Photos/`](Demo Cartridge/Photos/) — README pointer to [`docs/photos/accessories/demo-cartridge/`](docs/photos/accessories/demo-cartridge/)

### Chassis

Most content present (manuals, photos, firmware, datasheets, sounds, disassembly).

**Completely missing** `[empty]`

- [ ] [`Chassis/3D-Model/`](Chassis/3D-Model/) — full robot body mechanical CAD
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
- [ ] Power-module datasheets

### Face `[planned]`

- [ ] KiCad project under [`Face/KiCAD/`](Face/KiCAD/)
- [ ] Display / speech subsystem schematic
- [ ] PCB layout, photos, datasheets

### Paddle Mirror `[archive]`

**Completely missing** `[empty]`

- [ ] [`Paddle Mirror/KiCAD/`](Paddle Mirror/KiCAD/) — KiCad project
- [ ] [`Paddle Mirror/3D-Model/`](Paddle Mirror/3D-Model/) — mechanical CAD
- [x] [`Paddle Mirror/Photos/`](Paddle Mirror/Photos/) — README pointer to [`docs/photos/accessories/paddle-mirror/`](docs/photos/accessories/paddle-mirror/)

---

## Repository hygiene

- [x] Remove orphan empty [`Accessories/`](Accessories/) (leftover after promoting Demo Cartridge and Paddle Mirror to top-level modules)
- [x] Photo policy: canonical copies in [`docs/photos/`](docs/photos/); module `Photos/` folders are README pointers (documented in [`docs/photos/README.md`](docs/photos/README.md))

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
4. **Demo Cartridge** — hardware CAD (schematic, PCB, 3D)
5. **Chassis** — 3D body model and mechanical drawings
6. **Power / Face** — KiCad projects when boards are ready to document
7. ~~**Housekeeping**~~ — done (stale placeholders removed, `Accessories/` deleted, photo README pointers added)