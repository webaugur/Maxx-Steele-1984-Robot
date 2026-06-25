# Chapter 5 — Internal ROM Operating System

This chapter documents the robot's **8 KB masked ROM** (`$E000`–`$FFFF`): boot, operating modes, the main interpreter loop, device drivers, and how cartridge bootstrap stubs hand off to it.

Cartridge layout and authoring workflow remain in [§ Cartridge bootstrap](#cartridge-bootstrap) below and in [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md).

**Primary sources:** [`Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm`](../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm) (R. Wind), [`Mainboard/Firmware/Binary/Maxxrom.64`](../Mainboard/Firmware/Binary/Maxxrom.64), [`tools/maxxbas/patches.json`](../tools/maxxbas/patches.json) (simulator traps).

---

## What the internal ROM is

At power-on the **MOS 6502** executes firmware in **`$E000`–`FFFF`**. This is the robot's only native execution environment — analogous to a C64 KERNAL, but specialized for:

- Cold/warm boot and RAM initialization
- Cartridge detection and table loading
- Keypad scan and **five operating modes** (`$0D` = 0–4)
- Bytecode fetch/decode from `$0200`
- Speech, music, motor, and display drivers
- Built-in games (mode 4)

User motion programs are **not** generally written in 6502. They are opcode/operand pairs in RAM, interpreted by this ROM. Cartridges supply a short **6502 bootstrap** plus data tables; they do not replace the interpreter.

---

## Boot sequence

### Reset and copyright check

| Address | Role |
|---------|------|
| `$E041` | **RESET** entry — `SEI`, init `$1E00` and display, set stack |
| `$E04F`–`$E058` | Compare ROM copyright at `$E000` with RAM mirror `$0100` |
| `$E05A` | **Cold start** — zero page, phrase RAM `$0500`–`$05FF` ← `$FF`, clear music |
| `$E06F` | **Warm start** — refresh copyright mirror, copy vector table |

Cold start calls:

1. `$EF55` — zero music table `$0400`–`$04FF`
2. `$E582` — init program pointers and flags
3. `$E57B` — set program terminator at `$0180` ← `$FF`

Warm start copies the **37-byte vector table** from ROM `$E01C` into RAM **`$72`–`$96`**, then calls `$EEEE` (timer init).

### RAM vector table (`$72`–`$96`)

Copied from ROM at boot. Key entries (initial values from listing):

| ZP | Points to | Purpose |
|----|-----------|---------|
| `$76` | `$E014` | NMI handler |
| `$78` | `$FDC8` | IRQ handler |
| `$7A` | `$E0CB` | Immediate-mode return (main loop) |
| `$7C`–`$85` | `$F1D8` region | Music IRQ jump table |
| `$8C` | `$EE32` | **Opcode dispatch** (`JMP ($008C)` at `$EE2F`) |
| `$8E`/`$8F` | `$1000` | Cartridge scan pointer (page/bank) |
| `$90` | `$E598` | Alternate dispatch |
| `$92` | `$F3D8` | Speech output vector |
| `$94` | `$F8CE` | **Game mode** entry |

Full table: [Appendix D — Zero-page registers](Appendices.md#d-zero-page-registers).

### Cartridge scan

After vectors are loaded, the ROM walks 4 KB boundaries from **`$2000`** through **`$B000`**:

1. Read 16-bit entry vector at cart base (`$8E`/`$8F`)
2. Compare bytes at offset +2 against ROM copyright template (`(c) 1985 CBS Toys`)
3. On match: store vector in `$8E`/`$8F` and **`JMP ($008E)`** into cartridge bootstrap
4. On no match: fall through to **`$E0B6`** (main loop without cart)

The factory demo cart at **`$A000`** is the reference bootstrap; see [Cartridge bootstrap](#cartridge-bootstrap).

---

## Main loop and mode dispatch

### Entry: `$E0B6`

Whether or not a cartridge was found, execution eventually reaches **`$E0B6`**:

1. Display **"9"** on the LED (`$ED4F` / `$ED7B`)
2. Greet: *"Hello, I'm Maxx Steele."* (`$F475`, phrase `$10`) if speech enabled
3. Check stalled motors (`$EDAF`)
4. Set **`$75` ← `$80`** (keypad ready), **`$0D` ← 0** (immediate mode)
5. Load startup tune 1 (`$EF01`)
6. Call `$E905` (status/key housekeeping)
7. Dispatch on **`$0D`** at **`$E0ED`**

### Mode variable `$0D`

| Value | Mode | Display label | Entry |
|-------|------|---------------|-------|
| 0 | Immediate | — | `$E154` — live key execution |
| 1 | Learn | `LErn` | `$E161` — record key sequences |
| 2 | Program | `Prog` | `$E346` — enter bytecode via keypad |
| 3 | Execute | `run` | `$E434` — run program at `$0200` |
| 4 | Game | `PLAy` | `JMP ($0094)` → `$F8CE` |

Modes 1–4 show a two-character LED label (`$0D` + 5 → index into display strings at `$F684`).

Mode changes re-enter through **`$E0D1`** (`STA $0D` then fall into the dispatch chain). The **GAME** remote key and clock/status paths also jump to `$E0D1` with the new mode in A.

### Shared busy flag: `$2B`

Many paths call **`$EF63`** first — a tight loop until **`$2B` = 0**. The listing marks `$2B` as a motor/speech **talkback** busy flag (motor controller confirms motion before the next command). The simulator bypasses this wait at `$EF63` ([`patches.json`](../tools/maxxbas/patches.json)).

### Keypad input: `$75` / `$15`

Decoded remote keys land in **`$75`** (bit 7 set when a key is pending). **`$E617`** polls the RF/matrix path; **`$E6A4`** copies the keycode to **`$15`** and clears the ready bit in `$75`.

Trap addresses for debugging: `$E161` (learn loop), `$E17B` (immediate decode), `$E617` (poll), `$EE2F` (opcode dispatch).

---

## Mode-specific behavior

### Immediate (mode 0) — `$E154` / `$E161` / `$E17B`

- Says *"I'm ready."* (phrase `$20`) when speech is on
- **`$E161`** loop: `$E617` → test `$15`
- **`$E17B`**: key in A — WAIT (`$0C`), motion keys, PLAY, SPEECH, etc.
- Motion keys call **`$EF2E`** (drive) or **`$EF40`** (stop); PLAY resolves tune via **`$EF01`**
- **`$E6B5`** table maps keycodes 0–`$13` to opcode bytes for learn/program paths

### Learn (mode 1) — `$E161` / `$E574`

- Init program pointer (`$E574` sets `$0200` terminator, `$0F`/`$10` ← `$0180`)
- Prompt: *"Please teach me."* (phrase `$1E`)
- Records key sequences into `$0200`; CLEAR (`$0E`) calls **`$E3A9`**
- Shift/CLEAR keys move or prepend steps (`$E2A2` region)

### Program (mode 2) — `$E346`

- Shows current step number (`$F64E`)
- Keypad maps through **`$E519`** → opcode in **`$11`**, operand in **`$13`**
- **`$E3F9`** / **`$E409`** — step entry and RUN (ENTER) preview via **`$F66C`**
- OUT OF SPACE → *"Sorry, my circuits are full"* (`$F40F`)
- BEGIN (`$13`) decrements pointer; END markers `$FF`/`$FE`

### Execute (mode 3) — `$E434`

- Sets run flag, walks **`($0F,$10)`** through `$0200`
- Each step: **`$E519`** → **`$F66C`** (execute one command) → **`$E600`** (wait for `$75`)
- **`$FF`** ends program; returns to immediate via **`JMP ($007A)`**
- Low battery: **`$96`** counter triggers *"I need energy, please recharge me."* during execute/game

### Game (mode 4) — `$F8CE`

- Vector at **`$94`** → **`$F8CE`**
- Prompt: *"Please choose game"* (phrase `$12`)
- **Game 1** (`$F8E9`): reflex/timing style loop
- **Game 2** (`$FAA4`): extended play with difficulty selection (`$F9E8`)
- Win/lose speech via **`$F40F`**; replay prompt at **`$FA67`**

Game internals (scoring, IRQ timing at `$FB8C`–`$FC6C`) are not fully reverse-engineered in this archive.

---

## Bytecode executor

### One step: `$F66C`

Called from program mode preview, execute loop, and learn playback:

1. **`$F737`** — compute 1-based step number from `$0F`/`$10`
2. Load **`$11`** (opcode), **`$13`** (operand)
3. **`$FF`** → display **End**, exit; **`$FE`** → display **beg**
4. **`$0B` (HOME)** → four-byte init pattern from `$F7F8`–`$F858` tables
5. Opcodes `$80`–`$87` remap to table indices `$0C`–`$13`
6. Display two-character name from **`$F878`** table; show **`$24`/`$25`** as hex for extended opcodes
7. **`JMP ($008C)`** at **`$EE2F`** — dispatch to motion/speech/music handlers

### Program pointer

| Addr | Role |
|------|------|
| `$0F`/`$10` | Current step address in `$0200` (starts `$0180` = byte offset into table) |
| `$11`/`$13` | Current opcode / operand |
| `$24`/`$25` | Executor working pointer (displayed for `$8x` opcodes) |

Increment: **`$E41A`** (+2); decrement/BEGIN: **`$E555`**.

---

## Device drivers

### Display (`$1200`)

| Address | Role |
|---------|------|
| `$ED4F` | Shift LO bit to display chain |
| `$ED7B` | Send one byte to `$1200` |
| `$ED48` | Display prefix / spacing |
| `$F684` | Send null-terminated 2-char strings (mode labels, tune names) |
| `$F878` | Opcode → LED segment patterns (Appendix B) |

Simulator bypasses serial wait loops at `$ED5F`–`$EDA0` ([`patches.json`](../tools/maxxbas/patches.json)).

### Speech (`$1400`)

| Address | Role |
|---------|------|
| `$F3D5` | Speech output entry (`JMP ($0092)` → `$F3D8`) |
| `$F460` / `$F465` | Clock nybbles to `$1400` |
| `$F567` / `$F4DB` | Phoneme/duration tables (140 entries, index in X) |
| `$F475` | Say built-in ROM phrase by index (table at `$F640`) |
| `$F40F` | Say phrase with index in X (games, errors, prompts) |

Built-in phrases `$10`–`$20` include greetings, mode prompts, and game text (see listing at `$F640`).

### Music

| Address | Role |
|---------|------|
| `$EF01` | Resolve tune pointer for operand (tune table in ROM) |
| `$F15B` | Note duration table (indexed 1–8) |
| `$F1D8` | Music IRQ handler region |
| `$F0B8` | Add note pair to `$0400` |
| `$EF55` | Clear music RAM |

IRQ path decrements **`$26`**, **`$28`**, **`$2A`**, **`$27`** for note timing and game delays.

### Motors (`$1600`, `$1C00`, `$1E00`)

| Address | Role |
|---------|------|
| `$EF2E` | Motor drive — Y = bank, A = command nibble |
| `$EF40` | Stop all motors (Y=`$01`, A=`$6F`) |
| `$EF63` | Wait for talkback (`$2B` = 0) |
| `$EDAF` | Stalled-motor check (main loop) |
| `$EF76` | Leave learn mode / motor cleanup |

I/O bitfields at `$1000`, `$1600`, `$1C00`, and `$1E00` are not fully documented; see [Chapter 6](06-Input-Output-Guide.md).

---

## Cartridge bootstrap

### What a cartridge is

A **4 KB EPROM** maps into **`$2000`–`$B000`** in 4 KB steps. The factory demo sits at **`$A000`**.

A valid image supplies:

1. Entry vector and copyright header
2. A short **6502 bootstrap stub**
3. Program, speech, and music **data tables**

The stub copies tables into RAM and jumps to **`$E0B6`** — it does **not** replace the interpreter.

### ROM layout (`$A000` base)

| Offset | Size | Content |
|--------|------|---------|
| `$00` | 2 | Entry vector (word), e.g. `$A013` |
| `$02` | 17 | Copyright: `(c) 1985 CBS Toys` |
| `$13` | code | Bootstrap stub (6502) |
| `$35` | table | Program bytecode |
| `$81` | table | Speech phrases (opcode `$83`) |
| `$BB` | table | Music notes (opcode `$81`) |

Cartridge detection compares the copyright string at offset +2 against the internal ROM template.

### Demo stub sequence

Entry at **`$A013`** ([`maxx_demo_ROM_532.dsm`](../Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm)):

1. `STA $02` / `STA $03` — initialize status flags
2. Copy program from **`$A035`** → **`$0200`**
3. Copy phrases from **`$A081`** → **`$0500`**
4. Copy music from **`$A0BB`** → **`$0400`**
5. **`JMP $E0B6`** — enter internal ROM main loop

Expected entry prologue: `A9 02 85 02` (LDA #$02 / STA $02). [`tools/maxx_rom.py validate`](../tools/maxx_rom.py) checks this.

---

## 6502 vs bytecode — when to use which

| Task | Language |
|------|----------|
| Motion/speech demo sequence | Bytecode in `$0200` |
| Custom phrases/music in cart | Data tables + bootstrap |
| Patch OS behavior | 6502 patch (advanced RE only) |
| New cartridge program | Bootstrap stub (minimal 6502) + bytecode tables |
| Debug firmware paths | `maxx simulate` + [`patches.json`](../tools/maxxbas/patches.json) traps |

Do not place arbitrary 6502 execution paths in the demo program area unless you fully understand cartridge mapping and internal ROM expectations.

---

## Tools and simulation

| Tool | Use |
|------|-----|
| [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) §9 | Step-by-step cartridge authoring |
| [`tools/maxx_rom.py`](../tools/maxx_rom.py) | `template`, `validate`, `disasm` |
| [`tools/maxx simulate`](../tools/maxxbas/README.md) | Patched ROM + program trace + traps |
| [`Mainboard/Firmware/README.md`](../Mainboard/Firmware/README.md) | ROM binary and listing index |

```bash
python3 tools/maxx_rom.py template mycart.532
python3 tools/maxx_rom.py validate mycart.532
maxx simulate Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532 --cycles 30000
```

EPROM type: Mitsubishi KM2365 family — [`DataSheets/Mitsubishi-KM2365.pdf`](../DataSheets/Mitsubishi-KM2365.pdf).

---

## Known gaps

- **Game internals** — scoring, `$A6`–`$AD` game state, IRQ paths `$FB8C`–`$FC6C` need deeper RE
- **Full zero page** — only programmer-facing locations are catalogued ([Appendix D](Appendices.md#d-zero-page-registers))
- **I/O bitfields** — `$1000`, `$1600`, `$1C00`, `$1E00` register layouts incomplete
- **Motor talkback** — `$EF63` / `$2B` protocol not fully traced to hardware
- **Phoneme tables** — `$F4DB` / `$F567` not transcribed to phoneme names
- **Complete subroutine catalog** — partial index in [Appendix I](Appendices.md#i-internal-rom-entry-points); full `$E000`–`FFFF` map not yet mined

---

**Previous:** [Chapter 4](04-Programming-Speech-and-Music.md) · **Next:** [Chapter 6 — I/O guide](06-Input-Output-Guide.md)