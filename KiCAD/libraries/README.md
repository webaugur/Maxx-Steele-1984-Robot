# KiCad libraries

## cop41xl-kicad-library

Remote source: [github.com/webaugur/cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library)

COP400-series schematic symbols (COP411L, COP410L, COP413L, etc.) by David L Norris.

### Update the vendored copy

```bash
cd KiCAD/libraries/cop41xl-kicad-library
git pull
cp cop41xl.lib cop41xl.dcm ../Receiver-27MHz/
```

### Project wiring

The transmitter project [`Receiver-27MHz/`](../Receiver-27MHz/) loads the library from the project directory:

- `cop41xl.lib` / `cop41xl.dcm` — copied from this repo (per upstream README)
- `sym-lib-table` — KiCad 6+ project library table
- `Transmitter-27MHz.pro` — `[eeschema/libraries]` entry for KiCad 5.x

Schematic symbol: `cop41xl:COP411L` (20-pin COP411L on the Maxx Steele remote MCU board).