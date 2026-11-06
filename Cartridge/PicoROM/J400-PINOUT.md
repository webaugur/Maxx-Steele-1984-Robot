# J400 expansion-port labels (unverified)

Chosen reference mapping for the cartridge edge in [`tscircuit/pico-dip245.circuit.tsx`](tscircuit/pico-dip245.circuit.tsx). The same rows are in [`J400-cartridge-port-pinout.csv`](J400-cartridge-port-pinout.csv), the copy confirmed for this design. That file is the content saved under Tower5810 `Mainboard/Schematic`; this pull request does not edit that path.

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

`+10 V` and `+5 V` are the guide's claims. They were not measured for this schematic. `VBAT` and `VSW` stay on different nets. `VBAT` does not connect to the Pico or to either 245 supply.

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

`B` in `OEB` and `WRB` is the guide's spelling. It is not a measured active-low proof, and `OEB` is not wired to the data-buffer output enable.

## What the Pico schematic does with each line

Candidates, stock unconfirmed: three `SN74LVC245AN` (PDIP-20, VCC = Pico 3V3, DIR low so the B side is the J400 side) and one `SN74HCT245N` (PDIP-20, VCC and DIR on `J400_VSW_UNMEASURED`). 74HC245 is not in the netlist. Datasheet limits used for the split: SN74LVC245A VCC 1.65–3.6 V, inputs to 5.5 V, outputs only to VCC; SN74HCT245 VCC 4.5–5.5 V, VIH min 2.0 V, outputs swing to VCC. Do not power the HCT or Pico VSYS from `VSW` until that pin measures inside 4.5–5.5 V. A 5 V pull-up is not tied to a Pico GPIO. HCT `/OE` is pulled to the VSW net through 10 kΩ and is pulled down only by the NPN. Pico VSYS stays open.

| J400 | Net | Buffer | Pico |
|---|---|---|---|
| A0–A7 | `J400_A0`–`J400_A7` | `U_ADDR` B1–B8 | GP10–GP17 |
| A8–A12 | `J400_A8`–`J400_A12` | `U_AHI` B1–B5 | GP18–GP22 |
| A13–A15 | `J400_A13`–`J400_A15` | `U_AHI` B6–B8 | GP26–GP28 |
| OEB | `J400_OEB` | `U_CTL` B1 → A1 | GP8 |
| WRB | `J400_WRB` | `U_CTL` B2 → A2 | none. Net `WRB_NO_FREE_GPIO` |
| D0–D7 | `J400_D0`–`J400_D7` | `U_DATA` B side | GP0–GP7 on the A side |
| data OE | `HCT_nOE` | `U_DATA` pin 19, 10 kΩ to VSW, NPN collector | GP9 through 10 kΩ into the NPN base. 100 kΩ base pulldown. |
| GND lines 1 and 43 | `GND` | all 245 GND pins | Pico GND and AGND |
| VSW | `J400_VSW_UNMEASURED` | HCT VCC, HCT DIR, 100 nF | not VSYS |
| VBAT | `J400_VBAT_NOT_LOGIC` | none | none |
| Audio Input, F2, SA0–SA3, WEB, BIGSPACEB, MNMIB, RESETSYSB, CT, DOUT, DREQ, ASTRB | `J400_*_UNUSED` | none | none |

HCT `/OE` is pulled up to the unmeasured VSW net, so the buffer output is high-Z when the NPN is off. GP9 high turns the NPN on and drives `/OE` low. GP9 low, or a Pico that is unpowered, leaves the base pulldown holding the NPN off, so `/OE` is not a Pico pin and is not pulled up into a Pico GPIO. That is a startup proposal from the SN74HCT245 `/OE`-high = Hi-Z specification. It is not a measurement of VSW, and it is not permission to apply VSW.

A stock Pico breaks out GP0–GP22 and GP26–GP28 (26 pins). GP23 (SMPS), GP24 (VBUS), GP25 (LED), and GP29 (VSYS sense) are not free headers. Address plus data plus OEB is 25 pins, so WRB is buffered and then stops. Upstream PicoROM P28 firmware (`BOARD_28P`, address on GPIO0–15, data on GPIO16–23) does not match this map. The P28 sheet in this folder stays a reference module with its edge left open. No firmware for the split map has been compiled or flashed.

## Still required before this is an electrical release

- Continuity from each J400 line to a known mainboard net, including which cartridge face and which end is line 1 versus the photo silk.
- Voltage on VSW and VBAT with the robot on, before HCT power or any Pico supply connection.
- Whether OEB and WRB are active low, and whether OEB is the ROM output enable the firmware should sample.
- Markings on the DIP 245 parts actually on hand. DIR polarity above is the TI SN74LVC245A / SN74HCT245 polarity, not a substitute for those markings. A 74HC245 is not a substitute: at 5 V its outputs are about 5 V into a Pico pin, and a 3.3 V high is a marginal high into an HC input at 5 V.
- A firmware build that samples the split address and drives data from GP0–GP7. That build does not exist in this change.
