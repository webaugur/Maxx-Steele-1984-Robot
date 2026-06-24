# Transmitter — KiCad 10

27 MHz remote transmitter schematic and PCB.

## Open in KiCad 10

**Open [`Transmitter-27MHz.kicad_pro`](Transmitter-27MHz.kicad_pro)** — not the legacy `.pro` / `.sch` pair (those expect a generated `*-cache.lib`).

| File | Description |
|------|-------------|
| [`Transmitter-27MHz.kicad_pro`](Transmitter-27MHz.kicad_pro) | KiCad 10 project |
| [`Transmitter-27MHz.kicad_sch`](Transmitter-27MHz.kicad_sch) | Native schematic (symbols embedded) |
| [`Transmitter-27MHz.kicad_pcb`](Transmitter-27MHz.kicad_pcb) | PCB layout (upgraded to KiCad 10 format) |
| [`Transmitter-27MHz-schematic.pdf`](Transmitter-27MHz-schematic.pdf) | Exported schematic PDF |
| [`Transmitter-27MHz.sch`](Transmitter-27MHz.sch) | Legacy schematic (kept for reference) |
| [`Transmitter-27MHz.pro`](Transmitter-27MHz.pro) | Legacy project (superseded) |

## Regenerate native files

```bash
python3 tools/upgrade_legacy_sch.py Transmitter/KiCAD/Transmitter-27MHz.sch
kicad-cli pcb upgrade Transmitter/KiCAD/Transmitter-27MHz.kicad_pcb
```

## ERC / DRC

```bash
kicad-cli sch erc Transmitter/KiCAD/Transmitter-27MHz.kicad_sch
kicad-cli pcb drc Transmitter/KiCAD/Transmitter-27MHz.kicad_pcb
```

The upgraded schematic loads and runs ERC in KiCad 10. Remaining violations are from the original design (e.g. undriven nets, footprint filters) — resolve during schematic/PCB sync work.