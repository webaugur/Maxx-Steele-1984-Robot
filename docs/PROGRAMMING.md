# Maxx Steele Cartridge Programming Manual

This document describes how to write programs for the 1984 CBS Toys / Ideal **Maxx Steele** robot using plug-in ROM cartridges. It is derived from:

- [`Demo Cartridge/Firmware/Assembly/maxx_demo_ROM_532.dsm`](../Demo%20Cartridge/Firmware/Assembly/maxx_demo_ROM_532.dsm) — annotated demo cartridge (R. Wind)
- [`Chassis/Firmware/Assembly/maxx_internal_ROM.dsm`](../Chassis/Firmware/Assembly/maxx_internal_ROM.dsm) — internal 6502 interpreter (R. Wind)
- [`Demo Cartridge/Firmware/Binary/MAXXCART.532`](../Demo%20Cartridge/Firmware/Binary/MAXXCART.532) — 4 KB demo cartridge binary
- Original reference guide: [`Chassis/Manual/MaxxSteeleReferenceGuide.pdf`](../Chassis/Manual/MaxxSteeleReferenceGuide.pdf)

Tools live in [`tools/maxx_rom.py`](../tools/maxx_rom.py).

---

## 1. System overview

The robot is controlled by a **6502** CPU with 8 KB internal ROM (`$E000–$FFFF`). User programs are stored on **4 KB cartridge EPROMs** mapped at `$2000–$B000` in 4 KB steps. The factory demo cartridge sits at **`$A000`**.

The interpreter executes a bytecode program stored in RAM. Cartridges do not run arbitrary 6502 code for the demo sequence — they supply a **bootstrap stub** that copies tables into RAM and returns to the internal ROM main loop.

### Operating modes (`$0D`)

| Value | Mode | Behavior |
|-------|------|----------|
| 0 | Immediate | Keys execute instantly |
| 1 | Learn | Record key sequences |
| 2 | Program | Enter bytecode steps via keypad |
| 3 | Execute / Game | Run stored program |

---

## 2. Memory map

| Region | Address | Size | Purpose |
|--------|---------|------|---------|
| Zero page | `$0000–$00FF` | 256 B | Flags, pointers, keypad state |
| Warm start | `$0100–$0103` | 4 B | Copyright mirror for warm boot |
| Stack | `$01FF` ↓ | — | 6502 stack |
| **Program RAM** | `$0200–$03FE` | 510 B | **2 bytes per command** |
| **Music RAM** | `$0400–$04FF` | 256 B | 2 bytes per note |
| **Speech RAM** | `$0500+` | — | Custom phrase data for opcode `$83` |
| I/O | `$1000` | — | Timer / misc |
| Display | `$1200` | — | Shift-register LED display |
| Speech chip | `$1400` | — | Parallel phoneme output |
| Motors | `$1600`, `$1C00` | — | Motor timing / drive |
| Cartridge ROM | `$2000–$B000` | 4K slots | Plug-in programs |
| Internal ROM | `$E000–$FFFF` | 8 KB | Operating system |

### Key zero-page registers

| Addr | Name | Purpose |
|------|------|---------|
| `$02` | Status A | Speech error, backoff, power-down, drive, speech flags |
| `$03` | Status B | User control, speech enable, execute gating |
| `$0D` | Mode | 0=immediate, 1=learn, 2=program, 3=execute |
| `$0F/$10` | Program pointer | Current step in `$0200` table |
| `$11/$13` | Current opcode/operand | During entry and execution |
| `$24/$25` | Program byte pointer | Used by executor |

#### Status byte `$02` (set by cartridge entry stub)

Demo cartridge entry uses `LDA #$02` / `STA $02` → **SEr, Ebof** (speech error + enable backoff).

#### Status byte `$03`

Demo entry uses `LDA #$82` / `STA $03` → **PDon, Edof, SPof, UCon**.

Toggle meanings (from internal ROM keypad handler):

| Key row | Toggles |
|---------|---------|
| 0 | SEr, PAr |
| 2 | Edon, Edof |
| 3 | UCon, UCof |
| 4 | Ebon, Ebof |
| 5 | SPon, SPof |

---

## 3. Cartridge ROM layout

A valid 4 KB image (`.532`) for address `$A000`:

```
$A000 + $00:  word   entry vector (e.g. $A013)
$A000 + $02:  bytes  copyright — must be "(c) 1985 CBS Toys" (17 bytes)
$A000 + $13:  code   bootstrap stub (6502)
$A000 + $35:  table  program bytecode
$A000 + $81:  table  speech phrases (for opcode $83)
$A000 + $BB:  table  music notes (for opcode $81)
```

### Bootstrap stub (demo cart)

The demo entry at `$A013` performs:

1. `STA $02` / `STA $03` — initialize status flags
2. Copy program bytes from `$A035` → `$0200`
3. Copy phrase bytes from `$A081` → `$0500`
4. Copy music bytes from `$A0BB` → `$0400`
5. `JMP $E0B6` — return to internal ROM

Cartridge detection scans 4 KB boundaries from `$2000` upward, comparing the copyright string at offset +2 against internal ROM expectations.

---

## 4. Program bytecode format

Programs are a sequence of **(opcode, operand)** byte pairs stored in RAM at `$0200`. The program **must** end with:

```
FF FF
```

Maximum program length is limited by RAM (`$0200–$03FE` → 255 command pairs). The internal ROM displays **FULL** when `$037F` is exceeded.

### Motion and I/O opcodes (`$00–$0F`)

Display names are taken from the internal ROM LED segment table at `$F878`.

| Opcode | Display | Name | Operand |
|--------|---------|------|---------|
| `$00` | L | Turn / drive left | Distance or angle |
| `$01` | F | Drive forward | Distance |
| `$02` | b | Drive reverse (back up) | Distance |
| `$03` | r | Turn / drive right | Angle |
| `$04` | Uu | Wrist up | Value |
| `$05` | Ud | Wrist down | Value |
| `$06` | Au | Arms up | Value |
| `$07` | Ad | Arms down | Value |
| `$08` | Cr | Claw rotate | Value |
| `$09` | Cc | Claw close / open | `0`=open, `1`=close |
| `$0A` | HL | Lamp | `0`=off, `1`=on |
| `$0B` | init | Home (all joints) | Must be `$00` |
| `$0C` | d | Delay | Seconds |
| `$0D` | Sn | Song number reference | Index |
| `$0E` | S | Speech (ROM phrase) | Phrase # |
| `$0F` | SS | Speech (shift mode) | Phrase # |
| `$10` | PS | Program speech capture | Slot # |

### Extended opcodes (`$80+`)

Opcodes with bit 7 set are remapped to display table entries `$0C–$13` in the executor.

| Opcode | Display | Name | Operand |
|--------|---------|------|---------|
| `$81` | PLAY | Play tune from music RAM | Tune # (`$0400` table) |
| `$82` | SPEE | Speak ROM phrase | Phrase # |
| `$83` | SS | Speak RAM phrase | Phrase # in `$0500` table |
| `$84` | CLr | Clear speech phrase slot | Slot # |
| `$FE` | beg | Program begin marker | Display only |
| `$FF` | End | End of program | Must be `$FF` |

### Special keypad-mapped opcodes

Internal ROM keycode table at `$E6B5` maps keypad scan codes to opcodes:

```
00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
41 46 43 13 84 82 81 83 80
```

---

## 5. Demo cartridge walkthrough

Source: [`Demo Cartridge/Firmware/Assembly/maxx_demo_ROM_532.dsm`](../Demo%20Cartridge/Firmware/Assembly/maxx_demo_ROM_532.dsm)

| Step | Bytes | Action |
|------|-------|--------|
| 1 | `83 10` | Speak RAM phrase 0 — "Hello, I am Maxx Steele" |
| 2 | `0C 02` | Delay 2 seconds |
| 3 | `83 00` | Speak RAM phrase — "I am great, and you" |
| 4 | `0C 0A` | Delay 10 seconds |
| 5 | `83 16` | "Good morning" (ROM phrase) |
| 6 | `0C 01` | Delay 1 second |
| 7 | `83 17` | "It is time to get up" |
| 8 | `0C 01` | Delay 1 second |
| 9 | `81 06` | Play tune #6 (Reveille) |
| 10 | `0C 0A` | Delay 10 seconds |
| 11 | `83 01` | "I am ready when you are" |
| 12 | `0C 01` | Delay 1 second |
| 13 | `01 14` | Move forward |
| 14 | `0C 04` | Delay 4 seconds |
| 15 | `06 28` | Raise arm |
| 16 | `05 23` | Rotate wrist down |
| 17 | `0C 03` | Delay 3 seconds |
| 18 | `08 15` | Rotate claw |
| 19 | `09 00` | Open claw |
| 20 | `0C 02` | Delay 2 seconds |
| 21 | `0A 01` | Turn light on |
| 22 | `0C 07` | Delay 7 seconds |
| 23 | `0A 00` | Turn light off |
| 24 | `03 06` | Turn right |
| 25 | `0C 05` | Delay 5 seconds |
| 26 | `82 3F` | Speak ROM phrase — "Ha ha ha ha ha" |
| 27 | `0C 04` | Delay 4 seconds |
| 28 | `83 02` | "I am a great match for humans" |
| 29 | `0C 03` | Delay 3 seconds |
| 30 | `83 03` | "Goodbye for now, have a good day" |
| 31 | `0C 01` | Delay 1 second |
| 32 | `00 05` | Turn left |
| 33 | `0C 02` | Delay 2 seconds |
| 34 | `81 00` | Play tune #0 |
| 35 | `0C 02` | Delay 2 seconds |
| 36 | `0B 00` | Home |
| 37 | `02 14` | Back up |
| 38 | `FF FF` | End |

Disassemble any cart image:

```bash
python3 tools/maxx_rom.py disasm "Demo Cartridge/Firmware/Binary/MAXXCART.532" \
  --compare-dsm "Demo Cartridge/Firmware/Assembly/maxx_demo_ROM_532.dsm"
```

Validate structure:

```bash
python3 tools/maxx_rom.py validate "Demo Cartridge/Firmware/Binary/MAXXCART.532"
```

Generate a blank template:

```bash
python3 tools/maxx_rom.py template mycart.532
```

Export opcode JSON:

```bash
python3 tools/maxx_rom.py opcodes docs/opcodes.json
```

---

## 6. Speech phrase encoding

### ROM phrases (`$82`, `$0E`)

Phrases are indexed phoneme strings. The speech driver at `$F3D5` clocks 4-bit nybbles to `$1400`. Phoneme code tables reside at `$F567` / `$F4DB` in internal ROM (140 entries).

### RAM phrases (`$83`)

Cartridges embed phrase data copied to `$0500`. Each phrase is a sequence of 16-bit phoneme tokens; `$FF` pads unused slots.

Demo cart phrase table (from `$A081`):

```
Phrase 0: 44 19 3E 8C ...  "I am great (pause) and you"
Phrase 1: 44 19 68 83 ...  "I am ready when you are"
Phrase 2: 44 19 13 3E ...  "I am a great match for humans"
Phrase 3: 3D 25 04 5B ...  "Goodbye for now, have a good day"
```

Tokens reference the internal speech synthesizer codebook — treat them as opaque phoneme IDs until the `$F4DB` table is fully transcribed.

---

## 7. Music encoding

Music for opcode `$81` is stored as byte pairs in `$0400`:

```
70 12   ; note
70 11   ; note
38 0F   ; ...
54 0D
E0 0F
00 00   ; end
```

The music IRQ handler at `$F1D8` reads duration tables at `$F15B` and frequency data from `$0400`. Tune numbers in `$81` operands select phrase pointers via `$EF01`.

---

## 8. Keyboard matrix

The remote keypad is a row/column matrix. Key labels **A–Y** map to matrix lines:

| | L7 | L6 | L5 | L4 | PK |
|--|----|----|----|----|-----|
| D0 | A | B | C | D | |
| D1 | E | F | G | H | |
| L0 | I | J | K | L | |
| L1 | M | N | O | P | |
| L2 | Q | R | S | T | |
| L3 | U | V | W | X | |
| Gnd | | | | | Y |

Physical button names (remote faceplate):

- **Drive row**: four DRIVE buttons
- **Wrist / Arms / Grip row**
- **CLAW, LAMP, MOVE**
- **WAIT, NOTE RESET, SHIFT/DELETE, CLEAR, ENTER**
- **SONGS, CLICK, NOTES, SPEECH, MOTION**
- **GAME, PROGRAM, LEARN, EXECUTE**
- **POWER/STOP** (Y)

Reference images: [`keyboard-matrix-reference-1.png`](photos/transmitter/reverse-engineering/keyboard-matrix-reference-1.png), [`keyboard-matrix-reference-2.png`](photos/transmitter/reverse-engineering/keyboard-matrix-reference-2.png)

---

## 9. Writing a new cartridge

1. Start from `python3 tools/maxx_rom.py template mycart.532`
2. Edit the program table at file offset `$35` (address `$A035`) with your bytecode
3. Optionally patch phrase/music tables at `$81` / `$BB`
4. Keep the bootstrap stub and copyright header intact
5. Validate with `python3 tools/maxx_rom.py validate mycart.532`
6. Burn to a 4 KB EPROM (e.g. KM2365 family — see [`Chassis/Datasheets/U400 KM2365 (ROM).pdf`](../Chassis/Datasheets/U400%20KM2365%20(ROM).pdf))

### Minimal example program

```
83 10    ; speak phrase 0
0C 03    ; wait 3 seconds
01 10    ; forward
0B 00    ; home
FF FF    ; end
```

---

## 10. Known gaps / future work

- **Phoneme token table**: Full `$F4DB` / `$F567` transcription would enable authoring custom speech without copying demo tokens
- **Operand scaling**: Distance/angle units for motion opcodes are empirical (demo uses values like `$14`, `$06`, `$28`)
- **8080 code at `$A0C7+`** in the demo image is marked "not Maxx-related" in the `.dsm` listing — treat as padding
- **Robot main board KiCad**: placeholder at [`Mainboard/KiCAD/`](../Mainboard/KiCAD/); schematics in [`Mainboard/Schematic/`](../Mainboard/Schematic/)

---

## References

- GitHub archive: https://github.com/webaugur/Maxx-Steele-1984-Robot
- R. Wind disassemblies: `maxxbot@yahoo.com` (2002–2006)
- Demo cartridge ROM: [`Demo Cartridge/Firmware/`](../Demo%20Cartridge/Firmware/)
- Internal ROM: [`Chassis/Firmware/`](../Chassis/Firmware/)
- Transmitter hardware: [`docs/transmitter-bom.md`](transmitter-bom.md)