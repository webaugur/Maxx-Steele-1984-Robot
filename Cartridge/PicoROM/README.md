# PicoROM cartridge carrier (preliminary)

This directory is the active schematic for a Maxx Steele program cartridge whose ROM engine is an upstream [PicoROM P28](https://github.com/wickerwaka/PicoROM) module. The module is a custom RP2040 board in a DIP-28 outline. It is not a Raspberry Pi Pico, and this repository does not re-draw the P28 internals as a new board.

The KiCad photo-trace of the factory CBS cartridge stays where it is, as reference evidence:

- [`../Examples/CBSDemo/KiCAD/`](../Examples/CBSDemo/KiCAD/)
- [`../Examples/UltraMaxx/PICOROM.md`](../Examples/UltraMaxx/PICOROM.md) (older note about plugging a P28 into that factory board)

Nothing here is fabrication-ready. There is no cartridge PCB layout. Placeholder footprints exist only so the tscircuit project can build. `routingDisabled` is set. Do not order boards from this project.

Related open issues, not closed by this draft: [#51](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/51) (PicoROM cartridge), [#46](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/46) (J1 / U2 / U3 evidence), [#25](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/25) (PCB layout), [#26](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/26) (schematic evidence), [#27](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/27) (fab archive), [#50](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/50) (programming and bring-up), [#49](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/49) (mechanical fit), [#47](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/47) / [#48](https://github.com/webaugur/Maxx-Steele-1984-Robot/issues/48) (CAD and labels).

## What was verified

Upstream tree used for this pass:

- Repository: `wickerwaka/PicoROM`
- Commit: `dddab41f9a47c7666115837b50b93eb78c221e33` (2026-10-25, "Gerbers")
- Schematic: `hardware/PicoROM-28/PicoROM-28.kicad_sch` title **PicoROM-28 - ROM Emulator**, rev **1.10**, date **2026-10-24**
- Firmware pin map: `firmware/system.h` `BOARD_28P`
- Firmware targets in `firmware/CMakeLists.txt`: `P28_27C512_70ns` and `P28_27C512_100ns` (`BOARD_28P=1`). The 100 ns target also sets `FEATURE_STABLE_ADDRESS`. Version string in that file is **2.0**.
- Toolchain notes: [`INSTALL.md`](https://github.com/wickerwaka/PicoROM/blob/main/INSTALL.md). Host tool is `picorom` 2.x.

tscircuit workflow used here is the published CLI (`tsci check`, `tsci export`), matching [tscircuit/skill](https://github.com/tscircuit/skill) `SKILL.md` / `CLI.md`. Package pinned in `tscircuit/package.json` (`tscircuit@0.0.3108` at the time of this change). The `tsci` launcher is a Bun script.

## Supported hardware choice

**Use a finished PicoROM P28 module as the ROM, on a carrier that still has to be designed.** Do not build this cartridge by wiring a Raspberry Pi Pico to the robot bus, and do not copy the P28's RP2040, flash, regulators, and buffers onto a new PCB in this pass.

Reason, from upstream sources rather than from issue labels:

- P28 firmware requires a contiguous GPIO map: address GPIO0–GPIO15, data GPIO16–GPIO23, output-enables GPIO24–GPIO25, buffer OE GPIO26, buffer direction GPIO27, reset-enable GPIO28, LED GPIO29 (`firmware/system.h`, and the same nets on RP2040 pads in the rev 1.10 PCB).
- A Raspberry Pi Pico uses GPIO23 as the SMPS mode pin, GPIO24 as VBUS sense, GPIO25 as the LED, and GPIO29 as VSYS sense (`src/boards/include/boards/pico.h` in `raspberrypi/pico-sdk`). Those pins are not free headers. Stock `P28_27C512_*` firmware does not fit that module.
- The P28 schematic is a bare RP2040 plus W25Q16 flash, 3.3 V LDOs, and the bus buffers below. That is the integrated upstream design. A second integration would be a fork, not the supported ROM emulator.
- This pass does not switch the design to RP2350 or Pico 2.

The carrier schematic therefore shows one part, `U_P28`, with the DIP-28 pin functions taken from the P28 PCB, and the Maxx edge `J1` with 44 unresolved indexes. No wire joins them.

## Buffers

Upstream P28 already buffers the ROM socket. This schematic does **not** add another 7400-series driver.

| Upstream ref | What the rev 1.10 schematic/PCB actually says | Role seen on the PCB |
|---|---|---|
| U4, U5, U9 | Value `74LVC245`. Symbol library `74xx:74LS245` (same 245 pin names, not a claim that an LS part is fitted). Datasheet link `sn74lvc245a.pdf`. LCSC field `C294612`. VCC pad is `+3V3_SYS`. | Address and `/CE` `/OE` between RP2040 and the DIP pins. |
| U10 | Value text `74LVC8245`. LCSC field `C548858`, which is Nexperia **74LVC8T245BQ,118** in DHVQFN-24, matching the footprint. Symbol library and datasheet URL still say SN74AVC8T245. | Data transceiver. A-side pads `D0`–`D7` on `+3V3_SYS`. B-side pads `PIN_D0`–`PIN_D7` on `+5V_SYS`. DIR = `BUF_DIR`. `/OE` = `~{BUF_OE}`. |
| U7, U11 | `74AHCT1G126`, SOT-353, VCC `+5V_SYS`. | Reset pins. U7 input tied to `+3V3` (high). U11 input tied to GND (low). Both OE pins are `EXT_RESET_EN`. |

74HC, 74HCT, 74LVC, 74LS, and 74AHCT are not interchangeable. In particular:

- SN74LVC245A (the datasheet linked from U4/U5/U9) operates at 1.65 V to 3.6 V. Its inputs accept voltages to 5.5 V. Its outputs swing to its own VCC, which is 3.3 V on this board, not 5 V. ([TI SN74LVC245A](https://www.ti.com/product/SN74LVC245A))
- 74LVC8T245 is a dual-supply translator. Nexperia specifies both rails from 1.2 V to 5.5 V. DIR high sends A to B. `/OE` high puts both ports in high-Z. Either rail at 0 V forces the high-Z suspend state. ([LCSC C548858](https://www.lcsc.com/product-detail/C548858.html))
- 74AHCT1G126 is a 5 V TTL-threshold buffer. It is only the upstream reset path.
- A 3.3 V supply does not make a part 5 V tolerant. 74HC powered at 3.3 V is not specified for 5 V inputs.

`firmware/rom.cpp` drives `BUF_DIR` high and leaves it there (`gpio_put(BUF_DIR_PIN, true)`). With the 74LVC8T245 DIR polarity, that is A (RP2040) toward B (DIP data pins). The PIO program `or_output_enable` drives buffer `/OE` low only when both DIP `/CE` and `/OE` read low. Data-bus direction is not a second 245 that this carrier should duplicate.

On-hand 7400 line drivers are a **candidate only**. Full markings are not in the repo yet. No family is approved, and none is drawn here. Adding a driver in series with U4/U5/U9/U10 would be a second tri-state stage on the same ROM pins, with its own OE and power-up behavior, and it is not justified by the upstream circuit.

## Voltage

- RP2040 datasheet absolute maximum for a GPIO is IOVDD + 0.5 V (IOVDD itself max 3.63 V). Design to that. Do not connect robot bus signals to RP2040 or Pi Pico pins.
- Raspberry Pi forum comments describe a narrower, non-datasheet exception for some non-ADC GPIOs after IOVDD is up. That exception is not used here. GPIO26–GPIO29 are ADC pins; the P28 uses GPIO26–GPIO29 for buffer control, reset enable, and the LED, as outputs.
- P28 DIP pin 28 is PCB net `+5V_SYS`. Upstream README also calls pin 28 the 5 V VCC input. The factory cartridge photo shows a 27C512, which is a 5 V EPROM family. The Maxx edge voltage and the finger that would carry it have **not** been measured (#46). `P28_VCC_PIN28_OPEN` and `P28_GND_OPEN` are therefore left open in this schematic.

## Build and edit

From `Cartridge/PicoROM/tscircuit`, with [Bun](https://bun.sh) on `PATH` (the published `tsci` script starts with `#!/usr/bin/env bun`):

```bash
npm ci
export PATH="$HOME/.bun/bin:$PATH"
npm test
npm run check
npm run export
node --input-type=module -e 'import { readFileSync, writeFileSync } from "node:fs"; import { Resvg } from "@resvg/resvg-js"; const svg = readFileSync("../render/picorom-carrier-schematic.svg"); const png = new Resvg(svg, { fitTo: { mode: "width", value: 1600 }, background: "white" }).render(); writeFileSync("../render/picorom-carrier-schematic.png", png.asPng())'
```

`npm test` rebuilds the readable netlist and checks it against `../render/picorom-carrier.net.txt`. `npm run check` is:

```bash
tsci check source picorom-carrier.circuit.tsx
tsci check netlist picorom-carrier.circuit.tsx
tsci check pin_specification picorom-carrier.circuit.tsx
tsci check schematic-placement picorom-carrier.circuit.tsx
tsci check placement picorom-carrier.circuit.tsx
```

Edit pin tables in `tscircuit/src/p28-pinout.ts` and `tscircuit/src/j1-unresolved.ts`. Do not connect a `P28_*_OPEN` net to an `UNRESOLVED_J1_*` net unless a continuity log or both-sides photo in #46 identifies that finger. After a real connection is added, change the net name so the test stops calling it open.

Rendered review files:

- [`render/picorom-carrier-schematic.svg`](render/picorom-carrier-schematic.svg)
- [`render/picorom-carrier-schematic.png`](render/picorom-carrier-schematic.png)
- [`render/picorom-carrier.net.txt`](render/picorom-carrier.net.txt)

`tsci check schematic-placement` may still note that `U_P28`'s body is wider than its interior pin text. The width is intentional: at the default narrow body, the open-net labels collided with the pin names. The wide body keeps `P28_*_OPEN` outside the symbol.

## What a passing test does not prove

`tsci` checks and `npm test` prove that this source builds and that the open nets are disjoint. They do not prove the robot can read the cartridge, that `/CE` timing matches the 6502, that 5 V from the robot is safe or present, or that a P28 module fits the shell. Those remain #46, #49, and #50.
