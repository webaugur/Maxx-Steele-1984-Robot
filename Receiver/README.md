# Receiver

27 MHz receiver module in the Maxx Steele robot (superhet strip, OOK demodulation).

| Path | Description |
|------|-------------|
| [`KiCAD/`](KiCAD/) | KiCad schematic — [`Receiver-27MHz.pro`](KiCAD/Receiver-27MHz.pro) |

Shared symbols: [`libraries/`](../libraries/).

## TODO

Status: **Active** — schematic only; no PCB layout or fab outputs. Full backlog: [`TODO.md`](../TODO.md#receiver).

**KiCad (in progress)**

- [ ] Create `.kicad_pcb` from schematic
- [ ] Assign footprints for superhet / OOK strip
- [ ] Route board; generate `.net` and sync with schematic
- [ ] Run ERC and DRC
- [ ] Export Gerbers
- [ ] Add [`KiCAD/README.md`](KiCAD/README.md)
- [ ] Expand this README with BOM/architecture cross-links (mirror Transmitter)

**Not yet started**

- [ ] Schematic scans / photos (if separate from KiCad)
- [ ] PCB photos / layout archive
- [ ] Add receiver IC datasheets to [`DataSheets/`](../DataSheets/)
- [ ] 3D model