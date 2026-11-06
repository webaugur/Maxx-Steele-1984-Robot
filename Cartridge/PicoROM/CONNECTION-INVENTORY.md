# Connection inventory

The cartridge schematic is [`tscircuit/picorom-carrier.circuit.tsx`](tscircuit/picorom-carrier.circuit.tsx). J400 names come from [`J400-cartridge-port-pinout.csv`](J400-cartridge-port-pinout.csv). `npm test` checks that each address and data bit is one net between `J1` and `U_P28`, that `OEB` is `nOE`, that `nCE` is ground, that `VSW` is pin 28, and that the netlist has no external 74-series part. That is a netlist check. It is not a continuity test and it is not a timing measurement.

## Module

`U_P28` is one part: PicoROM-28 rev 1.10, `wickerwaka/PicoROM` commit `dddab41f9a47c7666115837b50b93eb78c221e33`. DIP pin numbers are the PCB headers `J3` pin 1 = DIP pin 1 and `J4` pin 1 = DIP pin 28. They match a 27C512, which is the `BOARD_28P` image (`ROM_SIZE` `0x10000`).

The module's own buffers stay inside the module:

| Ref | Fitted value | What the PCB nets do |
|---|---|---|
| U4, U5, U9 | `74LVC245`, VCC `+3V3_SYS`, DIR pad grounded, `/OE` pad grounded | Robot address, `/CE`, and `/OE` into the RP2040. DIR low makes the DIP side the input. |
| U10 | Value text `74LVC8245`, MPN `C548858` (Nexperia 74LVC8T245BQ,118), footprint DHVQFN-24. Symbol library still says `SN74AVC8T245PW`. | A side `D0`–`D7` on `+3V3_SYS`. B side `PIN_D0`–`PIN_D7` on `+5V_SYS`. |
| U7, U11 | `74AHCT1G126` on `+5V_SYS` | Reset drivers for `J2`, which is not a DIP pin and is not on this carrier. |

`firmware/rom.cpp` sets `BUF_DIR` high. On the 74LVC8T245 that is A toward B, so the RP2040 drives the 5 V data pins. `data_bus.pio` program `or_output_enable` drives the buffer `/OE` only when both DIP `/CE` and `/OE` read low.

## Carrier nets

| P28 pin | J400 line | Net |
|---|---|---|
| A0–A15 | same names | `J400_A0`–`J400_A15` |
| D0–D7 | same names | `J400_D0`–`J400_D7` |
| nOE (pin 22) | 12 `OEB` | `J400_OEB` |
| nCE (pin 20) | none | `GND`. The reference map has no chip-select finger. |
| VCC (pin 28) | 41 `VSW` | `VSW_ASSUMED_5V` |
| GND (pin 14) | 1 and 43 `GND` | `GND` |
| — | 39 `VBAT` | `J400_VBAT_NOT_LOGIC` |
| — | `WRB` and the sound, audio, and CPU straps | `NC_*` |

`WRB` is not a 27C512 pin on this module. Pin 27 is A14.

## Still unknown

- Continuity from each J400 line to the mainboard, and which face is line 1 versus the photo silk.
- Whether `OEB` really is the only enable the robot uses. Strapping `/CE` low follows the reference map. It is not a trace of the factory U2/U3 decode.
- Which 4 KB of the 64 KB image the robot addresses. The wires include A12–A15 because those fingers are on the map.
- Measured VSW, including spikes.
- Robot-side timing. Upstream's 70 ns and 40 ns figures are its own measurements.
- Whether a P28 and its USB-C plug fit the shell (#49). Reset header `J2` is unused.
