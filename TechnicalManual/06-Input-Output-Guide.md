# Chapter 6 — Input/Output Guide

This chapter describes memory-mapped hardware, the remote keypad, cartridge slot, and the 27 MHz radio control link.

---

## Memory map summary

| Region | Address | Purpose |
|--------|---------|---------|
| Zero page | `$0000–$00FF` | Flags, pointers, keypad state |
| Warm start | `$0100–$0103` | Copyright mirror |
| Stack | `$01FF` ↓ | 6502 stack |
| Program RAM | `$0200–$03FE` | Bytecode (2 bytes/step) |
| Music RAM | `$0400–$04FF` | Note pairs |
| Speech RAM | `$0500+` | Custom phrases |
| Timer / misc | `$1000` | I/O |
| Display | `$1200` | Shift-register LEDs |
| Speech chip | `$1400` | Parallel phoneme output |
| Motors | `$1600`, `$1C00` | Drive and joint timing |
| Cartridge ROM | `$2000–$B000` | 4 KB slot stepping |
| Internal ROM | `$E000–$FFFF` | 8 KB operating system |

Full table: [Appendix C](Appendices.md#c-memory-map).

---

## Remote keypad (RF input)

The handheld transmitter uses a **COP411L** MCU scanning a row/column matrix. Key events are encoded as **27 MHz OOK** packets received by the robot superhet strip.

### Matrix layout

| | L7 | L6 | L5 | L4 | PK |
|--|----|----|----|----|-----|
| D0 | A | B | C | D | |
| D1 | E | F | G | H | |
| L0 | I | J | K | L | |
| L1 | M | N | O | P | |
| L2 | Q | R | S | T | |
| L3 | U | V | W | X | |
| Gnd | | | | | Y |

Faceplate labels (from [`Remote-Front.jpg`](../Transmitter/Photos/Product/Remote-Front.jpg)): **U/1/2/3 DRIVE**, **4/5 WRIST**, **6/7 ARMS**, **8/9 CLAW**, **A LAMP**, **B HOME**, **NOTE REST** (WAIT), **SHIFT OCTAVE**, **CLEAR**, **ENTER**, **SONG/NOTES**, **CLOCK/STATUS**, **SPEECH**, **MOTION**, **GAME**, **PROGRAM**, **LEARN**, **EXECUTE**, **POWER/STOP** (**Y**).

Full key-to-matrix table: [`Transmitter/remote-keypad.md`](../Transmitter/remote-keypad.md). Diagram: [`Remote-Front.svg`](../Transmitter/Photos/Product/Remote-Front.svg).

Matrix wiring refs: [`keyboard-matrix-reference-1.png`](../Transmitter/Photos/ReverseEngineering/keyboard-matrix-reference-1.png), [`keyboard-matrix-reference-2.png`](../Transmitter/Photos/ReverseEngineering/keyboard-matrix-reference-2.png).

### RF link (summary)

- Carrier: **27 MHz** on-off keying
- MCU clock: **455 kHz** ceramic resonator (IF reference)
- Bit period: ~**1.55 ms**; packet repeat ~**29 ms** (Power/Stop ~**21 ms**)

Details: [`Transmitter/transmitter-architecture.md`](../Transmitter/transmitter-architecture.md), [`tools/rfcap/`](../tools/rfcap/).

---

## Cartridge expansion

| Property | Value |
|----------|-------|
| ROM size | 4 KB per image |
| Mapping | `$2000`, `$6000`, `$A000`, … (4 KB steps) |
| Demo cart base | `$A000` |
| Required header | 17-byte CBS Toys copyright at offset +2 |

Hardware EPROM: see [`DataSheets/Mitsubishi-KM2365.pdf`](../DataSheets/Mitsubishi-KM2365.pdf).

---

## On-board speech and face hardware

Speech synthesis ICs documented in [`DataSheets/`](../DataSheets/) include Sanyo LC3100/LC8100 and Eurotechnique ET9420 family parts. The CPU writes phoneme nybbles to **`$1400`**.

Face/display COP41xL subsystem: [`National-COP41xL-Display-Motors.pdf`](../DataSheets/National-COP41xL-Display-Motors.pdf) (refdes **U500** in archive naming).

---

## CPU and support chips

| Part | Role | Datasheet |
|------|------|-----------|
| MOS 6502 | Main CPU | [`MOS-6502.pdf`](../DataSheets/MOS-6502.pdf) |
| COP420 family | Auxiliary MCU | [`National-COP420.pdf`](../DataSheets/National-COP420.pdf) |
| M6116 | Static RAM | [`Mitsubishi-M6116.pdf`](../DataSheets/Mitsubishi-M6116.pdf) |

CPU pinout notes (project doc): [`Maxx-Steele-CPU-Pinout.pdf`](../DataSheets/Maxx-Steele-CPU-Pinout.pdf).

---

## Schematics

See [Schematic diagram index](Schematics.md) for board-level drawings.

---

**Previous:** [Chapter 5](05-Cartridge-Bootstrap-and-Internal-ROM.md) · **Next:** [Appendices](Appendices.md)