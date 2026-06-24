# Chapter 5 — Cartridge Bootstrap and Internal ROM

This chapter explains the relationship between **6502 machine code** in cartridge ROM and the **bytecode interpreter** in the robot's 8 KB internal ROM.

---

## What is the internal ROM?

At power-on the **MOS 6502** CPU executes firmware in **`$E000`–`FFFF`** (8 KB). This code is the robot's only native language — analogous to the C64 KERNAL + BASIC ROM, but specialized for:

- Keypad scan and mode handling
- Bytecode fetch/decode from `$0200`
- Speech, music, motor, and display drivers
- Cartridge detection and bootstrap handoff

User programs are **not** generally written in 6502. They are bytecode tables interpreted by this ROM.

Source listing: [`Chassis/Firmware/Assembly/maxx_internal_ROM.dsm`](../Chassis/Firmware/Assembly/maxx_internal_ROM.dsm)  
Binary: [`Chassis/Firmware/Binary/Maxxrom.64`](../Chassis/Firmware/Binary/Maxxrom.64)

---

## What is a cartridge?

A **4 KB EPROM** plugs into the cartridge slot, mapped into **`$2000`–`B000`** in 4 KB steps. The factory demo sits at **`$A000`**.

A valid image supplies:

1. Entry vector and copyright header
2. A short **6502 bootstrap stub**
3. Program, speech, and music **data tables**

The stub copies tables into RAM and jumps to the internal main loop — it does **not** replace the interpreter.

---

## Cartridge ROM layout (`$A000` base)

| Offset | Size | Content |
|--------|------|---------|
| `$00` | 2 | Entry vector (word), e.g. `$A013` |
| `$02` | 17 | Copyright: `(c) 1985 CBS Toys` |
| `$13` | code | Bootstrap stub (6502) |
| `$35` | table | Program bytecode |
| `$81` | table | Speech phrases (opcode `$83`) |
| `$BB` | table | Music notes (opcode `$81`) |

Cartridge detection scans 4 KB boundaries from `$2000` upward, comparing the copyright string at offset +2.

---

## Bootstrap stub (demo cart)

Entry at **`$A013`** ([`maxx_demo_ROM_532.dsm`](../Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm)):

1. `STA $02` / `STA $03` — initialize status flags
2. Copy program from **`$A035`** → **`$0200`**
3. Copy phrases from **`$A081`** → **`$0500`**
4. Copy music from **`$A0BB`** → **`$0400`**
5. **`JMP $E0B6`** — enter internal ROM main loop

Expected entry prologue: `A9 02 85 02` (LDA #$02 / STA $02). [`tools/maxx_rom.py validate`](../tools/maxx_rom.py) checks this.

---

## Key internal ROM addresses

| Address | Role |
|---------|------|
| `$E0B6` | Main loop entry after cartridge bootstrap |
| `$E0D1` | Alternate loop / mode handler (`JMP $E0D1` sites) |
| `$E6B5` | Keycode → opcode translation table |
| `$F878` | Opcode → LED segment display table |
| `$F3D5` | Speech output subroutine |
| `$F1D8` | Music IRQ handler region |
| `$EF01` | Tune pointer resolver (music) |
| `$F15B` | Note duration table |

---

## 6502 vs bytecode — when to use which

| Task | Language |
|------|----------|
| Motion/speech demo sequence | Bytecode in `$0200` |
| Custom phrases/music in cart | Data tables + bootstrap |
| Patch OS behavior | 6502 patch (advanced RE only) |
| New cartridge program | Bootstrap stub (minimal 6502) + bytecode tables |

Do not place arbitrary 6502 execution paths in the demo program area unless you fully understand cartridge mapping and internal ROM expectations.

---

## Tools and authoring

Step-by-step cartridge creation: [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) §9.

```bash
python3 tools/maxx_rom.py template mycart.532
python3 tools/maxx_rom.py validate mycart.532
python3 tools/maxx_rom.py disasm mycart.532 --compare-dsm Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm
```

EPROM type: Mitsubishi KM2365 family — [`DataSheets/Mitsubishi-KM2365.pdf`](../DataSheets/Mitsubishi-KM2365.pdf).

---

## Known gaps

- Full subroutine catalog for `$E000`–`FFFF` not yet indexed (see [Appendix I](Appendices.md#i-internal-rom-entry-points)).
- Padding at `$A0C7+` in demo image marked "not Maxx-related" in `.dsm` — treat as unused.

---

**Previous:** [Chapter 4](04-Programming-Speech-and-Music.md) · **Next:** [Chapter 6 — I/O guide](06-Input-Output-Guide.md)