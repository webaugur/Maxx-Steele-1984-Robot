# Cartridge

4 KB program cartridge included with the Maxx Steele robot (demo program).

| Path | Description |
|------|-------------|
| [`PicoROM/`](PicoROM/) | **Primary cartridge schematic** — Raspberry Pi Pico plus DIP 245 buffers, wired to the chosen J400 reference map. Not continuity-tested. P28 sheet is reference only. |
| [`Examples/`](Examples/) | Example cartridges — ROM, KiCad ([`CBSDemo`](Examples/CBSDemo/), [`UltraMaxx`](Examples/UltraMaxx/)) |
| [`Model3D/`](Model3D/) | Mechanical model |
| [`Photos/`](Photos/) | Cartridge card photo |
| [`PROGRAMMING.md`](PROGRAMMING.md) | Cartridge programming manual (bytecode, speech, music) |
| [`Docs/Technical/`](../Docs/Technical/) | Full technical manual — [PDF](../Docs/Technical/Maxx-Steele-Technical-Manual.pdf) |

Shared KiCad symbols: [`libraries/`](../libraries/).

## TODO

Status: **Partial** — firmware done; Pico plus DIP 245 schematic is wired to the chosen J400 reference map and is still not continuity-tested or a PCB; the P28 sheet stays unwired; factory KiCad remains reference. Full backlog: [`TODO.md`](../TODO.md#cartridge). GitHub: [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Acartridge+label%3Abacklog).

Example firmware is complete ([`Examples/`](Examples/)).

- [x] [`Examples/CBSDemo/KiCAD/`](Examples/CBSDemo/KiCAD/) — CBSDemo cartridge schematic (rev 0.1; PCB TBD). Reference evidence, not the PicoROM carrier.
- [x] [`PicoROM/`](PicoROM/) — preliminary tscircuit schematic: Pico RP2040, DIP 245 buffers, J400 reference map. Physical mating and voltages remain open ([#46](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/46)). Not a PCB and not fab-ready ([#25](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/25), [#27](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/27)).
- [x] [`Examples/UltraMaxx/KiCAD/`](Examples/UltraMaxx/KiCAD/) — points to CBSDemo hardware
- [x] [`Examples/UltraMaxx/PICOROM.md`](Examples/UltraMaxx/PICOROM.md) — PicoROM P28 adaptation for U1 ([PicoROM](https://github.com/wickerwaka/PicoROM))
- [x] [`Model3D/`](Model3D/) — cartridge STEP assembly (shell + PCB placeholder + 44-pos edge)
- [x] [`Photos/`](Photos/) — cartridge card photo

Simulator GUI, in-place CBS program editing:

- [ ] Edit the loaded program's opcode steps ([#43](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/43))
- [ ] Save the edited program as a `.532` ([#44](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/44))
- [ ] Edit speech-phrase and music tables beside those steps ([#45](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/45))
