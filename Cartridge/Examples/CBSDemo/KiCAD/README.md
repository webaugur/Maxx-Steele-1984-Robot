# CBSDemo — KiCad

Reverse-engineered schematic of the factory CBS demo cartridge PCB, derived from [`../../Photos/maxxcard.jpg`](../../Photos/maxxcard.jpg).

## Files

| File | Description |
|------|-------------|
| [`CBSDemo.pro`](CBSDemo.pro) | KiCad project (schematic-only; no PCB yet) |
| [`CBSDemo.sch`](CBSDemo.sch) | Schematic (legacy EESchema v4; KiCad 10 opens natively) |
| [`CBSDemo-schematic.pdf`](CBSDemo-schematic.pdf) | Exported schematic PDF |
| [`CBSDemo-cartridge-bom.csv`](CBSDemo-cartridge-bom.csv) | Bill of materials |
| [`trace-worksheet.md`](trace-worksheet.md) | Photo trace notes and confidence table |
| [`reference/`](reference/) | Enhanced and annotated PCB photos |
| [`../../Model3D/Maxx+Steele+Cartridge.stp`](../../Model3D/Maxx+Steele+Cartridge.stp) | Shell, PCB placeholder, 44-pos edge, chassis slot |

## Regenerate

```bash
python3 tools/gen_cartridge_sch.py
cd Cartridge/Examples/CBSDemo/KiCAD
kicad-cli sch export pdf CBSDemo.sch -o CBSDemo-schematic.pdf
```

## Board summary (as photographed)

| Refdes | Value | Notes |
|--------|-------|-------|
| **J1** | MaxxCard edge (**44 contacts**, 2.54 mm pitch; 22 per PCB face) | STEP + photo — see [`../../Model3D/`](../../Model3D/) |
| **U1** | **27C512** | 512 Kbit EPROM; demo uses 4 KB window at `$A000` |
| **U2** | **74HC14?** | 14-pin glue logic; exact P/N not readable |
| **U3** | **5085-TBD** | 24-pin IC with handwritten sticker; identity unknown |
| **C1–C3** | 0.1 µF | Decoupling (orange passives in photo) |

**EPROM note:** The photographed board uses a **27C512**. Community rebuilds may substitute a **KM2365** 4 KB EPROM once pin-compatible programming is confirmed — see [`../../../../DataSheets/Mitsubishi-KM2365.pdf`](../../../../DataSheets/Mitsubishi-KM2365.pdf) and [`../../PROGRAMMING.md`](../../PROGRAMMING.md).

## ERC status (rev 0.1)

The schematic loads in KiCad 10 and documents bus topology (address, data, control) between J1, U1, U3, and U2. **Pin-level connections remain provisional** because only the component-side photo is available. Expect ERC warnings until:

- Bottom-side copper photo or continuity checks
- Mainboard cartridge-slot pinout ([#22](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/22))
- Positive ID of the **5085** IC

Documented TBD nets are listed in [`trace-worksheet.md`](trace-worksheet.md).

## Open questions

1. What is the **5085** IC (custom mapper, latch, or date code)?
2. Full J1 finger map on both sides of the edge
3. Exact U2 part number from die/silkscreen under better magnification