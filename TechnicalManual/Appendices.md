# Appendices

Reference tables for the Maxx Steele Programmer's Reference Guide.

---

## A — Opcode abbreviations

See [Quick reference](Quick-Reference.md) for the full hex table. Names match [`tools/maxx_rom.py`](../tools/maxx_rom.py) `OPCODES` dict.

---

## B — Display segment table

Internal ROM table **`$F878`** maps opcode indices to two-character LED segment patterns. Documented display names:

| Index | Display | Opcode context |
|-------|---------|----------------|
| — | L, F, b, r | Drive |
| — | Uu, Ud, Au, Ad, Cr, Cc | Joints |
| — | HL, init, d | Lamp, home, delay |
| — | PLAY, SPEE, SS, CLr | Extended |
| — | End, beg | `$FF`, `$FE` |

Full glyph bitmaps are embedded in [`maxx_internal_ROM.dsm`](../Chassis/Firmware/Assembly/maxx_internal_ROM.dsm) near label `FULL` / `$F878` references.

---

## C — Memory map

| Region | Address | Size | Purpose |
|--------|---------|------|---------|
| Zero page | `$0000–$00FF` | 256 B | Flags, pointers |
| Warm start | `$0100–$0103` | 4 B | Copyright mirror |
| Stack | `$01FF` ↓ | — | 6502 stack |
| Program RAM | `$0200–$03FE` | 510 B | Bytecode |
| Music RAM | `$0400–$04FF` | 256 B | Note pairs |
| Speech RAM | `$0500+` | — | Phrases for `$83` |
| I/O | `$1000` | — | Timer |
| Display | `$1200` | — | LED shift register |
| Speech | `$1400` | — | Phoneme parallel port |
| Motors | `$1600`, `$1C00` | — | Motor drive |
| Cartridge | `$2000–$B000` | 4K slots | Plug-in ROM |
| Internal ROM | `$E000–$FFFF` | 8 KB | OS / interpreter |

---

## D — Zero-page registers

| Addr | Name | Purpose |
|------|------|---------|
| `$02` | Status A | Speech error, backoff, power, drive flags |
| `$03` | Status B | User control, execute gating |
| `$0D` | Mode | 0=immediate, 1=learn, 2=program, 3=execute |
| `$0F`/`$10` | Program pointer | Step in `$0200` |
| `$11`/`$13` | Current op/operand | Entry state |
| `$24`/`$25` | Program byte pointer | Executor |

---

## E — Status bits

### `$02` (from [`tools/maxx_rom.py`](../tools/maxx_rom.py))

| Bit | Label |
|-----|-------|
| 0 | SEr (speech error) |
| 1 | Ebof (enable backoff) |
| 2 | PDon (power down on) |
| 3 | Edof (enable drive off) |
| 4 | SPon (speech on) |

Keypad toggles (internal ROM): row 0 → SEr, PAr; row 2 → Edon, Edof; row 3 → UCon, UCof; row 4 → Ebon, Ebof; row 5 → SPon, SPof.

### `$03`

| Bit | Label |
|-----|-------|
| 0 | UCon (user control on) |
| 1 | SPon (speech enable) |

Demo bootstrap: `$02` ← `$02`, `$03` ← `$82`.

---

## F — Music duration table

Note durations for the music IRQ path are read from **`$F15B`** (indexed 1–8 in [`maxx_internal_ROM.dsm`](../Chassis/Firmware/Assembly/maxx_internal_ROM.dsm)). Frequency/note bytes live in **`$0400`**.

Complete note name → byte mapping is not yet transcribed in this archive.

---

## G — Cartridge file layout

File offset = cartridge address − `$A000` for demo cart base.

| File offset | Addr | Content |
|-------------|------|---------|
| `$0000` | `$A000` | Entry vector |
| `$0002` | `$A002` | Copyright (17 bytes) |
| `$0013` | `$A013` | Bootstrap code |
| `$0035` | `$A035` | Program table |
| `$0081` | `$A081` | Phrase table |
| `$00BB` | `$A0BB` | Music table |

---

## H — [`tools/maxx_rom.py`](../tools/maxx_rom.py) commands

```bash
python3 tools/maxx_rom.py disasm PATH [--compare-dsm DSM]
python3 tools/maxx_rom.py validate PATH
python3 tools/maxx_rom.py template OUTPUT.532
python3 tools/maxx_rom.py opcodes OUTPUT.json
```

---

## I — Internal ROM entry points

Partial list from [`maxx_internal_ROM.dsm`](../Chassis/Firmware/Assembly/maxx_internal_ROM.dsm):

| Address | Role |
|---------|------|
| `$E0B6` | Post-bootstrap main loop |
| `$E0D1` | Mode / executor loop |
| `$E6B5` | Keycode table base |
| `$EF01` | Music tune pointer helper |
| `$F1D8` | Music IRQ region |
| `$F3D5` | Speech output |
| `$F878` | Display pattern table |

---

## J — Messages and errors

| Message / flag | Cause |
|----------------|-------|
| **FULL** | Program exceeds `$037F` / RAM limit |
| **SEr** | Speech error status bit |
| `validate` copyright mismatch | Cart missing `(c) 1985 CBS Toys` |
| Missing `FF FF` | Program table not terminated |

---

## K — Bibliography

- R. Wind — [`maxx_internal_ROM.dsm`](../Chassis/Firmware/Assembly/maxx_internal_ROM.dsm), [`maxx_demo_ROM_532.dsm`](../Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm) (2002–2006)
- Factory manual — [`Chassis/Manual/MaxxSteeleReferenceGuide.pdf`](../Chassis/Manual/MaxxSteeleReferenceGuide.pdf)
- This repository — https://github.com/webaugur/Maxx-Steele-1984-Robot
- C64 Programmer's Reference Guide — structural inspiration (Commodore, 1982)

---

## L — Glossary

| Term | Meaning |
|------|---------|
| Bytecode | Opcode/operand pairs interpreted by internal ROM |
| Bootstrap | Short 6502 stub in cartridge that loads RAM tables |
| OOK | On-off keying RF modulation (27 MHz) |
| IF | Intermediate frequency (455 kHz in transmitter/receiver) |
| Phoneme | Speech sound unit; 4-bit codes to `$1400` |
| Refdes | Schematic reference designator (U1, U400, …) |
| `.532` | 4 KB cartridge image file extension used in archive |