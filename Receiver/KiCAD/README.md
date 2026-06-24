# Receiver — KiCad 10

27 MHz receiver schematic.

## Open in KiCad 10

**Open [`Receiver-27MHz.kicad_pro`](Receiver-27MHz.kicad_pro)** — not the legacy `.pro` / `.sch` pair (those expect a generated `*-cache.lib`).

| File | Description |
|------|-------------|
| [`Receiver-27MHz.kicad_pro`](Receiver-27MHz.kicad_pro) | KiCad 10 project |
| [`Receiver-27MHz.kicad_sch`](Receiver-27MHz.kicad_sch) | Native schematic (symbols embedded) |
| [`Receiver-27MHz-schematic.pdf`](Receiver-27MHz-schematic.pdf) | Exported schematic PDF |
| [`Receiver-27MHz.sch`](Receiver-27MHz.sch) | Legacy schematic (kept for reference) |
| [`Receiver-27MHz.pro`](Receiver-27MHz.pro) | Legacy project (superseded) |

## Regenerate native files

```bash
python3 tools/upgrade_legacy_sch.py Receiver/KiCAD/Receiver-27MHz.sch
```

## ERC

```bash
kicad-cli sch erc Receiver/KiCAD/Receiver-27MHz.kicad_sch
```

The upgraded schematic loads and runs ERC in KiCad 10. Remaining violations are from the original design — resolve during schematic review.