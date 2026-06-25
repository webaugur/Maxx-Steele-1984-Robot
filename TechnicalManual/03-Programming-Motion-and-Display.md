# Chapter 3 — Programming Motion and Display

This chapter describes motion opcodes, the headlamp, and how the LED display shows program steps during entry and execution.

Distance and angle **operands are empirical** — the internal ROM scales them to motor timing. Values in the factory demo (e.g. `$14` forward, `$28` arm raise) are starting points, not documented engineering units.

---

## Motion overview

Drive and joint motion opcodes `$00`–`$09` write timing values to motor control hardware mapped around **`$1600`** and **`$1C00`** (see [Chapter 6](06-Input-Output-Guide.md)).

The executor runs each step to completion (or until interrupted) before advancing the program pointer at `$0200`.

---

## Drive opcodes

| Opcode | Action | Demo cart examples |
|--------|--------|-------------------|
| `$00` | Turn left | `00 05` |
| `$01` | Forward | `01 14`, `01 10` |
| `$02` | Reverse | `02 14` |
| `$03` | Turn right | `03 06` |

Operands control how long or how far the drive system runs. Compare steps in [`maxx_demo_ROM_532.dsm`](../Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm) with on-robot behavior when calibrating new values.

---

## Arm and wrist opcodes

| Opcode | Action | Demo example |
|--------|--------|--------------|
| `$04` | Wrist up | — |
| `$05` | Wrist down | `05 23` |
| `$06` | Arms up | `06 28` |
| `$07` | Arms down | — |
| `$08` | Claw rotate | `08 15` |
| `$09` | Claw open/close | `09 00` (open) |

---

## Home opcode `$0B`

Always use operand **`00`**:

```
0B 00    ; home — all joints to reference position
```

The demo cart homes before backing up at the end of the sequence.

---

## Lamp opcode `$0A`

| Operand | Effect |
|---------|--------|
| `$00` | Lamp off |
| `$01` | Lamp on |

Demo sequence: `0A 01` … delay … `0A 00`.

---

## LED segment display

During program entry and execution, the internal ROM fetches a **two-byte display code** from table **`$F878`** indexed by the current opcode (see `LDA $F878,Y` in [`maxx_internal_ROM.dsm`](../Chassis/Firmware/Assembly/maxx_internal_ROM.dsm)). This drives the shift-register display at **`$1200`**.

Display names in [Chapter 2](02-Opcode-Vocabulary.md) match that table (e.g. **L**, **F**, **PLAY**, **d**).

When program RAM is full, the display spells **FULL** (bytes at internal ROM label referencing `$037F` limit).

---

## Programming techniques

1. **Home after motion blocks** — reduces accumulated joint error.
2. **Delay after every drive** — allows the chassis to settle before the next command.
3. **Lamp as status** — signal “busy” during long speech delays.
4. **Small operand sweeps** — test `$01 08`, `$01 10`, `$01 14` on your robot and record what works.

---

## Known gaps

- Exact mm/degree mapping for operands is not documented in the internal ROM listing.
- Motor ramp profiles and limit switches are hardware-dependent; see [`Mainboard/Schematic/`](../Mainboard/Schematic/) and [`Mainboard/KiCAD/`](../Mainboard/KiCAD/) (digitization in progress).

---

**Previous:** [Chapter 2](02-Opcode-Vocabulary.md) · **Next:** [Chapter 4 — Speech and music](04-Programming-Speech-and-Music.md)