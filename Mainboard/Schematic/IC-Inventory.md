# Mainboard and face IC inventory

Focused inventory of integrated circuits that participate in **memory-mapped I/O** and OS driver paths. Scope: main logic board + face/display/motor/speech assembly (~10–15 ICs), not the full chassis BOM.

**Confidence:** **confirmed** (datasheet + schematic refdes), **provisional** (project doc / archive naming), **TBD** (not yet traced on raster schematic).

Sources: [`Maxx_Steele_Schematic_enh-v1.1.png`](Maxx_Steele_Schematic_enh-v1.1.png), [`Maxx-Steele-CPU-Pinout.pdf`](../../DataSheets/Maxx-Steele-CPU-Pinout.pdf), [`DataSheets/README.md`](../../DataSheets/README.md).

---

## CPU and memory

| Refdes | Part | Board | Conf. | Role | MMIO / bus |
|--------|------|-------|-------|------|------------|
| — | **MOS 6502** | Mainboard | confirmed | System CPU | A0–A15, D0–D7, R/W, Φ2; see [CPU pinout](../../DataSheets/Maxx-Steele-CPU-Pinout.pdf) |
| U401 | **M6116** 2K×8 SRAM | Mainboard | confirmed | Program/music RAM backing | `$0000`–`$07FF` (with ROM at `$E000`) — [`Mitsubishi-M6116.pdf`](../../DataSheets/Mitsubishi-M6116.pdf) |
| — | **Masked 8 KB ROM** | Mainboard | confirmed | Internal OS | `$E000`–`$FFFF` — [`Maxxrom.64`](../Firmware/Binary/Maxxrom.64) |
| TBD | **74xx glue** (decode) | Mainboard | TBD | I/O chip selects `$1000`–`$1FFF` | See [MMIO-Pin-Map.md](MMIO-Pin-Map.md) decode table |
| TBD | **74xx / 244** buffers | Mainboard | provisional | Bus buffering | CPU pinout lists **F244 Hz Head** (pin 54) |

---

## Face — display, speech, lamps

| Refdes | Part | Board | Conf. | Role | MMIO / pins |
|--------|------|-------|-------|------|-------------|
| U500 | **COP41xL** | Face / main | provisional | LED shift register, motor drivers | `$1200` display; likely `$1600`/`$1C00` motor paths — [`National-COP41xL-Display-Motors.pdf`](../../DataSheets/National-COP41xL-Display-Motors.pdf) |
| TBD | **LC8100** or **ET9420** | Face | provisional | Speech synthesizer | `$1400` parallel nybbles; CPU pins **C0–C4**, **SPBusyB** (31), **SPStartB** (33) |
| TBD | Lamp drivers | Face / body | provisional | Lamp outputs | CPU pins **Lamp0–Lamp2**, **ArmLamp** (26–29) — opcode `$0A` |

---

## Motion and radio glue

| Refdes | Part | Board | Conf. | Role | MMIO / pins |
|--------|------|-------|-------|------|-------------|
| TBD | **COP420** family | Mainboard | provisional | Motion COP serial interface | **COPData** (37), **COPAckB** (50), **MoCOP Data Out** (51), **MoCOP Done** (52), **COPRWB** (53), **MoDataClk** (54), **Master CLK** (34) — [`National-COP420.pdf`](../../DataSheets/National-COP420.pdf) |
| — | RF receiver strip | Receiver | confirmed | 27 MHz OOK → robot | **RadioIn** CPU pin (25); keypad path to `$75` via `$E617` |
| — | **COP411L** U1 | Transmitter | confirmed | Remote keypad scan | Not on robot CPU board — [`National-COP411L.pdf`](../../DataSheets/National-COP411L.pdf) |

---

## Cartridge (expansion)

| Refdes | Part | Board | Conf. | Role | Map |
|--------|------|-------|-------|------|-----|
| U1 | **27C512** / KM2365 | Cartridge | confirmed | 4 KB program ROM | `$2000`–`$B000` slots — [`Mitsubishi-KM2365.pdf`](../../DataSheets/Mitsubishi-KM2365.pdf) |
| U2 | **74HC14?** | Cartridge | provisional | Glue / inversion | [`CBSDemo-cartridge-bom.csv`](../../Cartridge/Examples/CBSDemo/KiCAD/CBSDemo-cartridge-bom.csv) |
| U3 | **5085** (sticker) | Cartridge | TBD | Address mapper | 24-pin DIP; function not fully traced |

---

## Audio

| Refdes | Part | Board | Conf. | Role | Notes |
|--------|------|-------|-------|------|-------|
| TBD | Music / alarm driver | Head / main | provisional | Square-wave music | CPU pin **AudioOut Music** (10); IRQ path `$F1D8` writes `$1C00` |

---

## Next steps (hardware RE)

1. Read refdes silkscreen from [`Maxx_Steele_Schematic_enh-v1.1.png`](Maxx_Steele_Schematic_enh-v1.1.png) and assign **TBD** rows.
2. Confirm speech IC variant (LC8100 vs ET9420) from face PCB photo or die marking.
3. Trace **MoCOP Done** / **COPAckB** to ZP **`$2B`** talkback semantics.
4. Digitize decode logic into KiCad when [`Mainboard/KiCAD/`](../KiCAD/) matures.

---

See also: [MMIO-Pin-Map.md](MMIO-Pin-Map.md), [MMIO-ROM-Crossref.md](MMIO-ROM-Crossref.md) (auto-generated from ROM listing).