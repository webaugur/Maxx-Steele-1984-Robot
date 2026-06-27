# Chapter 5 — Internal ROM Operating System

This chapter documents the robot's **8 KB masked ROM** (`$E000`–`$FFFF`): boot, operating modes, the main interpreter loop, device drivers, and how cartridge bootstrap stubs hand off to it.

Cartridge layout and authoring workflow remain in [§ Cartridge bootstrap](#cartridge-bootstrap) below and in [`Cartridge/PROGRAMMING.md`](../../Cartridge/PROGRAMMING.md).

**Primary sources:** [`Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm`](../../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm) (R. Wind), [`Mainboard/Firmware/Binary/Maxxrom.64`](../../Mainboard/Firmware/Binary/Maxxrom.64), [`tools/maxxbas/patches.json`](../../tools/maxxbas/patches.json) (simulator traps).

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

Many paths call **`$EF63`** first — a tight loop until **`$2B` = 0**. The listing marks `$2B` as a motor/speech **talkback** busy flag (motor controller confirms motion before the next command). The simulator bypasses this wait at `$EF63` ([`patches.json`](../../tools/maxxbas/patches.json)).

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

Player-facing rules for the two built-in games are in [User Manual Ch 6 — Games](../User/06-Games-And-Other-Modes.md) (*Moon Ball* and *Force Field*). This section maps that behavior to ROM entry points and zero-page state.

**Mode entry:** **`$94`** → **`$F8CE`**. The **GAME** remote key sets **`$0D` ← 4** via **`$E0D1`** (not mode 3 — execute is mode 3). Display label **`PLAy`** comes from the mode string table at **`$F684`** (`$0D` + 5 → index).

#### Shared entry flow (`$F8CE`–`$F8E6`)

| Step | Address | Action |
|------|---------|--------|
| 1 | `$EBDC` | HOME arms/wrist/claw (game preamble) |
| 2 | `$EF63` | Wait for motor talkback (`$2B` = 0) |
| 3 | `$F9EA` | Say phrase **`$12`** — *"Please choose game."* |
| 4 | `$E9FE` | Wait for digit + **ENTER**; operand **`$20`** in **`$13`**, selector **`$05`** in A |
| 5 | `$F8E4` | Operand **0** → Moon Ball (`$F8E9`); non-zero → Force Field (`$FAA4`) |

Both games call **`$F9DE`** (zero **`$A6`–`$AD`**) and **`$F9E8`** (difficulty picker: phrase **`$11`**, accept **0–3**, return index in X).

#### Moon Ball — game 1 (`$F8E9`)

Reflex game: reflect the headlamp into the photo transistor between the eyes when the lamp flashes.

| User Manual rule | ROM implementation |
|------------------|-------------------|
| Difficulty 0–3 | **`$F9E8`** → **`$A6`** |
| Ready: reflect light within ~20 s | **`$27` ← `$14`** IRQ countdown; **`BIT $1200`** / **`BVS`** at `$F91D`–`$F920` |
| Say *"I'm ready."* | Phrase **`$20`** at `$F90C` |
| 3 misses → game over | **`$AC` ← 3** at `$F8FB`; **`DEC $AC`** on timeout at `$F98F` |
| *"Good play"* + score on display | Phrase **`$13`** at `$F99C`; BCD add **`$A6`** → **`$A7`/`$A8`**; **`$FA01`** copies to **`$24`/`$25`** |
| Difficulty ramps (shorter window, dodge, more points) | Tables **`$FA90`–`$FAA0`**: seconds (`$FA90`), dodge delay (`$FA94`), points per return (`$FAA0`); **`$A6`** incremented after **`$A9`** rounds expire |
| Game over + replay | **`$F9C6`**: phrase **`$1B`**; **`$FA67`**: phrase **`$1C`**, wait for **ENTER** (`$27` ← `$10`); timeout → immediate via **`JMP ($007A)`** |

Main play loop: **`$F92C`** (random drive + flash) → **`$F996`** (hit) or miss path → **`$F9C6`** when **`$AC`** underflows.

Photo input uses the display/MMIO path at **`$1200`** (see [Chapter 6](06-Input-Output-Guide.md)); the faceplate photo transistor sits in the light-return path described in the User Manual.

#### Force Field — game 2 (`$FAA4`)

Two-player-style duel: player shield/laser on the remote vs. Maxx shield/laser (siren + warble + headlamp).

| User Manual rule | ROM implementation |
|------------------|-------------------|
| Difficulty 0–3 | **`$F9E8`** → **`$A6` ← X × 4** at `$FAAD`–`$FAAF` |
| Shield = hold **`<0>`**, laser = hold **`<3>`** | **`$FA0C`** reads keypad; mask table **`$FA98`**; **`$FB8C`** shield/laser state machine |
| Scores: player left, Maxx right on display | BCD **`$A7`** (player) / **`$A8`** (Maxx) → **`$24`/`$25`** at `$FC34`–`$FC3C` |
| 1-in-4 ricochet off Maxx shield | **`$FBB1`/`$FBB5`**: compare **`$66`/`$69`** to **1** or **4**; penalty via **`$FC12`** (subtract from **`$A7`**) |
| First to **25** points wins | **`CMP #$25`** at **`$FC30`** |
| ~20 s levels + intermission tune | Level timer **`$28`** (scaled by **`$A6`** at `$FB4B`); **`$FBCE`** intermission; **`$2A`** stun countdown |
| Win / lose speech | Phrase **`$18`** (*Maxx Steele wins*) or **`$19`** (*Congratulations, you win*) at `$FC4A`–`$FC50` |
| Replay | **`$FA67`** after win sequence at `$FC69` |

IRQ-driven timing for level ticks lives in **`$FB8C`–`$FC6C`** and the game slice of the IRQ handler at **`$FC6F`** / **`$FDCD`**.

#### Game state variables (`$A6`–`$AD`)

| ZP | Moon Ball | Force Field |
|----|-----------|-------------|
| **`$A6`** | Difficulty index; also **points added** per successful return (ramps to 3) | Difficulty × 4; level index incremented between rounds |
| **`$A7`** | Player score (BCD low) | Player score (display left / **`$24`**) |
| **`$A8`** | Player score (BCD high) | Maxx score (display right / **`$25`**) |
| **`$A9`** | Rounds-until-ramp counter (starts 3) | Phase/level sub-counter |
| **`$AA`** | — | Sub-phase timer (shield down, laser burst) |
| **`$AB`** | — | Keypad-derived shield/laser flags |
| **`$AC`** | Misses remaining (starts 3) | — |
| **`$AD`** | Cleared by **`$F9DE`**; role TBD | Cleared by **`$F9DE`**; role TBD |

Full ZP catalog: [Appendix D — Zero-page registers](Appendices.md#d-zero-page-registers).

#### Game speech phrases (ROM index ↔ factory manual #)

Factory manual lists phrases **16–32**; ROM stores them at **`$F640`** with indices **`$10`–`$20`**. Game-related entries:

| Manual # | ROM index | Text |
|----------|-----------|------|
| 17 | **`$12`** | Please choose game. |
| 18 | **`$11`** | Please choose how tough. |
| 19 | **`$13`** | Good play. |
| 24 | **`$18`** | Maxx Steele wins. |
| 25 | **`$19`** | Congratulations, you win. |
| 27 | **`$1B`** | Game over. |
| 28 | **`$1C`** | Choose enter to play again. |
| 32 | **`$20`** | I'm ready. |

Say path: **`$F40F`** (index in X). Greeting uses **`$F475`** for phrase **`$10`** at main loop entry.

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

Simulator bypasses serial wait loops at `$ED5F`–`$EDA0` ([`patches.json`](../../tools/maxxbas/patches.json)).

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

MMIO bitfields and CPU pin mapping: [Chapter 6](06-Input-Output-Guide.md), [`Mainboard/Schematic/MMIO-Pin-Map.md`](../../Mainboard/Schematic/MMIO-Pin-Map.md).

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

Entry at **`$A013`** ([`maxx_demo_ROM_532.dsm`](../../Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm)):

1. `STA $02` / `STA $03` — initialize status flags
2. Copy program from **`$A035`** → **`$0200`**
3. Copy phrases from **`$A081`** → **`$0500`**
4. Copy music from **`$A0BB`** → **`$0400`**
5. **`JMP $E0B6`** — enter internal ROM main loop

Expected entry prologue: `A9 02 85 02` (LDA #$02 / STA $02). [`tools/maxx_rom.py validate`](../../tools/maxx_rom.py) checks this.

### Extending vs replacing the OS

**Full OS replacement via cartridge alone is not possible.** Reset always enters **`$E041`** in the 8 KB mask ROM (`$E000`–`$FFFF`). That ROM stays mapped; there is no documented bank-out or cartridge overlay of the OS region. Replacing the entire firmware requires a new mask ROM, a piggyback/modified ROM chip, or similar hardware work—not the stock cartridge slot.

**Partial extension is possible.** Boot order matters:

1. Warm start copies the vector table from ROM **`$E01C`** → RAM **`$72`–`$96`** (`$E085`–`$E08B`)
2. Cartridge scan: on copyright match, **`JMP ($008E)`** into cart bootstrap (`$E0A9`–`$E0B3`)
3. Cart code runs **after** vectors are in RAM but **before** the main loop at **`$E0B6`**

The factory stub only loads RAM tables and returns to **`$E0B6`**. A custom bootstrap can do more in that window:

| Technique | What it can change |
|-----------|-------------------|
| **RAM vector patch** | Repoint entries in **`$72`–`$96`** before handing off (see table below) |
| **Extended cart 6502** | Skip **`JMP $E0B6`** and run a cart-resident loop at **`$A000`–`$AFFF`**, calling ROM drivers via **`JSR`** |
| **Data layer (factory pattern)** | Replace program/phrases/music in RAM — bytecode interpreted by mask ROM |

**Reference example:** [`Cartridge/Examples/MaxxOS/`](../../Cartridge/Examples/MaxxOS/) — bootstrap at **`$A013`** jumps to **`$A080`** and runs a keypad math quiz using **`$E60D`**, **`$F684`**, **`$F475`**, and **`$F8BE`** display segments. No bytecode tables; [`tools/maxx_rom.py`](../../tools/maxx_rom.py) treats images without **`JMP $E0B6`** as extended carts.

**RAM vectors a cart bootstrap can repoint** (defaults from ROM `$E01C`):

| ZP | Default target | Role |
|----|----------------|------|
| `$8C`/`$8D` | `$EE32` | Opcode dispatch — **`JMP ($008C)`** at **`$EE2F`** |
| `$7A`/`$7B` | `$E0CB` | Return to main loop (execute end, game replay timeout, …) |
| `$94`/`$95` | `$F8CE` | Game mode entry — **`JMP ($0094)`** |
| `$92`/`$93` | `$F3D8` | Speech output — **`JMP ($0092)`** |
| `$78`/`$79` | `$FDC8` | IRQ handler |
| `$76`/`$77` | `$E014` | NMI handler |

Cart handlers can **`JSR`** back into mask-ROM drivers (`$EF2E` motors, `$F3D8` speech, `$ED7B` display, …) for hardware they do not reimplement.

**Limits:**

- Only a **few** OS paths use **`JMP ($00xx)`** through these vectors; most firmware **`JSR`/`JMP`s fixed addresses** in `$E000`–`$FFFF` (mode dispatch, keypad poll, execute loop, IRQ internals).
- Cart space is **4 KB** per image; header and tables consume much of the factory layout.
- **Copyright required** for bootstrap to run; without a match, firmware goes straight to **`$E0B6`**.
- Scan walks **`$2000`–`$B000`**; the **first** matching cart wins—no second bootstrap in the same boot.

| Goal | Via cart bootstrap? | Notes |
|------|---------------------|-------|
| Custom motion/speech/music program | **Yes** | Bytecode + tables (factory pattern) |
| Custom game mode | **Partially** | Patch **`$94`** → cart code; mode entry/keypad still ROM |
| Custom opcode dispatch | **Partially** | Patch **`$8C`**; many paths bypass the vector |
| Replace keypad / mode / IRQ core | **Practically no** | Hardcoded ROM; IRQ timing critical |
| Replace entire OS | **No** | Mask ROM always owns reset and most code paths |
| Full custom firmware | **Hardware** | New mask ROM or piggyback; not cartridge alone |

Patching **`Maxxrom.64`** itself (or simulator [`patches.json`](../../tools/maxxbas/patches.json) traps) is a separate path from cartridge bootstrap—see **6502 vs bytecode** below.

---

## 6502 vs bytecode — when to use which

| Task | Language |
|------|----------|
| Motion/speech demo sequence | Bytecode in `$0200` |
| Custom phrases/music in cart | Data tables + bootstrap |
| Patch OS behavior (mask ROM or cart vectors) | 6502 — see **Extending vs replacing the OS** above |
| New cartridge program | Bootstrap stub (minimal 6502) + bytecode tables |
| Debug firmware paths | `maxx simulate` + [`patches.json`](../../tools/maxxbas/patches.json) traps |

Do not place arbitrary 6502 execution paths in the demo program area unless you fully understand cartridge mapping and internal ROM expectations.

---

## Tools and simulation

| Tool | Use |
|------|-----|
| [`Cartridge/PROGRAMMING.md`](../../Cartridge/PROGRAMMING.md) §9 | Step-by-step cartridge authoring |
| [`tools/maxx_rom.py`](../../tools/maxx_rom.py) | `template`, `validate`, `disasm` |
| [`tools/maxx simulate`](../../tools/maxxbas/README.md) | Patched ROM + program trace + traps |
| [`Mainboard/Firmware/README.md`](../../Mainboard/Firmware/README.md) | ROM binary and listing index |

```bash
python3 tools/maxx_rom.py template mycart.532
python3 tools/maxx_rom.py validate mycart.532
maxx simulate Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532 --cycles 30000
```

EPROM type: Mitsubishi KM2365 family — [`DataSheets/Mitsubishi-KM2365.pdf`](../../DataSheets/Mitsubishi-KM2365.pdf).

---

## Known gaps

- **Game IRQ / timing tables** — decode **`$FA90`–`$FAA0`** bytes to seconds and motor arcs; trace **`$FB8C`–`$FC6C`** and **`$FDCD`** game tick to headlamp/motor **`$EF2E`** commands
- **`$F9FE` / `$E9FE`** — keypad wait helper spans display-init bytes in the `.dsm`; re-anchor entry for game-select operand **`$05`**
- **Full zero page** — only programmer-facing locations are catalogued ([Appendix D](Appendices.md#d-zero-page-registers))
- **I/O bitfields** — provisional tables in Ch 6 + MMIO pin map; glue IC refdes still TBD
- **Motor talkback** — `$EF63` / `$2B` not traced to **MoCOP Done** / COP41xL pin yet
- **`$F222` motor serial** — target of `$EF2E`/`$EF40`; not fully disassembled in `.dsm`
- **Phoneme tables** — `$F4DB` / `$F567` not transcribed to phoneme names
- **Complete subroutine catalog** — partial index in [Appendix I](Appendices.md#i-internal-rom-entry-points); full `$E000`–`FFFF` map not yet mined

---

**Previous:** [Chapter 4](04-Programming-Speech-and-Music.md) · **Next:** [Chapter 6 — I/O guide](06-Input-Output-Guide.md)