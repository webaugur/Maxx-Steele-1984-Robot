# KiCad libraries

## cop41xl-kicad-library

Remote source: [github.com/webaugur/cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library)

COP400-series schematic symbols (COP411L, COP410L, COP413L, etc.) by David L Norris.

### Update the vendored copy

```bash
cd libraries/cop41xl-kicad-library
git pull
cp cop41xl.lib cop41xl.dcm ../../Transmitter/KiCAD/
cp cop41xl.lib cop41xl.dcm ../../Receiver/KiCAD/
```

### Project wiring

Both KiCad projects load the library from the project directory (KiCad 5.x) and via `sym-lib-table` (KiCad 6+):

| Project | Path |
|---------|------|
| Transmitter | [`Transmitter/KiCAD/`](../Transmitter/KiCAD/) |
| Receiver | [`Receiver/KiCAD/`](../Receiver/KiCAD/) |

- `cop41xl.lib` / `cop41xl.dcm` — copied from this repo (per upstream README)
- `sym-lib-table` — points at `${KIPRJMOD}/../../libraries/cop41xl-kicad-library/`
- `*.pro` — `[eeschema/libraries]` entry for KiCad 5.x

Schematic symbol: `cop41xl:COP411L` (20-pin COP411L on the Maxx Steele remote MCU board).