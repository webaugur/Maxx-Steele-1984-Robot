# Arduino remote (factory keypad + nRF905)

Replacement for the mask-ROM COP411L and the 27 MHz OOK stage. The factory keypad stays. [`maxx_remote/maxx_remote.ino`](maxx_remote/maxx_remote.ino) scans it and sends key codes to an nRF905. The 455 kHz resonator is not used.

The matching robot-side sketch is [`Receiver/Firmware/Arduino/maxx_rx`](../../../Receiver/Firmware/Arduino/maxx_rx/maxx_rx.ino). It uses this payload and bit-bangs the old OOK envelope into RadioIn. The stock 27 MHz receiver does not hear these packets.

## Chip

Bare **ATmega328P** in DIP-28, at 3.3 V. Build it as **Arduino Pro or Pro Mini, ATmega328P (3.3 V, 8 MHz)**. That fuse setting expects an **8 MHz crystal** between DIP pins 9 and 10 (XTAL1, XTAL2), with about 22 pF from each leg to ground. A 16 MHz crystal is outside the 328P speed rating at 3.3 V.

The nRF905 supply and every I/O pin are **3.6 V absolute maximum**. The chip has no regulator. Feed a 3.3 V regulator from the 9 V battery, and run the 328P and the nRF905 from that rail. Put 0.1 µF across VCC (DIP 7) and GND, and tie AVCC (DIP 20) to 3.3 V. J2 pin 1 (+9 V) goes only to the regulator input.

Program the chip with the nRF905 unpowered. ISP uses the same MOSI, MISO, and SCK pins as the radio.

PWR_UP on the module is tied to 3.3 V. There is no GPIO for it.

## Keypad connector J2

Pin numbers are the KiCad netlist order in [`transmitter-bom.md`](../../transmitter-bom.md). Confirm molded pin 1 with a meter before soldering.

| J2 pin | Signal | 328P | DIP | Role |
|--------|--------|------|-----|------|
| 1 | +9 V | regulator input | — | Power only |
| 2 | DataOut | leave open | — | Old COP OOK line |
| 3 | GND | GND | 8 and 22 | Common ground |
| 4 | PK | PD2 | 4 | Power/Stop, active low. Internal pull-up. The old 1 kΩ pull-up (R8) is on the COP board. |
| 5 | L1 | PD6 | 12 | Row input, pull-up |
| 6 | L0 | PD5 | 11 | Row input, pull-up |
| 7 | L2 | PC4 | 27 | Row input, pull-up |
| 8 | L3 | PC5 | 28 | Row input, pull-up |
| 9 | L4 | PC0 | 23 | Column strobe |
| 10 | L5 | PC1 | 24 | Column strobe |
| 11 | L6 | PC2 | 25 | Column strobe |
| 12 | L7 | PC3 | 26 | Column strobe |
| 13 | D0 | PD3 | 5 | Row input, pull-up |
| 14 | D1 | PD4 | 6 | Row input, pull-up |

Matrix map (columns L7 L6 L5 L4, rows D0 D1 L0 L1 L2 L3), from [`keyboard-matrix-reference-1.png`](../../Photos/ReverseEngineering/keyboard-matrix-reference-1.png):

|  | L7 | L6 | L5 | L4 |
|--|----|----|----|----|
| D0 | A DRIVE | B DRIVE | C DRIVE | D DRIVE |
| D1 | E WRIST | F WRIST | G ARMS | H ARMS |
| L0 | I CLAW | J CLAW | K LAMP | L HOME |
| L1 | M NOTE REST | N SHIFT OCTAVE | O CLEAR | P ENTER |
| L2 | Q SONG/NOTES | R CLOCK/STATUS | S SPEECH | T MOTION |
| L3 | U GAME | V PROGRAM | W LEARN | X EXECUTE |

Power/Stop is **Y**. It shorts PK to ground and is not in the matrix. The membrane has no diodes, so two matrix keys at once can ghost. The sketch reports one key.

## nRF905

The module keeps its own crystal (this sketch assumes **16 MHz**). SPI SCK is an Arduino output used only during a transfer. Leave CD, AM, DR, and uPCLK open. DR is polled in the SPI status byte.

| nRF905 | 328P | DIP |
|--------|------|-----|
| VCC | 3.3 V | — |
| GND | GND | 8 and 22 |
| CSN | PB2 | 16 |
| MOSI | PB3 | 17 |
| MISO | PB4 | 18 |
| SCK | PB5 | 19 |
| TX_EN | PB1 | 15 |
| TRX_CE | PB0 | 14 |
| PWR_UP | 3.3 V | — |

The original 27 MHz envelope is about **645 baud** (1.55 ms per cell). This sketch does not reproduce that timing. It sends a key letter every 30 ms. The robot-side sketch rebuilds the 645 baud cells. Derivation: [`transmitter-architecture.md`](../../transmitter-architecture.md#envelope-baud-rate).

## What the sketch does

One column is driven low at a time. The other columns stay inputs so two keys cannot short two outputs. Power/Stop is read on its own pin and wins over the matrix. A key must hold still for **15 ms**. The letter is `'A'` plus the matrix index above, and `'Y'` for Power/Stop.

Each nRF905 packet is two bytes: the letter, then `'D'` while the key is held or `'U'` once on release. `'D'` repeats every **30 ms**. Serial at 9600 prints the same two characters. If Data Ready does not rise within **20 ms**, the sketch drops that packet and tries again on the next repeat. A failed config readback is retried once a second.

## Air settings

Change these in lockstep with the receiver sketch.

| Setting | Value |
|---------|--------|
| Band | 433 MHz, channel 108 → **433.2 MHz** |
| Formula | `(422.4 + channel / 10) × (1 + HFREQ_PLL)` MHz. HFREQ_PLL is 0 below 844.8 MHz and 1 from 844.8 MHz up (200 kHz steps). |
| Address | `MAXX` (`4D 41 58 58`), 4 bytes, same TX and RX |
| Payload | 2 bytes: key `'A'`–`'Y'`, edge `'D'` (held, repeated) or `'U'` (released) |
| CRC | 16-bit, enabled |
| PA | −10 dBm |
| Repeat while held | 30 ms |

Serial at 9600 on PD0 and PD1 (DIP pins 2 and 3) prints the same letters for bring-up. Those pins are not used by the keypad.
