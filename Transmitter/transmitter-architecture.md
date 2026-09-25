# Maxx Steele 27 MHz Transmitter — Architecture

The remote transmitter is built around a **National Semiconductor COP411L** 4-bit microcontroller (U1) driving a discrete **27 MHz on-off keying (OOK)** RF stage. The unusual part of the design is how the MCU is clocked.

## 455 kHz IF clock (not a normal CPU speed)

Most microcontrollers run from a crystal in the **megahertz** range. The Maxx transmitter instead uses **Y700**, a **455 kHz ceramic resonator**, wired directly to the COP411L **CKI** (clock input) pin.

| Ref | Part | Role |
|-----|------|------|
| Y700 | 455 kHz ceramic resonator | MCU clock source |
| U1 | COP411L-PAC/N | Keyboard scan, packet encode, RF keying |
| R10, R11 | 33 kΩ, 1 kΩ | Resonator bias / CKI network |

This frequency is deliberate: **455 kHz is the standard AM intermediate-frequency (IF)** used in the robot's 27 MHz receiver strip. The transmitter MCU is clocked at the **same IF reference** the receiver uses after RF mix-down, rather than at an unrelated CPU speed.

### Why clock the MCU from the IF frequency?

Commands are sent as **OOK** — the 27 MHz carrier is keyed on and off in a fixed bit pattern ([`Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf`](ReverseEngineering/Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf)). Measured on-air:

- Each bit is approximately **1.55 ms** (carrier present = `1`, absent = `0`)
- While a button is held, the same packet repeats every **~29 ms** (Power/Stop repeats every **~21 ms**)

Because bit periods, packet framing, and repeat timing are all derived from **MCU instruction cycles**, running the COP411L at **455 kHz** ties the serial OOK timeline to the **IF clock rate** the receiver already expects.

That keeps the transmitted envelope **phase-coherent** with the demodulated IF waveform: the on/off transitions land on predictable positions relative to the 455 kHz IF phase, instead of drifting against an arbitrary MCU crystal. In practice this gives the receiver a stable, repeatable OOK pattern to detect through its 455 kHz IF chain.

```
  Keyboard matrix ──► COP411L @ 455 kHz (CKI ← Y700)
                           │
                           ▼
                    OOK bit stream (~1.55 ms/bit)
                           │
                           ▼
              Q700–Q702 27 MHz RF keying stage
                           │
                           ▼
                    27 MHz OOK over the air
                           │
                           ▼
              Robot receiver → mix-down → 455 kHz IF → data out
```

## Signal path (simplified)

1. **Power** — 9 V battery, regulated to ~5.6 V (D1 zener + Q701).
2. **Input** — 4×6 keyboard matrix on J2; Power/Stop on COP411 **PK** (wake/interrupt).
3. **Encode** — COP411L firmware (mask ROM) scans keys and assembles OOK packets.
4. **RF output** — GPIO keying through Q700 (PNP driver), Q702 (NPN output), resonant network (C1, C3, etc.).
5. **Activity LED** — CR701 lights when transmitting.

**DataOut** on J2 is the logic-level OOK stream fed to the RF stage (also available on the 3.5 mm programming jack for external injection).

## Related files

| Path | Contents |
|------|----------|
| [`KiCAD/Transmitter-27MHz.kicad_pro`](KiCAD/Transmitter-27MHz.kicad_pro) | KiCad 10 project (Y700 → CKI, RF chain) |
| [`transmitter-bom.md`](transmitter-bom.md) | Bill of materials |
| [`tools/rfcap/README.md`](../tools/rfcap/README.md) | GNU Radio IQ captures of live OOK packets |
| [GNU Radio OOK capture demo](https://www.instagram.com/p/CLOjig8nCJS/) | Screen recording of the `RemoteSpectrum` flowgraph plotting RF data |
| [`DataSheets/National-COP411L.pdf`](../DataSheets/National-COP411L.pdf) | COP411L datasheet |
| [`Firmware/Arduino/`](Firmware/Arduino/) | Bare ATmega328P keypad scanner, nRF905 remote |

## COP411L clock notes

The COP411L divides CKI internally; instruction cycle time scales directly with the 455 kHz input. At this clock rate the MCU is slow by modern standards, but sufficient for keyboard debounce, short packet assembly, and bit-banged RF keying — and it matches the receiver IF reference by design.

Unused COP411 pins on the production board (MOSI, SCK, G2, NRESET) are left unconnected per the original layout.

## Envelope baud rate

The OOK cell is the symbol. There is no UART start or stop bit. The 27 MHz figure is the RF carrier, not this rate.

| Figure | Value |
|--------|--------|
| Measured cell | ~1.55 ms |
| Baud from that cell | 1 / 1.55 ms ≈ **645 baud** |
| COP411L instruction time | 455 kHz ÷ 8 ≈ 17.6 µs |

455 kHz sits in the COP411L divide-by-8 CKI window (0.2–0.5 MHz). One instruction is about 17.6 µs, inside the datasheet’s 16–40 µs instruction cycle. A divide-by-4 cycle at this resonator would be about 8.8 µs, which is shorter than that minimum.

1.55 ms / 17.6 µs ≈ 88 instructions. An 88-cycle cell is 88 × 8 / 455000 = **1.547 ms**, or **646 baud**. The published 1.55 ms measurement and that instruction count are the same cell.

Most keys are 11 bits (about 17 ms of cells) inside a 29 ms repeat. Power/Stop is 13 bits (about 20.2 ms) inside a 21 ms repeat. The cell rate stays about 645 baud. The idle gap between packets is part of the frame, not extra baud.

The replacement link does not send this waveform over the air. [`Firmware/Arduino/`](Firmware/Arduino/) sends key letters on an nRF905. [`Receiver/Firmware/Arduino/`](../Receiver/Firmware/Arduino/) rebuilds the 1.55 ms cells on RadioIn. `BIT_US` there starts at 1550.