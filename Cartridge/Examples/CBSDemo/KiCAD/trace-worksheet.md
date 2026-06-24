# CBSDemo cartridge trace worksheet

Reverse-engineering notes from [`../../Photos/maxxcard.jpg`](../../Photos/maxxcard.jpg) (239×213 px, component side only).

## Photo confidence

| Level | Items |
|-------|-------|
| **Confirmed** | U1 silkscreen **27C512** (28-pin DIP, upper left); U3 sticker **5085** (24-pin DIP, center); U2 14-pin DIP (upper right, P/N illegible); ~5 orange passives; gold edge connector; two mounting holes |
| **Provisional** | Address/data/control net names; J1 finger-to-signal map; U2 part number (74HC14 hypothesis) |
| **TBD** | Bottom copper; via hops; full J1 pinout vs mainboard slot |

## Edge connector (J1)

**Mechanical reference:** [`../../Model3D/Maxx+Steele+Cartridge.stp`](../../Model3D/Maxx+Steele+Cartridge.stp) defines an **EDGE PCB Connector P2.54mm 44pin 2 Rows** — 44 contacts at 2.54 mm pitch (22 per PCB face). The photo shows only the **component-side** row; silkscreen numbers **1…22** are visible left-to-right on that face.

| Finger | Silk # (top face) | Net (provisional) | Confidence |
|--------|-------------------|-------------------|------------|
| 1–8 | 1–8 | D0–D7 | Low — bus width matches 6502 data |
| 9–16 | 9–16 | A0–A7 | Low |
| 17–20 | 17–20 | A12–A15 or control | TBD |
| 21–22 | 21–22 | VCC / GND | TBD — typical edge placement |
| 23–44 | (bottom face) | Mirror / extended bus | TBD — not visible in photo |

Mating envelope: **Maxx Chassis Cart Slot** in the same STEP assembly.

## 27C512 (U1) — JEDEC pin reference

| Pin | Name | Schematic net (provisional) |
|-----|------|----------------------------|
| 1 | VPP | +5V (read mode) |
| 2–5,21,23–26 | A12,A7–A4,A10,A11,A9,A8,A13 | Address bus |
| 3–10 | A7–A0 | A7–A0 |
| 11–13,15–19 | D0–D7 | Data bus |
| 14 | GND | GND |
| 20 | /CE | /CE (via U2/U3 glue) |
| 22 | /OE | /OE |
| 27 | /WE | +5V (read-only) |
| 28 | VCC | +5V |

## 5085 (U3)

Handwritten label on a 24-pin DIP. Hypotheses (unconfirmed):

- CBS / Mitsubishi custom mapper for `$A000` slot decode
- Address latch or bank register for 512 Kbit EPROM

All U3 pin nets are **TBD** until bottom-side photo or continuity test.

## Glue logic (U2)

14-pin package size matches **74HC14** (hex inverter) or similar 74xx. Used in schematic as `74HC14?` placeholder.

## Firmware cross-check

- Demo ROM image: 4 KB at **`$A000`** ([`PROGRAMMING.md`](../../PROGRAMMING.md))
- Alt burn target: [`Mitsubishi-KM2365`](../../../../DataSheets/Mitsubishi-KM2365.pdf) (4 KB)

## Regenerate schematic

```bash
python3 tools/gen_cartridge_sch.py
```