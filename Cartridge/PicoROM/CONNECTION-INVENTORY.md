# Connection inventory

The cartridge schematic is [`tscircuit/picorom-p32-carrier.circuit.tsx`](tscircuit/picorom-p32-carrier.circuit.tsx). J400 names come from [`J400-cartridge-port-pinout.csv`](J400-cartridge-port-pinout.csv). `npm test` checks `COMPONENT_PINS`: each of A0–A15 and D0–D7 is one net between `J1` and `U_P32`, `OEB` is `nOE`, `/CE` plus A16 and A17 are ground, `VSW` is pin 32, and VPP and PGM are single-pin nets. That is a netlist check. It is not a continuity test and it is not a timing measurement.

## Module

`U_P32` is PicoROM-32 rev 1.10, `wickerwaka/PicoROM` commit `dddab41f9a47c7666115837b50b93eb78c221e33`. `J3` pin 1 is DIP pin 1. `J4` pin 1 is DIP pin 32, then the DIP number counts down. The connected pins match a 27C020, which is firmware target `P32_27C020_70ns` (`ROM_SIZE` `0x40000`).

| DIP | PCB net | Carrier |
|---|---|---|
| 1 | header pad, no copper net | `NC_VPP` |
| 2 | `PIN_A16` | `GND` (fixed) |
| 3–12, 23, 25–29 | `PIN_A15` … `PIN_A0`, `PIN_A10`, `PIN_A11`, `PIN_A9`, `PIN_A8`, `PIN_A13`, `PIN_A14` | same J400 address names |
| 13–15, 17–21 | `PIN_D0`–`PIN_D7` | J400 `D0`–`D7` |
| 16 | `GND` | J400 GND lines 1 and 43 |
| 22 | `~{PIN_CE}` | `GND` |
| 24 | `~{PIN_OE}` | J400 `OEB` |
| 30 | `PIN_A17` | `GND` (fixed) |
| 31 | header pad, no copper net | `NC_PGM` |
| 32 | `+5V_SYS` | `VSW_ASSUMED_5V` |

Buffers stay inside the module. U4/U5/U9 are `74LVC245` inputs from the DIP address pins. U10 is the 5 V data driver (MPN `C548858`) with DIR tied high. U7 (`74LVC1G32`) ORs `/CE` and `/OE` into the one enable the firmware samples. U11/U12 are the reset drivers for `J2`, which is not a DIP pin and is not on this carrier.

## Banking

A16 and A17 are tied low. Stock firmware still reads those GPIOs. With both low, A0–A15 select bytes `0x00000`–`0x0FFFF` of the 256 KB image. Nothing on this sheet switches banks.

## Still unknown

- Continuity from each J400 line to the mainboard, and which face is line 1 versus the photo silk.
- Whether `OEB` is the robot's only enable. `/CE` strapped low follows the reference map. It is not a trace of factory U2/U3.
- Which part of the low 64 KB the robot's cartridge window actually uses. The edge wires include A12–A15 because those fingers are on the map.
- Measured VSW, including spikes.
- Robot-side timing. Upstream's 70 ns and 40 ns figures are its own measurements.
- Whether a P32 and its USB-C plug fit the shell (#49).
