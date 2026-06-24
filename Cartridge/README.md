# Cartridge

4 KB program cartridge included with the Maxx Steele robot (demo program).

| Path | Description |
|------|-------------|
| [`Examples/`](Examples/) | Example cartridges — ROM, KiCad ([`CBSDemo`](Examples/CBSDemo/), [`UltraMaxx`](Examples/UltraMaxx/)) |
| [`Model3D/`](Model3D/) | Mechanical model |
| [`Photos/`](Photos/) | Cartridge card photo |
| [`PROGRAMMING.md`](PROGRAMMING.md) | Cartridge programming manual (bytecode, speech, music) |
| [`TechnicalManual/`](../TechnicalManual/) | Full technical manual — [PDF](../TechnicalManual/Maxx-Steele-Technical-Manual.pdf) |

Shared KiCad symbols: [`libraries/`](../libraries/).

## TODO

Status: **Partial** — firmware and CBSDemo schematic done; PCB layout not started. Full backlog: [`TODO.md`](../TODO.md#cartridge). GitHub: [open issues](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues?q=is%3Aissue+is%3Aopen+label%3Acartridge+label%3Abacklog).

Example firmware is complete ([`Examples/`](Examples/)).

- [x] [`Examples/CBSDemo/KiCAD/`](Examples/CBSDemo/KiCAD/) — CBSDemo cartridge schematic (rev 0.1; PCB TBD)
- [x] [`Examples/UltraMaxx/KiCAD/`](Examples/UltraMaxx/KiCAD/) — points to CBSDemo hardware
- [x] [`Examples/UltraMaxx/PICOROM.md`](Examples/UltraMaxx/PICOROM.md) — PicoROM P28 adaptation for U1 ([PicoROM](https://github.com/wickerwaka/PicoROM))
- [x] [`Model3D/`](Model3D/) — cartridge STEP assembly (shell + PCB placeholder + 44-pos edge)
- [x] [`Photos/`](Photos/) — cartridge card photo