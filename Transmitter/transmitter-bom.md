# Maxx Steele 27 MHz Remote Transmitter — BOM

Authoritative bill of materials for the COP411-based remote transmitter, merged from:

- Handwritten transcription: [`handwritten-bom.jpg`](Photos/ReverseEngineering/handwritten-bom.jpg)
- PCB photo: [`mcu-board-pcb.jpg`](Photos/ReverseEngineering/mcu-board-pcb.jpg)
- KiCad project: [`Transmitter-27MHz.kicad_pro`](KiCAD/Transmitter-27MHz.kicad_pro) ([`Transmitter-27MHz.kicad_sch`](KiCAD/Transmitter-27MHz.kicad_sch))
- Reverse-engineering notes: [`Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf`](ReverseEngineering/Maxx-Steele-Transmitter-Reverse-Engineered-Notes.pdf)

## Active components

| Ref | Value | Package | Notes |
|-----|-------|---------|-------|
| U1 | COP411L-PAC/N | DIP-20 | National 4-bit MCU; 455 kHz operation ([`National-COP411L.pdf`](../DataSheets/National-COP411L.pdf)) |
| Q700 | S8550 (PNP) | TO-92 | RF driver; handwritten notes say "1S02N PNP" — S8550 is the KiCad/RE match |
| Q701 | 2N3904 (NPN) | TO-92 | Regulator / keying stage |
| Q702 | 2N3904 (NPN) | TO-92 | RF output |
| Q703 | 2N3904 (NPN) | TO-92 | Data output buffer |
| Y700 | 455 kHz ceramic resonator | 2-pin | MCU clock (IF reference — see [transmitter architecture](transmitter-architecture.md)) |
| D1 | 1N5232B (5.6 V zener) | DO-35 | Regulates 9 V down to ~5.6 V rail |
| CR701 | Red LED 3/4 size | 5 mm | TX activity indicator |

## Passives

| Ref | Value | Notes |
|-----|-------|-------|
| C1 | 0.002 µF (202) | RF network |
| C3 | 0.001 µF (102) | RF network |
| C4 | 1.5 µF electrolytic | Supply decoupling |
| R1 | 4.7 kΩ | COP411 I/O |
| R2 | 8.2 kΩ | Data output |
| R3 | 10 kΩ | Supply |
| R5 | 3.9 kΩ | Q700 bias |
| R6 | 220 Ω | Zener / regulator network |
| R7 | 3.3 kΩ | Zener bias |
| R8 | 1 kΩ | PK pull-up |
| R9 | 330 Ω | LED current limit |
| R10 | 33 kΩ | Resonator bias |
| R11 | 1 kΩ | CKI network |
| R12 | 2.7 kΩ | COP411 I/O |

## Connectors / power

| Ref | Value | Notes |
|-----|-------|-------|
| BT1 | 9 V battery | Snap connector |
| J1 | 2-pin header | +9 V in / GND to main remote PCB |
| J2 | 14-pin spring connector | Matrix bus to keyboard PCB |

## J2 pinout (from KiCad netlist)

| Pin | Signal |
|-----|--------|
| 1 | +9Vout |
| 2 | DataOut |
| 3 | GND |
| 4 | PK |
| 5 | L1 |
| 6 | L0 |
| 7 | L2 |
| 8 | L3 |
| 9 | L4 |
| 10 | L5 |
| 11 | L6 |
| 12 | L7 |
| 13 | D0 |
| 14 | D1 |

## Schematic status

The KiCad schematic in [`KiCAD/`](KiCAD/) has all listed components placed and 31 named nets including power, clock, RF, LED, and the 14-pin matrix interface. Unconnected COP411 pins (MOSI, SCK, G2, NRESET) are intentionally left as NoConn per the production board.