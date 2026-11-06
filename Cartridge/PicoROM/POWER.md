# Power on the PicoROM-28 carrier

The carrier connects J400 `VSW` to PicoROM-28 pin 28 and leaves `VBAT` open. There is no 7805 and no external buffer supply.

`VSW` is the assumed nominal 5 V rail. Confirm it stays inside 4.5–5.5 V, including transients, before power. That window is the 74LVC8T245 5 V JEDEC range (JESD36) and the range already accepted for this robot rail. It is not a measurement.

Inside the module (rev 1.10, not redrawn here):

| Onboard part | Connection |
|---|---|
| D5 `NSR0320MW2T1G` | Anode `+5V_SYS` (pin 28), cathode `+5V` |
| D4 `NSR0320MW2T1G` | Anode USB VBUS, cathode `+5V` |
| U8 `NCP114AMX330TCG` | 3.3 V LDO from the diode-OR node `+5V` |
| U1 `NCP114AMX330TCG` | 3.3 V LDO from `+5V_SYS` for the bus buffers |
| U10 VCC(B) | `+5V_SYS`, the data pins toward the robot |
| U10 VCC(A) | `+3V3_SYS`, the RP2040 side |

The two Schottkys are the module's USB and host OR. This carrier does not tie USB to `VSW`. With the robot off, USB does not feed pin 28. With USB unplugged, pin 28 still feeds the MCU rail through D5.

`VBAT` stays the unused nominal 10 V contact.
