# Connection inventory

The wired cartridge is [`tscircuit/pico-dip245.circuit.tsx`](tscircuit/pico-dip245.circuit.tsx). Its J400 names come from [`J400-cartridge-port-pinout.csv`](J400-cartridge-port-pinout.csv), the chosen reference mapping. `npm test` checks that A0–A15, D0–D7, and OEB meet the Pico only through the DIP 245s, that WRB stops on `WRB_NO_FREE_GPIO`, that VBAT and VSW are each a single pin, that the HCT 5 V net is unsourced and is not VSW, and that OEB is not the HCT output-enable net. That test is a netlist check of the chosen map. It is not a continuity test.

## P28 reference carrier

Status of every net in [`tscircuit/picorom-carrier.circuit.tsx`](tscircuit/picorom-carrier.circuit.tsx). This sheet is not the chosen board. Confidence below is about the **P28 module pin**, not about the Maxx robot.

Tooling that rebuilds this list: `npm test` in `tscircuit/`. That test fails if any P28 net is tied to any J1 net.

## Known: P28 DIP-28 pin functions

Source: pad nets on `J3` and `J4` in `hardware/PicoROM-28/PicoROM-28.kicad_pcb` at commit `dddab41f9a47c7666115837b50b93eb78c221e33`. `J3` pin 1 is DIP pin 1. `J4` pin 1 is DIP pin 28, then the DIP number counts down. The order matches the 27C512 pin names used by firmware targets `P28_27C512_70ns` and `P28_27C512_100ns`. This table is **not** a Maxx edge map.

| DIP pin | PCB net | Carrier net | In this schematic |
|---|---|---|---|
| 1 | `PIN_A15` | `P28_A15_OPEN` | open |
| 2 | `PIN_A12` | `P28_A12_OPEN` | open |
| 3 | `PIN_A7` | `P28_A7_OPEN` | open |
| 4 | `PIN_A6` | `P28_A6_OPEN` | open |
| 5 | `PIN_A5` | `P28_A5_OPEN` | open |
| 6 | `PIN_A4` | `P28_A4_OPEN` | open |
| 7 | `PIN_A3` | `P28_A3_OPEN` | open |
| 8 | `PIN_A2` | `P28_A2_OPEN` | open |
| 9 | `PIN_A1` | `P28_A1_OPEN` | open |
| 10 | `PIN_A0` | `P28_A0_OPEN` | open |
| 11 | `PIN_D0` | `P28_D0_OPEN` | open |
| 12 | `PIN_D1` | `P28_D1_OPEN` | open |
| 13 | `PIN_D2` | `P28_D2_OPEN` | open |
| 14 | `GND` | `P28_GND_OPEN` | open. Ground finger on J1 is unknown. |
| 15 | `PIN_D3` | `P28_D3_OPEN` | open |
| 16 | `PIN_D4` | `P28_D4_OPEN` | open |
| 17 | `PIN_D5` | `P28_D5_OPEN` | open |
| 18 | `PIN_D6` | `P28_D6_OPEN` | open |
| 19 | `PIN_D7` | `P28_D7_OPEN` | open |
| 20 | `~{PIN_CE}` | `P28_nCE_OPEN` | open. Active-low chip enable into the module. |
| 21 | `PIN_A10` | `P28_A10_OPEN` | open |
| 22 | `~{PIN_OE}` | `P28_nOE_OPEN` | open. Active-low output enable into the module. |
| 23 | `PIN_A11` | `P28_A11_OPEN` | open |
| 24 | `PIN_A9` | `P28_A9_OPEN` | open |
| 25 | `PIN_A8` | `P28_A8_OPEN` | open |
| 26 | `PIN_A13` | `P28_A13_OPEN` | open |
| 27 | `PIN_A14` | `P28_A14_OPEN` | open |
| 28 | `+5V_SYS` | `P28_VCC_PIN28_OPEN` | open. Upstream treats this pad as the 5 V host supply for the buffers. |

Inside the module (not redrawn as carrier parts), those ROM-side nets meet the buffers described in the README. RP2040 pad nets at the same commit, lined up with `BOARD_28P` GPIO numbers:

| GPIO | PCB net on the RP2040 | Function in firmware |
|---|---|---|
| 0–15 | `A0`–`A15` | address in |
| 16–23 | `D0`–`D7` | data out |
| 24 | `~{OE}` | DIP `/OE`, active low |
| 25 | `~{CE}` | DIP `/CE`, active low |
| 26 | `~{BUF_OE}` | data-buffer output enable |
| 27 | `BUF_DIR` | held high in `rom.cpp` |
| 28 | `EXT_RESET_EN` | enable for the two reset buffers |
| 29 | `INFO_LED1` | activity LED |

## Known: mechanical edge only

The cartridge STEP model has an edge connector of **44 contacts, 2.54 mm pitch, 22 per face**. `J1` in the schematic has 44 pins for that count. Indexes `IDX01`–`IDX44` are drawing labels. They are not silk numbers from `Cartridge/Photos/maxxcard.jpg`, and they are not a row A / row B physical order.

## Unresolved: every J1 finger

| Drawing index | Carrier net | Signal |
|---|---|---|
| IDX01–IDX44 | `UNRESOLVED_J1_01`–`UNRESOLVED_J1_44` | Unknown. Do not assign D0–D7, A0–A15, `/CE`, `/OE`, VCC, or GND. |

The factory photo shows silk **1–22 on one face**. The other face is not in that photo. The trace worksheet's bus guess (D0–D7 on fingers 1–8, and so on) is explicitly low confidence and is **not** copied into this schematic.

Also still unknown, so not wired:

- Which finger is 5 V, and the measured voltage on it.
- Which finger is ground.
- Whether `/CE` is a decode of `$A000`, a raw 6502 select, or something else. Factory U3 is still the unidentified 24-pin part marked `5085`. U2's part number is still illegible. Both stay in the KiCad reference only.
- Which address bits the robot drives into a 64 KB P28 image. Firmware `ROM_SIZE` for `BOARD_28P` is `0x10000` (64 KB). The robot's cartridge window documented in firmware is 4 KB. Both can be true; the address wiring that would select the window is not known. Do not pick the `4kb` versus `27c512` PicoROM upload token from this schematic.
- Reset. P28's two reset pads are `EXT_RESET_HIGH` and `EXT_RESET_LOW` on `J2`. `peripherals.cpp` drives the single `RESET_PIN` high for both the "high" and "low" software settings, and low for `z`. The two 74AHCT1G126 outputs are opposite levels and share one enable, with a 1 kΩ pull-up (`R29`) from that enable to `+3V3`. Whether the Maxx cartridge needs either pad is unverified. They are not on the DIP-28 and are not in the carrier netlist.
- DIR and `/OE` strap pins of U4/U5/U9. They do not appear as named signal nets on the extracted pads (VCC is `+3V3_SYS`; several pads are GND or unnamed unconnected pads). Do not invent which pad is DIR.

## Assumed, and marked as such

- Drawing a DIP-28 symbol for the P28 module assumes the two 1×14 headers in the upstream PCB are the two sides of that DIP. The net order above is what makes that assumption checkable.
- Placeholder `dip28` and 44-pin header footprints, and their `pcbX` / `pcbY` values, are build scaffolding. They are not a cartridge outline, a card-edge footprint, or a placement proposal.
- No electrical assumption is made that the robot bus is 5 V TTL beyond the two facts already cited (27C512 on the factory card, `+5V_SYS` on P28 pin 28). Slot voltage is unmeasured.

## Still required before any finger is connected

1. Bottom copper photo and/or a continuity log from all 44 edge contacts to the factory ROM socket, with the method and date (#46).
2. A decision, after that log, whether the carrier replaces the factory glue (U2/U3) or still depends on it.
3. Confirmation that P28 `/CE` and `/OE` timing is acceptable. Upstream quotes 70 ns address-to-data and 40 ns OE-to-data as its own measurements. This repo has no scope capture on the robot.
4. Part markings for any extra line driver. Until those exist, do not add a 74-series part "because it is 3.3 V compatible."
5. Mechanical check that a P28 module and its USB-C plug fit the shell (#49). USB is how `picorom` 2.x loads an image; this schematic does not route USB.
