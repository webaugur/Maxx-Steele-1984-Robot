# Cartridge

4 KB program cartridge included with the Maxx Steele robot (demo program).

| Path | Description |
|------|-------------|
| [`PicoROM/`](PicoROM/) | **Primary cartridge schematic** — upstream PicoROM-28 module on the chosen J400 reference map. No external HCT. Not continuity-tested. |
| [`Examples/`](Examples/) | Example cartridges — ROM, KiCad ([`CBSDemo`](Examples/CBSDemo/), [`UltraMaxx`](Examples/UltraMaxx/)) |
| [`Model3D/`](Model3D/) | Mechanical model |
| [`Photos/`](Photos/) | Cartridge card photo |
| [`PROGRAMMING.md`](PROGRAMMING.md) | Cartridge programming manual (bytecode, speech, music) |
| [`Docs/Technical/`](../Docs/Technical/) | Full technical manual — [PDF](../Docs/Technical/Maxx-Steele-Technical-Manual.pdf) |

Shared KiCad symbols: [`libraries/`](../libraries/).

## TODO

Status: **Partial** — the primary schematic is an upstream PicoROM-28 module on the chosen J400 reference map. It is not continuity-tested and it is not a PCB. The bare-Pico split-GPIO sheet is not the design. Factory KiCad remains reference. Full backlog: [`TODO.md`](../TODO.md#cartridge). GitHub: [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Acartridge+label%3Abacklog).

Example firmware is complete ([`Examples/`](Examples/)).

- [x] [`Examples/CBSDemo/KiCAD/`](Examples/CBSDemo/KiCAD/) — CBSDemo cartridge schematic (rev 0.1; PCB TBD). Reference evidence, not the PicoROM carrier.
- [x] [`PicoROM/`](PicoROM/) — preliminary tscircuit schematic: upstream PicoROM-28, J400 reference map, no external buffer. Physical mating and robot timing remain open ([#46](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/46), [#53](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/53)). Not a PCB and not fab-ready ([#25](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/25), [#27](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/27)). Module choice is the PicoROM-28; stock was not rechecked ([#54](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/54)).
- [x] [`Examples/UltraMaxx/KiCAD/`](Examples/UltraMaxx/KiCAD/) — points to CBSDemo hardware
- [x] [`Examples/UltraMaxx/PICOROM.md`](Examples/UltraMaxx/PICOROM.md) — PicoROM P28 adaptation for U1 ([PicoROM](https://github.com/wickerwaka/PicoROM))
- [x] [`Model3D/`](Model3D/) — cartridge STEP assembly (shell + PCB placeholder + 44-pos edge)
- [x] [`Photos/`](Photos/) — cartridge card photo

Simulator GUI, in-place CBS program editing:

- [x] Edit the loaded program's opcode steps ([#43](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/43))
- [x] Save the edited program as a `.532` ([#44](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/44))
- [ ] Edit speech-phrase and music tables beside those steps ([#45](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/45))
