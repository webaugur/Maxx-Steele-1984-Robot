# Shared KiCad libraries

Vendored [cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library) — COP400-series symbols used by hardware modules.

| File | Format | Notes |
|------|--------|-------|
| [`cop41xl.kicad_sym`](cop41xl.kicad_sym) | KiCad 10 | **Use this** — generated via `kicad-cli sym upgrade cop41xl.lib` |
| [`cop41xl.lib`](cop41xl.lib) | Legacy | Upstream source; kept for reference |
| [`cop41xl.dcm`](cop41xl.dcm) | Legacy | Datasheet links |

All `*/KiCAD/` projects load `cop41xl.kicad_sym` via `${KIPRJMOD}/../../libraries/` (or `../../../../libraries/` under `Cartridge/Examples/`).

| Module | Project |
|--------|---------|
| Transmitter | [`Transmitter/KiCAD/`](../Transmitter/KiCAD/) |
| Receiver | [`Receiver/KiCAD/`](../Receiver/KiCAD/) |
| Power | [`Power/KiCAD/`](../Power/KiCAD/) (planned) |
| Mainboard | [`Mainboard/KiCAD/`](../Mainboard/KiCAD/) (planned) |
| Face | [`Face/KiCAD/`](../Face/KiCAD/) (planned) |
| Cartridge (CBSDemo) | [`Cartridge/Examples/CBSDemo/KiCAD/`](../Cartridge/Examples/CBSDemo/KiCAD/) |
| Cartridge (UltraMaxx) | [`Cartridge/Examples/UltraMaxx/KiCAD/`](../Cartridge/Examples/UltraMaxx/KiCAD/) (planned) |
| PaddleMirror | [`PaddleMirror/KiCAD/`](../PaddleMirror/KiCAD/) (planned) |
| Chassis | [`Chassis/KiCAD/`](../Chassis/KiCAD/) (mechanical, planned) |