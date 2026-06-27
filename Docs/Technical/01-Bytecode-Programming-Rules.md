# Chapter 1 — Bytecode Programming Rules

This chapter describes the rules for writing Maxx Steele **programs**: how bytecode is stored, how operating modes affect entry, and practical techniques for building sequences.

The robot does not run arbitrary user 6502 code for motion demos. The internal ROM interprets a **bytecode program** in RAM. Cartridges supply tables and a short bootstrap stub; see [Chapter 5](05-Cartridge-Bootstrap-and-Internal-ROM.md).

---

## Introduction

A Maxx program is a linear sequence of **(opcode, operand)** byte pairs. Each pair is one step. The executor reads from program RAM, displays the opcode name on the LED segment display, and performs the action.

Programs are edited through the remote keypad (Program mode), loaded from a cartridge bootstrap, or built offline and burned into EPROM.

---

## Program storage

| Property | Value |
|----------|-------|
| RAM region | `$0200`–`03FE` |
| Step size | 2 bytes (opcode, operand) |
| Maximum steps | 255 pairs (512 bytes); **FULL** displayed when `$037F` exceeded |
| Terminator | `FF FF` (required) |

Example minimal program:

```
83 10    ; speak RAM phrase 0
0C 03    ; delay 3 seconds
01 10    ; drive forward
0B 00    ; home all joints
FF FF    ; end
```

---

## Operating modes (`$0D`)

The mode byte in zero page selects how keypad input and the executor behave.

| Value | Name | Behavior |
|-------|------|----------|
| `$00` | Immediate | Keys execute motion/speech instantly |
| `$01` | Learn | Records key sequences into program RAM |
| `$02` | Program | Enter bytecode steps via keypad |
| `$03` | Execute / Game | Run the stored program |

Cartridge entry stubs typically initialize status bytes `$02` and `$03` before jumping to the internal main loop at `$E0B6`.

---

## Status bytes

### `$02` — Status A

Set by cartridge bootstrap (demo uses `LDA #$02` / `STA $02`). Bits include speech error (**SEr**), enable backoff (**Ebof**), power-down, drive, and speech flags. Keypad rows toggle subsets; see [Appendix E](Appendices.md#e-status-bits).

### `$03` — Status B

Demo entry uses `LDA #$82` / `STA $03` (**PDon**, **Edof**, **SPof**, **UCon**). Governs user control and execute gating.

---

## Program pointers (zero page)

| Addr | Role |
|------|------|
| `$0F` / `$10` | Current step in `$0200` table (program mode / execute) |
| `$11` / `$13` | Current opcode and operand during entry |
| `$24` / `$25` | Program byte pointer used by executor |

---

## Programming techniques

### Delays

Opcode `$0C` operand is delay time in **seconds** (decimal). Chain delays between motion commands so mechanics can finish.

```
0C 02    ; wait 2 seconds
01 14    ; forward (operand scale empirical — see Ch 3)
0C 04    ; wait 4 seconds
```

### Homing

Opcode `$0B` with operand **`$00` only** — returns joints to a known position before the next move.

### Speech then motion

RAM phrases (`$83`) must be copied to `$0500` before execute (cartridge stub or prior setup). ROM phrases (`$82`) need no RAM table.

### Program length

If program entry exceeds available RAM, the display shows **FULL** (internal ROM compares against `$037F`).

### Marker opcode `$FE`

Display-only **beg** marker; does not affect execution flow in the demo cart.

---

## Beginner checklist

1. Every program ends with `FF FF`.
2. Operands are one byte (`$00`–`FF`); meaning depends on opcode (Ch 2–4).
3. Test with `python3 tools/maxx_rom.py validate` (see [`tools/maxx_rom.py`](../../tools/maxx_rom.py)) before burning EPROM.
4. For a worked 38-step demo, see [`Cartridge/PROGRAMMING.md`](../../Cartridge/PROGRAMMING.md) §5.

---

**Next:** [Chapter 2 — Opcode vocabulary](02-Opcode-Vocabulary.md)