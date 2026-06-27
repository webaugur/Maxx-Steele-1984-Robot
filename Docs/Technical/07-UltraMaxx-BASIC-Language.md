# Chapter 7 — UltraMaxx BASIC Language

This chapter describes **UltraMaxx BASIC** — a line-oriented programming language for authoring Maxx Steele cartridge programs without hand-editing bytecode. UltraMaxx BASIC is a **compiler language**, not an interpreter: each statement becomes one **(opcode, operand)** pair in the cartridge program table (see [Chapter 1](01-Bytecode-Programming-Rules.md)).

UltraMaxx BASIC is intentionally small. It is **not** Commodore 64 BASIC: there are no variables, no `IF`/`THEN`, no `GOTO`, and no arithmetic expressions. Think of it as a readable front end to the opcode vocabulary in [Chapter 2](02-Opcode-Vocabulary.md).

**Tools:** [`tools/maxx`](../../tools/maxx) (command line), [`tools/maxxbas/`](../../tools/maxxbas/) (Rust compiler).  
**Example sources:** [`ultramaxx.bas`](../../Cartridge/Examples/UltraMaxx/Firmware/Basic/ultramaxx.bas), [`cbsdemo.bas`](../../Cartridge/Examples/CBSDemo/Firmware/Basic/cbsdemo.bas), [`hello.bas`](../../Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas).

---

## Introduction

The factory demo cartridge ([`CBSDemo.532`](../../Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532)) stores its 38-step sequence as raw bytes at ROM address **`$A035`**. UltraMaxx BASIC lets you write the same sequence as text:

```basic
FORWARD 20
DELAY 4
ARMS UP 40
END
```

The compiler emits a standard **4096-byte** cartridge image (`.532`) with bootstrap stub, copyright string, program table, and optional phrase/music tables — the layout described in [Chapter 5](05-Cartridge-Bootstrap-and-Internal-ROM.md).

After compile, upload the image with PicoROM or burn EPROM. The robot runs the program through the normal internal ROM executor at **`$E000`**; the cartridge does not interpret BASIC at runtime.

---

## Getting started

Add the toolchain ([`tools/bin/`](../../tools/bin/)) to your path once:

```bash
export PATH="$(git rev-parse --show-toplevel)/tools/bin:$PATH"
```

Compile a source file:

```bash
maxx compile myprogram.bas -o mycart.532 --copyright ultramaxx
maxx validate mycart.532
```

Upload to a socketed PicoROM:

```bash
maxx upload myprogram.bas --device maxx_cart --copyright ultramaxx
```

Parse without writing output:

```bash
maxx check myprogram.bas
```

List program steps (JSON for simulators):

```bash
maxx list mycart.532 --json
```

---

## Program format

### Lines

A program is a text file, one statement per line. Blank lines are ignored.

**Line numbers** are optional and may be used for editor convenience (as in C64 listings). The compiler strips a leading numeric prefix:

```basic
10 DELAY 2
20 FORWARD 20
30 END
```

### Comments

Two comment forms are accepted:

| Form | Example |
|------|---------|
| `REM` at start of line | `REM wait for motors` |
| `REM` after a statement | `FORWARD 20  REM short roll` |
| `#` to end of line | `LAMP ON  # headlamp` |

Comments are not stored in the ROM image.

### Statement syntax

- Keywords are **case-insensitive** (`delay`, `DELAY`, `Delay` are equivalent).
- Operands are unsigned integers **0–255** (decimal or `0x` hex).
- Multi-word keywords use spaces: `LAMP ON`, `ARMS UP`, `CLAW OPEN`.

### Program terminator

Every program must end with **`END`**, which compiles to **`FF FF`**. If you omit `END`, the compiler appends it automatically.

---

## Program size limits

| Limit | Value | Notes |
|-------|-------|-------|
| Steps per cartridge program | **38 pairs max** | Program ROM area `$A035`–`$A080` (76 bytes) |
| Operand range | 0–255 | One byte per step |
| Factory demo length | 38 pairs | Fills the cartridge program slot |
| RAM program buffer | ~255 pairs | `$0200` region; larger than cart ROM |

If you exceed 38 pairs, the compiler reports **program too large**. Short programs (for example [`hello.bas`](../../Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas)) are valid; unused program bytes in ROM are filled with **`$FF`**.

The robot may accept longer sequences entered from the remote in Program mode, but a **standard 4 KB cart image** cannot ship more than 38 compiled steps without changing the fixed table layout.

---

## UltraMaxx BASIC statements

Each statement maps to one bytecode pair. The **Byte** column shows the emitted `(opcode, operand)` in hex.

### DELAY

**Format:** `DELAY` *seconds*

**Action:** Pause execution. Operand is delay time in **seconds** (opcode `$0C`).

| Example | Bytes |
|---------|-------|
| `DELAY 2` | `0C 02` |
| `DELAY 10` | `0C 0A` |

---

### Drive: FORWARD, BACK, LEFT, RIGHT

**Format:** `FORWARD` *n* · `BACK` *n* · `LEFT` *n* · `RIGHT` *n*

**Action:** Drive or turn. Operand scaling is empirical — see [Chapter 3](03-Programming-Motion-and-Display.md).

| Statement | Opcode | Demo example |
|-----------|--------|--------------|
| `FORWARD 20` | `$01` | `01 14` |
| `BACK 20` | `$02` | `02 14` |
| `LEFT 5` | `$00` | `00 05` |
| `RIGHT 6` | `$03` | `03 06` |

---

### ARMS UP / ARMS DOWN

**Format:** `ARMS UP` *n* · `ARMS DOWN` *n*

**Action:** Raise or lower both arms (opcodes `$06` / `$07`).

| Example | Bytes |
|---------|-------|
| `ARMS UP 40` | `06 28` |
| `ARMS DOWN 30` | `07 1E` |

---

### WRIST UP / WRIST DOWN

**Format:** `WRIST UP` *n* · `WRIST DOWN` *n*

**Action:** Wrist joint motion (opcodes `$04` / `$05`).

| Example | Bytes |
|---------|-------|
| `WRIST DOWN 35` | `05 23` |

---

### CLAW ROTATE / CLAW OPEN / CLAW CLOSE

**Format:** `CLAW ROTATE` *n* · `CLAW OPEN` · `CLAW CLOSE`

**Action:** Claw rotate (`$08`), open (`$09`/`$00`), close (`$09`/`$01`).

| Example | Bytes |
|---------|-------|
| `CLAW ROTATE 21` | `08 15` |
| `CLAW OPEN` | `09 00` |
| `CLAW CLOSE` | `09 01` |

---

### LAMP ON / LAMP OFF

**Format:** `LAMP ON` · `LAMP OFF`

**Action:** Head lamp (opcode `$0A`).

| Example | Bytes |
|---------|-------|
| `LAMP ON` | `0A 01` |
| `LAMP OFF` | `0A 00` |

---

### HOME

**Format:** `HOME`

**Action:** Return all joints to reference position. Operand is always **`$00`**.

| Example | Bytes |
|---------|-------|
| `HOME` | `0B 00` |

---

### PLAY

**Format:** `PLAY` *tune*

**Action:** Play a tune from the cartridge music table (opcode `$81`). Tune data lives at ROM **`$A0BB`** (copied to RAM **`$0400`**). See [Chapter 4](04-Programming-Speech-and-Music.md).

| Example | Bytes |
|---------|-------|
| `PLAY 6` | `81 06` |
| `PLAY 0` | `81 00` |

---

### SAY

**Format:** `SAY` *phrase*

**Action:** Speak a **RAM phrase** by slot number (opcode `$83`). Phrase bytes must exist in the cartridge phrase table at **`$A081`** (copied to RAM **`$0500`**).

| Example | Bytes | Factory demo text |
|---------|-------|-------------------|
| `SAY 16` | `83 10` | "Hello, I am Maxx Steele" |
| `SAY 0` | `83 00` | "I am great, and you" |

UltraMaxx BASIC does **not** yet compile quoted strings (`SAY "HELLO"`) into phoneme nybbles. Use **`SPEAK`** for built-in ROM speech, or compile with **`--tables-from`** to copy phrase bytes from a reference ROM (see §Phrase and music tables below).

---

### SPEAK

**Format:** `SPEAK` *phrase*

**Action:** Speak a **ROM phrase** by index (opcode `$82`). Uses built-in phoneme data in internal ROM — no cart phrase table required.

| Example | Bytes | Notes |
|---------|-------|-------|
| `SPEAK 63` | `82 3F` | Laugh (factory demo) |

---

### END

**Format:** `END`

**Action:** Terminate the program. Both bytes must be **`$FF`**.

| Example | Bytes |
|---------|-------|
| `END` | `FF FF` |

---

## Statement summary table

| UltraMaxx BASIC | Opcode | Operand |
|-----------------|--------|---------|
| `DELAY` *n* | `$0C` | seconds |
| `FORWARD` *n* | `$01` | distance |
| `BACK` *n* | `$02` | distance |
| `LEFT` *n* | `$00` | distance/angle |
| `RIGHT` *n* | `$03` | angle |
| `WRIST UP` *n* | `$04` | value |
| `WRIST DOWN` *n* | `$05` | value |
| `ARMS UP` *n* | `$06` | value |
| `ARMS DOWN` *n* | `$07` | value |
| `CLAW ROTATE` *n* | `$08` | value |
| `CLAW OPEN` | `$09` | `$00` |
| `CLAW CLOSE` | `$09` | `$01` |
| `LAMP ON` / `OFF` | `$0A` | `$01` / `$00` |
| `HOME` | `$0B` | `$00` |
| `PLAY` *n* | `$81` | tune index |
| `SPEAK` *n* | `$82` | ROM phrase |
| `SAY` *n* | `$83` | RAM phrase slot |
| `END` | `$FF` | `$FF` |

---

## Copyright strings

The compiler writes a 17-byte copyright field at ROM offset **`$02`**. Choose with **`--copyright`**:

| Value | String | Typical use |
|-------|--------|-------------|
| `ultramaxx` | `(c) UltraMaxx    ` | Community cartridges |
| `cbs` | `(c) 1985 CBS Toys` | CBS factory branding |

Cartridge detection in internal ROM compares this field during warm start.

---

## Phrase and music tables

Programs that use **`SAY`** or **`PLAY`** need matching data in the cart tables at **`$A081`** and **`$A0BB`**. The compiler fills these regions with **`$FF`** / **`$00`** unless you supply a reference image:

Example ([`ultramaxx.bas`](../../Cartridge/Examples/UltraMaxx/Firmware/Basic/ultramaxx.bas), tables from [`UltraMaxx.532`](../../Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532)):

```bash
maxx compile ultramaxx.bas -o UltraMaxx.bas.532 \
  --copyright ultramaxx \
  --tables-from Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532
```

This copies phrase and music bytes from the reference ROM while replacing the program table with your compiled source. The full factory demo in [`ultramaxx.bas`](../../Cartridge/Examples/UltraMaxx/Firmware/Basic/ultramaxx.bas) compiles **byte-identical** to stock [`UltraMaxx.532`](../../Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532) when tables are copied from that file.

Custom speech authoring (phoneme encoding) remains future work — see [Chapter 4](04-Programming-Speech-and-Music.md) and [`Cartridge/PROGRAMMING.md`](../../Cartridge/PROGRAMMING.md) §10.

---

## Sample programs

### Minimal program

```basic
DELAY 1
FORWARD 20
LAMP ON
DELAY 3
LAMP OFF
HOME
END
```

Source: [`hello.bas`](../../Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas).

### Factory demo (38 steps)

The complete CBS/UltraMaxx demonstration sequence — speech, motion, lamp, tunes, home, and backup — is available as:

| Branding | Source |
|----------|--------|
| CBS Toys | [`cbsdemo.bas`](../../Cartridge/Examples/CBSDemo/Firmware/Basic/cbsdemo.bas) |
| UltraMaxx | [`ultramaxx.bas`](../../Cartridge/Examples/UltraMaxx/Firmware/Basic/ultramaxx.bas) |

Both sources compile to the same motion program; only the copyright string differs.

---

## Compiler errors

UltraMaxx BASIC reports line-numbered errors at compile time (not on the robot display).

| Message | Cause |
|---------|-------|
| `empty program` | No statements in file |
| `unknown statement` | Keyword not in vocabulary (e.g. `GOTO`, `PRINT`) |
| `operand … out of range 0..255` | Numeric operand too large |
| `program too large` | More than 38 pairs |
| `DELAY requires one operand` | Missing or extra operands |
| `LAMP requires ON or OFF` | Invalid lamp sub-keyword |
| `validation failed` | Emitted image failed cart structure checks |

Use **`maxx check`** *file* to validate syntax without writing a `.532` file.

---

## What UltraMaxx BASIC is not

The following Commodore 64 BASIC features are **intentionally absent**:

| C64 BASIC | UltraMaxx BASIC |
|-----------|-----------------|
| Variables (`A`, `X$`) | Not supported |
| `IF` / `THEN` | Not supported |
| `GOTO` / `GOSUB` | Not supported |
| `FOR` / `NEXT` | Not supported |
| Arithmetic expressions | Not supported |
| `INPUT` / `READ` / `DATA` | Not supported |
| String functions | Not supported |
| Inline interpreter | Not supported — compile only |

Control flow is **strictly linear**: the executor runs steps in order from `$0200` until **`END`**.

---

## Relationship to other chapters

| Topic | See |
|-------|-----|
| Bytecode rules, modes, RAM limits | [Chapter 1](01-Bytecode-Programming-Rules.md) |
| Opcode hex reference | [Chapter 2](02-Opcode-Vocabulary.md) |
| Motion operand tuning | [Chapter 3](03-Programming-Motion-and-Display.md) |
| Speech phonemes, music bytes | [Chapter 4](04-Programming-Speech-and-Music.md) |
| Bootstrap stub, ROM map | [Chapter 5](05-Cartridge-Bootstrap-and-Internal-ROM.md) |
| Cartridge authoring workflow | [`Cartridge/PROGRAMMING.md`](../../Cartridge/PROGRAMMING.md) |
| PicoROM upload | [`Cartridge/Examples/UltraMaxx/PICOROM.md`](../../Cartridge/Examples/UltraMaxx/PICOROM.md) |

---

## Quick command reference

All commands are provided by [`tools/maxx`](../../tools/maxx) (see [`tools/maxxbas/`](../../tools/maxxbas/) for the Rust implementation).

| Command | Purpose |
|---------|---------|
| `maxx compile` *file.bas* | Build `.532` image |
| `maxx check` *file.bas* | Syntax check |
| `maxx validate` *file.532* | Verify cart structure |
| `maxx list` *file.532* | Human-readable step list |
| `maxx list --json` | Machine-readable trace (simulators) |
| `maxx upload` *file* | Compile (if needed) + PicoROM |
| [`maxx simulate`](../../tools/maxx) *file* | Step preview ([`tools/maxxbas/`](../../tools/maxxbas/); add `--gui` for interactive playback) |

---

**Previous:** [Chapter 6 — Input/output guide](06-Input-Output-Guide.md) · **Next:** [Appendices](Appendices.md)