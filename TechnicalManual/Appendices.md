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

Full glyph bitmaps are embedded in [`maxx_internal_ROM.dsm`](../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm) near label `FULL` / `$F878` references.

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

### Status and mode

| Addr | Name | Purpose |
|------|------|---------|
| `$02` | Status A | Speech error, backoff, power, drive flags (Appendix E) |
| `$03` | Status B | User control, speech enable, execute gating |
| `$04` | Reveille flag | Set by clock path; cleared at `$EE32` |
| `$0D` | Mode | 0=immediate, 1=learn, 2=program, 3=execute, **4=game** |
| `$0E` | Entry state | Non-zero during multi-key opcode entry |
| `$0F`/`$10` | Program pointer | Address of current step in `$0200` (beg = `$0180`) |
| `$11`/`$13` | Current op/operand | Fetched by `$E519` for executor |
| `$15` | Keycode | Last decoded keypad value from `$E617` |
| `$24`/`$25` | Executor pointer | Working address; displayed for extended opcodes |
| `$2B` | Talkback busy | Non-zero while motors/speech pending; `$EF63` waits |
| `$26` | Timer prescale | IRQ countdown; reset to `$F4` at `$E99` |
| `$27` | Second timer | Game delays, enter-key timeout |
| `$39`–`$3C` | Music state | Pointers/flags during note entry (`$EF76` region) |
| `$56` | IRQ phase | Selects handler in IRQ jump table |
| `$57` | Speech timing | Used during `$1400` nybble output |
| `$5B` | Speech status | Error/ready flags for LC8100 path |
| `$67`–`$6A` | Motor timing | Decremented in IRQ; init to `$FF` at warm start |
| `$72`–`$96` | Vector table | Copied from ROM `$E01C` at boot (see below) |
| `$74` | Tune index | Last PLAY operand; power-down tune selection |
| `$75` | Keypad buffer | Bit 7 = key ready; `$E600` waits for match |
| `$8E`/`$8F` | Cart pointer | 16-bit cartridge scan address (page in `$8F`) |
| `$96` | Energy counter | Decremented in IRQ; triggers low-battery speech |

### Vector table (`$72`–`$96`, from ROM `$E01C`)

| ZP | Initial target | Role |
|----|----------------|------|
| `$72`/`$73` | `$0082` | Reserved / table header |
| `$76`/`$77` | `$E014` | NMI vector |
| `$78`/`$79` | `$FDC8` | IRQ vector |
| `$7A`/`$7B` | `$E0CB` | Return to main loop (immediate) |
| `$7C`–`$85` | `$F1D8`… | Music IRQ dispatch table |
| `$8C`/`$8D` | `$EE32` | Opcode dispatch (`$EE2F`) |
| `$8E`/`$8F` | `$1000` | Cartridge entry pointer |
| `$90`/`$91` | `$E598` | Alternate handler vector |
| `$92`/`$93` | `$F3D8` | Speech output vector |
| `$94`/`$95` | `$F8CE` | Game mode entry |

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

Note durations for the music IRQ path are read from **`$F15B`** (indexed 1–8 in [`maxx_internal_ROM.dsm`](../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm)). Frequency/note bytes live in **`$0400`**.

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

Curated catalog from [`maxx_internal_ROM.dsm`](../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm). See [Chapter 5](05-Cartridge-Bootstrap-and-Internal-ROM.md) for boot flow and mode diagrams.

### Boot and initialization

| Address | Role |
|---------|------|
| `$E041` | RESET entry |
| `$E05A` | Cold start — zero page, clear RAM tables |
| `$E06F` | Warm start — copy vectors `$E01C` → `$72` |
| `$E08E`–`$E0B3` | Cartridge scan and `JMP ($008E)` |
| `$E574` | Init learn pointer, `$0200` ← `$FF` |
| `$E582` | Init program pointers (cold start) |
| `$E57B` | Set `$0180` ← `$FF` terminator |
| `$EEEE` | Timer byte init |

### Main loop and modes

| Address | Role |
|---------|------|
| `$E0B6` | Main loop entry (post-cart or no cart) |
| `$E0D1` | Mode change — `STA $0D`, re-dispatch |
| `$E0ED` | Mode dispatch on `$0D` |
| `$E154` | Immediate mode entry |
| `$E161` | Learn/immediate keypad loop |
| `$E17B` | Immediate key decode (key in A) |
| `$E346` | Program mode loop |
| `$E434` | Execute mode runner |
| `$E905` | Status housekeeping (called each loop) |
| `$F8CE` | Game mode entry (`JMP ($0094)`) |
| `$F8E9` | Game 1 main |
| `$FAA4` | Game 2 main |

### Keypad and program I/O

| Address | Role |
|---------|------|
| `$E3A9` | CLEAR program function |
| `$E3EC` | Delay / wait for flag change |
| `$E409` | RUN preview (ENTER in program mode) |
| `$E41A` | Increment program pointer (+2) |
| `$E519` | Fetch opcode/operand to `$11`/`$13` |
| `$E555` | Decrement pointer (BEGIN) |
| `$E600` | Wait until `$75` matches A |
| `$E617` | Poll keypad → `$15` |
| `$E6A4` | Key poll wrapper (motors, RF) |
| `$E6B5` | Keycode → opcode translation table |
| `$E9FE` | Keypad input helper (games, music entry) |

### Bytecode executor

| Address | Role |
|---------|------|
| `$EE2F` | `JMP ($008C)` — opcode dispatch |
| `$EE32` | Dispatch entry (Reveille, execute paths) |
| `$F64E` | Program step number → display |
| `$F66C` | Execute one program command |
| `$F737` | Compute step number from `$0F`/`$10` |

### Display

| Address | Role |
|---------|------|
| `$ED48` | Display prefix |
| `$ED4F` | Shift LO bit to `$1200` |
| `$ED7B` | Send byte to display |
| `$EDAF` | Stalled-motor check |
| `$F684` | Send 2-char string to display |
| `$F878` | Opcode → segment pattern table |

### Speech

| Address | Role |
|---------|------|
| `$F3D5` | Speech output entry (`JMP ($0092)`) |
| `$F3D8` | Phoneme output (index X) |
| `$F40F` | Say ROM phrase by index (X) |
| `$F460` / `$F465` | Clock nybbles to `$1400` |
| `$F475` | Say built-in phrase (index X) |
| `$F4DB` | Phoneme duration table (140 entries) |
| `$F567` | Phoneme code table |

### Music

| Address | Role |
|---------|------|
| `$EF01` | Resolve tune pointer for operand |
| `$EF55` | Clear `$0400` music table |
| `$EF76` | Leave learn / music cleanup |
| `$F0B8` | Append note to `$0400` |
| `$F15B` | Note duration table |
| `$F1D8` | Music IRQ handler region |

### Motors

| Address | Role |
|---------|------|
| `$EF2E` | Motor drive command |
| `$EF40` | Stop motors |
| `$EF63` | Wait for talkback (`$2B` = 0) |

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

- R. Wind — [`maxx_internal_ROM.dsm`](../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm), [`maxx_demo_ROM_532.dsm`](../Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm) (2002–2006)
- Factory manual — [`Chassis/Manual/MaxxSteeleReferenceGuide.pdf`](../Chassis/Manual/MaxxSteeleReferenceGuide.pdf)
- This repository — https://github.com/webaugur/Maxx-Steele-1984-Robot
- C64 Programmer's Reference Guide — structural inspiration (Commodore, 1982)

---

## M — MMIO hardware cross-reference

Partial pin map and IC inventory (provisional decode). Full detail: [`Mainboard/Schematic/MMIO-Pin-Map.md`](../Mainboard/Schematic/MMIO-Pin-Map.md).

| MMIO | Peripheral | Key ROM drivers | Likely IC |
|------|------------|-----------------|-----------|
| `$1000` | Timer / display handshake | `$EEEE`, `$ED5F`–`$EDA0` | Counter TBD |
| `$1200` | LED shift register | `$ED4F`, `$ED7B`, `$F684` | COP41xL U500 |
| `$1400` | Speech nybbles + strobe | `$F3D8`, `$F460` | LC8100 / ET9420 |
| `$1600` | Motor aux / music attack | `$F21C`, `$F14B` | COP41xL / COP420 |
| `$1C00` | Music tone period | `$F207`, `$F21C` | Audio path |
| `$1E00` | Power latch | `$E041`, `$E07B` | Glue logic |

ROM access listing: [`MMIO-ROM-Crossref.md`](../Mainboard/Schematic/MMIO-ROM-Crossref.md) (`tools/gen_mmio_crossref.py`).

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