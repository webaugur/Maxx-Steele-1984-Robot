# KiCad hardware

All schematic and PCB projects for the Maxx Steele radio link.

| Project | Path | Description |
|---------|------|-------------|
| Transmitter | [`transmitter/`](transmitter/) | 27 MHz remote — COP411L MCU, RF keying |
| Receiver | [`receiver/`](receiver/) | Robot receiver module (work in progress) |
| Libraries | [`libraries/`](libraries/) | Shared [cop41xl](https://github.com/webaugur/cop41xl-kicad-library) symbols |

Open in KiCad:

- `kicad/transmitter/Transmitter-27MHz.pro`
- `kicad/receiver/Receiver-27MHz.pro`

Projects load symbols from `${KIPRJMOD}/../libraries/` (no per-project copies).