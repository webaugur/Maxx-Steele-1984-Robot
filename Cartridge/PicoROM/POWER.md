# Cartridge power: present topology and the open choice

No 7805 and no other regulator is on the schematic. None was requested. The signal map is unchanged.

## What the sheet does now

| Net | What is on it | What is not |
|---|---|---|
| Pico `3V3` | Three `SN74LVC245AN` VCC pins, their 100 nF capacitors, and the Pico 3V3 pin | HCT, VSW, VBAT |
| `HCT_5V_UNSOURCED` | `SN74HCT245N` VCC and DIR, its 100 nF, and the `/OE` pull-up | Any source. Not VSW. Not VBAT. Not Pico 3V3 |
| `J400_VSW_UNTRUSTED` | J400 pin 41 only | HCT, Pico VSYS, Pico VBUS |
| `J400_VBAT_NOT_LOGIC` | J400 pin 39 only | Everything else. Nominal 10 V gel cell, not a measured envelope |
| `PICO_VSYS_OPEN` | Pico VSYS only | USB VBUS, VSW, VBAT, HCT |
| `PICO_VBUS_USB_ONLY` | Pico VBUS only | Robot rails |

LVC parts stay on Pico 3V3 because that is their datasheet supply (1.65 V to 3.6 V). Three of them, outputs into Pico inputs, are tens of microamps against the Pico datasheet's recommendation to keep external 3V3 load under 300 mA. They are not a reason to add a regulator.

## User preference and the datasheet limit

The preference is to skip a separate 5 V regulator when it would only feed the HCT, and to run the low-voltage side from Pico 3V3. The LVC half of that preference is already the schematic, and it matches the LVC datasheet.

"Fairly hardened" does not move the HCT operating window. TI SN74HCT245 (SCLS020H):

- Recommended VCC is **4.5 V to 5.5 V** (nominal 5 V). Recommended input voltage is **0 to VCC**.
- Absolute maximum VCC is **−0.5 V to 7 V**. The same table says stresses beyond the recommended conditions are not functional operation. 7 V is a damage limit, not a supply to design to.
- The datasheet's "forgiving of 5 V rail accuracy" sentence is this 4.5–5.5 V window. It is not permission to run from an unmeasured robot rail, from the 10 V cell, or from 3.3 V.
- Absolute maximum current through VCC or GND is ±70 mA, and continuous output current is ±35 mA. A series resistor or other current limit does not hold VCC inside 4.5–5.5 V. At the HCT's microamp static current, a series resistor drops almost no voltage, so the pin still sees whatever VSW is.

Pico 3.3 V is a valid HCT **input** high (VIH minimum 2.0 V) only while HCT VCC stays in the 4.5–5.5 V window, so that 3.3 V is still below VCC. It is not an HCT supply. LVC VCC maximum is 3.6 V, so the HCT rail must not be tied back to the LVC rails.

## When VSW may feed the HCT, and when it must not

Robot VSW may be connected to `HCT_5V_UNSOURCED` only after a measurement shows VSW, including motor-start and motor-stop spikes, stays inside **4.5 V to 5.5 V**. If that log exists, no 5 V regulator is required for the buffer.

Until that log exists, the net stays unsourced. The sheet is not a ready-to-power build. The guide's "+5 V regulator" label and the schematic-scan "+5 V" note are not that log. The nominal 10 V gel cell is not a VSW measurement.

## Pico VSYS is still an open supply

The cartridge is not assumed to run from USB in the end. USB remains a bring-up source: the Pico datasheet feeds VBUS to VSYS through a Schottky, and VSYS is specified **1.8 V to 5.5 V**. A 10 V gel cell is outside that range and must not land on VSYS or VBUS.

Conditional choices, none of them fitted:

1. **VSW into VSYS**, only if the same spike log stays inside 1.8–5.5 V, and only onto VSYS. Do not tie VSW to VBUS. The onboard diode already ORs USB VBUS into VSYS; an external VSYS source must not be asked to backfeed the USB host.
2. **A separate external supply** already inside 1.8–5.5 V, again onto VSYS only.
3. **Conditioning from the gel cell** (a 7805 or a buck) only if VSW cannot meet the VSYS window and the cartridge has to run without USB. That part is not on the sheet. A 7805 still needs the cell to stay above its 7.5 V line-regulation input (TI LM340/LM7805, Rev. L, 25 °C, Iout ≤ 1 A) and inside the 35 V absolute maximum, and its heat is `(VBAT − 5) × Iout + VBAT × Iq` with Iq up to 8 mA. None of those pack numbers have been measured, so this option is not selected.

The HCT decision and the Pico decision are separate. A VSW log that passes 4.5–5.5 V can feed the HCT and can also feed VSYS. A log that fails either window does not get "fixed" with a current limiter.

## Questions that close the blocker

- VSW minimum, maximum, and the spikes while motors start and stop. The pass band for the HCT is 4.5–5.5 V. The pass band for Pico VSYS is 1.8–5.5 V.
- Whether the finished cartridge must run with USB unplugged. If yes, VSYS still needs one of the three sources above.
- Pack label and charger voltage, only if VSW fails and conditioning from the 10 V cell is actually wanted.
