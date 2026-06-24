# Cartridge 3D model

Mechanical assembly for the Maxx Steele program cartridge, including shell, mating slot reference, and internal placeholders for PCB layout.

## Files

| File | Description |
|------|-------------|
| [`Maxx+Steele+Cartridge.stp`](Maxx+Steele+Cartridge.stp) | Full cartridge assembly (STEP AP214, Autodesk export 2026-06-24) |
| [`Pasted image.png`](Pasted%20image.png) | Render of finished cartridge exterior (label + blue edge connector) |

## Assembly contents (from STEP)

| Part | Role |
|------|------|
| **Cartridge Front** / **Cartridge Back** | Injection-molded shell halves |
| **Window** | Label / graphics window in front shell |
| **PCB Placeholder** | Blank PCB solid for board outline and component clearance |
| **EDGE Card Connector 44pos Body** | Edge-connector housing |
| **EDGE Connector 2.54mm Contact Straight** (×44) | Individual 2.54 mm pitch edge fingers |
| **EDGE PCB Connector P2.54mm 44pin 2 Rows** | Full edge pattern (22 contacts per PCB side) |
| **DIP-28 W15.24mm EPROM** | Generic 28-pin EPROM envelope |
| **M27C256B EPROM 28pin. v1** | Fusion placeholder IC body |
| **Maxx Chassis Cart Slot** | Robot chassis slot reference for fit check |

## Key dimensions (for KiCad)

| Item | Value | Source |
|------|-------|--------|
| Edge connector | **44 positions**, **2.54 mm** pitch, **2 rows** (22 per PCB face) | STEP product names |
| EPROM package | DIP-28, **15.24 mm** row spacing | STEP + photo (`27C512` on real board) |
| Assembly bbox | ~116 × 143 × 41 mm (overall export envelope) | STEP Cartesian bounds |

The [`maxxcard.jpg`](../Photos/maxxcard.jpg) photo shows **22 gold fingers on the component side** only; the STEP model confirms the mating pattern is **44 contacts total** across both faces of the card edge.

## Use with KiCad

- Set PCB outline and keep-out from **PCB Placeholder** (import STEP into KiCad or measure in FreeCAD/Fusion).
- Use **Maxx Chassis Cart Slot** to verify insertion depth and shell clearance.
- Schematic edge symbol: [`Conn_01x44_Pin`](../Examples/CBSDemo/KiCAD/) (see [`CBSDemo.kicad_sch`](../Examples/CBSDemo/KiCAD/CBSDemo.kicad_sch)).

## Related

- Schematic (rev 0.1): [`../Examples/CBSDemo/KiCAD/`](../Examples/CBSDemo/KiCAD/)
- PCB photo: [`../Photos/maxxcard.jpg`](../Photos/maxxcard.jpg)