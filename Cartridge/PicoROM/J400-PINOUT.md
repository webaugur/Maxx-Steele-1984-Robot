# J400 expansion-port labels (unverified)

Chosen reference mapping for the cartridge edge in [`tscircuit/picorom-p32-carrier.circuit.tsx`](tscircuit/picorom-p32-carrier.circuit.tsx). The same rows are in [`J400-cartridge-port-pinout.csv`](J400-cartridge-port-pinout.csv) and in [`../../Mainboard/Schematic/J400-cartridge-port-pinout.csv`](../../Mainboard/Schematic/J400-cartridge-port-pinout.csv).

This is a schematic assignment taken from the community guide and the CSV Notes column. It is not a continuity-tested pinout. No contact has been buzzed from the cartridge finger to a mainboard net, and the photo silk `1`–`22` has not been matched to these line numbers.

Source: community guide *The Reference Guide to Programming, Hacking, Modifying, and Restoring a Maxx Steele Personal Robot*, Google Doc `1RSxLh9rSbNDSOTUrXm4flMKmsFEoP6RWNXGQrrtQhjs`, tab `t.0`, revision `AHj4eMTerV6p8oM8x4ORN-S7N5q-3zin1iMaLm2nH32DLN3-UDgSbYavvxt2Mc04yVAXFFyXkbVy76Jb0n3s9g`. The guide's own heading is "Pinout for edge card based on the labels on the schematic. (this needs to be verified)". Full transcription and the coverage map are in [`Docs/Technical/08-Community-Reference-Guide.md`](../../Docs/Technical/08-Community-Reference-Guide.md).

These numbers are the guide's edge-card line numbers. They are not the left-to-right order of a drawing, and they are not the silk numbers `1`–`22` on the factory cartridge photo. Which face is which, and which end is line 1, still has to be measured with a continuity check against the mainboard connector.

## Guide table, copied

The description of line 16 is copied as printed: "Address Line 100". The name on that row is `A10`.

| Line | Name | Description as printed |
|---:|---|---|
| 1 | GND | Ground Line |
| 2 | A15 | Address Line 15 |
| 3 | A14 | Address Line 14 |
| 4 | A13 | Address Line 13 |
| 5 | A12 | Address Line 12 |
| 6 | A11 | Address Line 11 |
| 7 | A7 | Address Line 7 |
| 8 | A8 | Address Line 8 |
| 9 | A6 | Address Line 6 |
| 10 | A9 | Address Line 9 |
| 11 | A5 | Address Line 5 |
| 12 | OEB | Output Enable Line For Memory Access |
| 13 | A4 | Address Line 4 |
| 14 | WRB | Write Enable Line for RAM |
| 15 | A3 | Address Line 3 |
| 16 | A10 | Address Line 100 |
| 17 | A2 | Address Line 2 |
| 18 | A1 | Address Line1 |
| 19 | D7 | Data Line 7 |
| 20 | A0 | Address Line 0 |
| 21 | D6 | Data line 6 |
| 22 | D0 | Data Line 0 |
| 23 | D5 | Data Line 5 |
| 24 | D1 | Data Line 1 |
| 25 | D4 | Data Line 4 |
| 26 | D2 | Data Line 2 |
| 27 | Audio Input | Goes to the internal Audio Amplifier |
| 28 | D3 | Data line 3 |
| 29 | F2 | Unknown - Goes to CPU Pin 59 |
| 30 | SA0 | Sound ROM Address line 0 |
| 31 | WEB | Unknown - Goes to CPU Pin 23 |
| 32 | SA1 | Sound ROM Address line 1 |
| 33 | BIGSPACEB | Unknown - Goes to CPU Pin 38 |
| 34 | SA2 | Sound ROM Address line 2 |
| 35 | MNMIB | Non-Maskable Interrupt |
| 36 | SA3 | Sound ROM Address line 3 |
| 37 | RESETSYSB | Unknown - Goes to CPU Pin 40 |
| 38 | CT | Sound ROM CT Line |
| 39 | VBAT | Voltage from Battery (+10vDC) |
| 40 | DOUT | Sound ROM Data OUT |
| 41 | VSW | Voltage from the Regulator (+5vDC) |
| 42 | DREQ | Sound ROM Data Request |
| 43 | GND | Ground Line |
| 44 | ASTRB | Strobe Line for Sound ROM Addressing |

The sheet uses VSW as an assumed nominal 5 V rail into PicoROM-32 pin 32. Confirm it stays inside 4.5–5.5 V, including transients, before powering. VBAT is not connected. See [POWER.md](POWER.md).

## Conflicts kept on purpose

Earlier reads of the factory schematic crops did not match this table on every contact. The schematic uses the guide names. The earlier readings stay here:

| Line | Chosen name | Earlier crop reading recorded in the CSV Notes |
|---:|---|---|
| 12 | OEB | WRB |
| 14 | WRB | CE0 |
| 27 | Audio Input | AUDIO AMP |
| 33 | BIGSPACEB | BUS…B. Middle characters were unclear. |
| 35 | MNMIB | MEM…B. Exact label was unclear. |
| 37 | RESETSYSB | TEST…. Ending was unclear. |
| 40 | DOUT | DOT |
| 42 | DREQ | DCC…. Exact label was unclear. |
| 44 | ASTRB | AT…B. Exact label was unclear. |

Address-line digits on the scan were also ambiguous before this table: the contact the guide calls A11, and the contact it calls D0 (a crop had that data bit near 32 or 33 rather than line 22).

The same guide's CPU pin table disagrees with two of its own J400 notes:

| J400 note | CPU table in the same guide |
|---|---|
| BIGSPACEB "Goes to CPU Pin 38" | Pin 36 is `BigSpaceB`. Pin 38 is `C0`. |
| RESETSYSB "Goes to CPU Pin 40" | Pin 46 is `ResetSysB`. Pin 40 is `C2`. |
| Port name `MNMIB` | CPU pin 48 is spelled `MNIMB`. |

The CPU table also repeats pin 54 (`F244 Hz` and `MoDataClk`) and leaves pins 20 and 58 blank. The compiler wrote "Yes, I know there are errors here." None of those rows were corrected here.

`B` in `OEB` and `WRB` is the guide's spelling. It is not a measured active-low proof. On this sheet `OEB` is the PicoROM `/OE` pin. `WRB` is unused.

## What this schematic does with each line

The only active part is a PicoROM-32 module. There is no external HCT or LVC.

| J400 | Net | PicoROM-32 pin |
|---|---|---|
| A0–A15 | `J400_A0`–`J400_A15` | A0–A15 |
| D0–D7 | `J400_D0`–`J400_D7` | D0–D7 |
| OEB | `J400_OEB` | nOE (pin 24) |
| no chip-select finger | `GND` | nCE (pin 22), strapped low |
| no A16 / A17 finger | `GND` | A16 pin 2 and A17 pin 30, tied low |
| GND lines 1 and 43 | `GND` | pin 16 |
| VSW | `VSW_ASSUMED_5V` | pin 32 |
| VBAT | `J400_VBAT_NOT_LOGIC` | none |
| WRB, Audio Input, F2, SA0–SA3, WEB, BIGSPACEB, MNMIB, RESETSYSB, CT, DOUT, DREQ, ASTRB | `NC_*` | none |
| — | `NC_VPP`, `NC_PGM` | pins 1 and 31, open on the module |

The bare-Pico split GPIO map is not this design. This sheet uses stock `BOARD_32P`, not a Pico header.

## Still required before this is an electrical release

- Continuity from each J400 line to a known mainboard net, including which cartridge face and which end is line 1 versus the photo silk.
- Confirm assumed VSW is inside 4.5–5.5 V, including transients, before powering. See [POWER.md](POWER.md).
- Whether OEB is the only enable, given that `/CE` is strapped low.
- A scope capture on the robot. Upstream's 70 ns and 40 ns numbers are not that capture.
