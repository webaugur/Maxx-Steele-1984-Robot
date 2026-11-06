# Cartridge power

The sheet assumes J400 `VSW` is nominal 5 V. That is a design assumption, not a measurement. Before applying power, confirm VSW stays inside **4.5–5.5 V** for the HCT and that the voltage at Pico VSYS stays at or below **5.5 V**, including transients. Pico VSYS minimum is 1.8 V. No 7805 is fitted. VBAT is not connected.

| Net | Connection |
|---|---|
| `VSW_ASSUMED_5V` | J400 VSW, HCT VCC, HCT DIR, HCT 100 nF, HCT `/OE` pull-up, anode of `D_VSYS` |
| `PICO_VSYS` | Cathode of `D_VSYS` (1N5817) and Pico VSYS |
| `PICO_3V3` | Pico 3V3 pin and the three SN74LVC245AN supplies |
| `PICO_VBUS_USB_ONLY` | Pico VBUS only |
| `J400_VBAT_NOT_LOGIC` | J400 VBAT only |

LVC buffers stay on Pico 3V3 (datasheet VCC 1.65–3.6 V). HCT stays on the assumed 5 V rail (recommended VCC 4.5–5.5 V).

USB and the robot supply are diode-ORed the way the Pico datasheet shows in Figure 16 (section 4.5). The Pico already has Schottky D1 from VBUS to VSYS. `D_VSYS` is a second Schottky from VSW to VSYS, anode toward the robot. Each source can power the Pico, and neither feeds the other. Do not short VSYS to VBUS; the datasheet allows that only when USB is the only source, and it would defeat the isolation.
