# Arduino receiver (nRF905 → RadioIn)

Companion to [`Transmitter/Firmware/Arduino/maxx_remote`](../../../Transmitter/Firmware/Arduino/maxx_remote/maxx_remote.ino). [`maxx_rx/maxx_rx.ino`](maxx_rx/maxx_rx.ino) receives the same two-byte packets and bit-bangs the original OOK envelope into the robot’s **RadioIn** pin (the receiver module’s Data Out).

The 455 kHz IF is not used. Cell timing comes from the Arduino `micros()` clock. The 6502 still decodes the envelope into zero-page `$75`.

## Chip

Bare **ATtiny85** at 3.3 V, internal 8 MHz oscillator. No crystal: the bit cell is `micros()` on that RC clock. Build with ATTinyCore, board **ATtiny85**, clock **8 MHz (internal)**. Burn the bootloader once so the fuses match that clock (`CKDIV8` off). `F_CPU` must be 8000000 or the sketch will not compile.

The chip has no regulator. Feed a 3.3 V regulator from the robot’s VSW, and put 0.1 µF across the tiny85 VCC and GND. The nRF905 uses that same 3.3 V rail.

Program the chip with the nRF905 unpowered. ISP uses PB0, PB1, PB2, and RESET, which are also the radio SPI pins.

## Plug

The stock 27 MHz module uses a 3-pin 0.1 inch header. The reverse-engineering notes name the signals **VSW (+5 V)**, **Data Out**, and **GND**, and do not say which contact is which. Find them with a meter before applying power. VSW is +5 V, GND is 0 V, the remaining pin is Data Out.

Unplug the 27 MHz module so this board is the only driver on Data Out.

| Robot | Connect to |
|-------|------------|
| GND | ATtiny85 GND and nRF905 GND |
| VSW (+5 V) | 3.3 V regulator input, and the 2N2222 collector resistor |
| Data Out (RadioIn) | 2N2222 collector |

## 2N2222

Data Out has to reach 5 V. The tiny85 pin does not. One 2N2222 inverts, and `DATA_INVERT` is **1** so carrier still comes out high at the collector.

| 2N2222 | Connect to |
|--------|------------|
| Emitter | GND |
| Base | PB4 through 10 kΩ |
| Collector | Data Out, and 10 kΩ up to VSW |

## nRF905

Leave CD, AM, DR, and uPCLK open. Three mode pins are tied because the tiny85 has no spare GPIO:

| nRF905 | ATtiny85 |
|--------|----------|
| VCC | 3.3 V |
| GND | GND |
| MOSI | PB0 |
| MISO | PB1 |
| SCK | PB2 |
| CSN | PB3 |
| PWR_UP | 3.3 V |
| TRX_CE | 3.3 V |
| TX_EN | GND |

| Setting | Value |
|---------|--------|
| Frequency | 433.2 MHz (channel 108, HFREQ_PLL 0) |
| Address | `MAXX` |
| Payload | key `'A'`–`'Y'`, edge `'D'` or `'U'` |
| CRC | 16-bit |
| Crystal | 16 MHz |

## OOK timing

The cell this pin must reproduce is the original envelope rate: about **645 baud**, from the measured **1.55 ms** bit. An 88-instruction cell on the COP411L (455 kHz ÷ 8) is 1.547 ms, or 646 baud. The derivation is in [`transmitter-architecture.md`](../../../Transmitter/transmitter-architecture.md#envelope-baud-rate).

`BIT_US` starts at **1550** (645 baud). A 1 is carrier, a 0 is no carrier, left bit first. Most keys are 11 bits inside a **29 ms** frame. **Y** (Power/Stop) is 13 bits inside a **21 ms** frame, with no separate release word. The idle time after the last cell is part of that frame.

While `'D'` packets for that key are less than **80 ms** old, the frame free-runs on `micros()`. `'U'` sends the terminate word once. **Y** just stops. If the radio packets stop with no `'U'`, the sketch still sends one terminate word so the key does not stick. SPI runs in the idle gap, and only when more than 200 µs remains before the next cell.

If the robot drops bits, change `BIT_US` before changing the bit patterns. The ATtiny85 internal oscillator is this bit clock. The patterns are the table in [`Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf`](../../../Transmitter/ReverseEngineering/Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf). `'A'` is the first row of that table.

The tiny85 has no serial port. The air settings above must match the remote. The nRF905 ShockBurst rate (50 kbps on the 433.2 MHz carrier) is only the hop between the two sketches. RadioIn still runs at about 645 baud.
