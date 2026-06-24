# KiCad libraries

## cop41xl-kicad-library

Remote source: [github.com/webaugur/cop41xl-kicad-library](https://github.com/webaugur/cop41xl-kicad-library)

COP400-series schematic symbols (COP411L, COP410L, COP413L, etc.) by David L Norris.

### Update the vendored copy

```bash
cd libraries/cop41xl-kicad-library
git pull
```

### Project wiring

Both KiCad projects reference this directory directly — no per-project copies of `cop41xl.lib` / `cop41xl.dcm`.

| Project | Path |
|---------|------|
| Transmitter | [`Transmitter/KiCAD/`](../Transmitter/KiCAD/) |
| Receiver | [`Receiver/KiCAD/`](../Receiver/KiCAD/) |

- `sym-lib-table` (KiCad 6+) — `${KIPRJMOD}/../../libraries/cop41xl-kicad-library/cop41xl.lib` (repo-root-relative from each `*/KiCAD/` project)
- `*.pro` (KiCad 5.x) — `LibDir1=${KIPRJMOD}/../../libraries/cop41xl-kicad-library`

Paths use KiCad's `${KIPRJMOD}` (project directory), not absolute filesystem paths.

Schematic symbol: `cop41xl:COP411L` (20-pin COP411L on the Maxx Steele remote MCU board).