# maxxbas (Rust library + `maxx` CLI)

Rust implementation of the Maxx Steele toolchain. The **`maxx`** binary is the primary command-line entry point.

SAM speech, boops, and mouth helpers live in the shared crate **[`sam-say`](../sam-say/)** (`path = "../sam-say"`). Other projects can depend on that crate without pulling in the full Maxx simulator.

## Quick start

```bash
export PATH="$(git rev-parse --show-toplevel)/tools/bin:$PATH"

maxx compile Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas
maxx list Cartridge/Examples/UltraMaxx/Firmware/Binary/hello.532 --json
maxx upload Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas --device maxx_cart --dry-run
```

Or build directly:

```bash
cargo build --release --manifest-path tools/maxxbas/Cargo.toml
tools/maxxbas/target/release/maxx --help
```

## Commands

| Command | Input | Purpose |
|---------|-------|---------|
| `compile` | `.bas` / `.maxx` | Emit 4096-byte `.532` cart image |
| `check` | `.bas` / `.maxx` | Parse only |
| `validate` | `.532` | Structure checks |
| `list` | `.532` | Human listing; `--json` for simulators |
| `upload` | `.bas` / `.maxx` / `.532` | Compile if needed → PicoROM |
| `simulate` | `.bas` / `.532` | Unified simulator (program + robot + firmware) |

## Simulator (`maxx simulate`)

Firmware bypass patches and trap table: [`patches.json`](patches.json) (from Joe / Andy-in-Indy `MaxxSteeleFirmwarePatchesForVirtual6502Simulator.xlsx`, 2022).

Trap addresses map to routines documented in [Technical Manual Ch 5](../../Docs/Technical/05-Cartridge-Bootstrap-and-Internal-ROM.md) (`$E161` learn loop, `$E17B` immediate decode, `$E617` keypad poll, `$EE2F` opcode dispatch). ROM listing: [`Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm`](../../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm).

`maxx simulate` runs all three layers in one command:

| Layer | Description |
|-------|-------------|
| **Program** | Decodes the cart bytecode table (`ProgramTrace`) |
| **Robot** | Kinematic preview with ASCII visual storyboard per opcode |
| **Firmware** | Patched internal ROM in an embedded 65C02 (`mos6502` crate) |

```bash
maxx simulate program.bas
maxx simulate cart.532 --json              # program + robot + firmware report
maxx simulate cart.532 --plain             # text only (no ASCII art)
maxx simulate cart.532 --no-firmware       # robot model only
maxx simulate cart.532 --gui              # live 65C02 + remote keypad GUI
maxx simulate cart.532 --gui --no-firmware  # step-playback only (no CPU)
maxx simulate cart.532 --image-out sim.bin # 64 KB image for masswerk virtual6502
maxx simulate hello.532 --cycles 30000     # more cycles to reach key-loop trap
```

`--gui` (default, with firmware) opens a **live simulator**: patched internal ROM runs in an embedded 65C02, with an on-screen **remote transmitter** (layout from `Transmitter/Photos/Product/Remote-Front.svg`). Key presses are modeled as a direct wire to zero-page `$75`; the ROM keypad path (`$E617` / `$E6A4`) latches into `$15` as on hardware. LED writes to `$1200` update the robot face display.

The toolbar **TX** button (off by default) also sends that key as 27.095 MHz OOK through `hackrf_transfer`, at about 645 baud, while the on-screen robot keeps running. TX VGA starts at 0 dB and the RF amp stays off. If no HackRF is plugged in, the simulator prints `HackRF not found` on stderr and keeps running. The picture is what the robot should do; a mismatch is a hardware fault. Details: [`Transmitter/transmitter-architecture.md`](../../Transmitter/transmitter-architecture.md#envelope-baud-rate).

A USB copy lives in [`portable/MaxxSim`](../../portable/MaxxSim/README.txt). The Start scripts open the GUI. Window state is stored in `config/` next to those scripts, via `current_exe()`, not in the host profile. `sh portable/build_linux.sh` fills in `maxx/linux/maxx` and `hackrf/linux/`. Double-click `Start-Linux.sh` (or the Windows or Mac script) to run `simulate --gui`. The GitHub Release is one `MaxxSim-<version>.zip` with those folders. The simulator looks for HackRF tools in `hackrf/<os>/` before `PATH`.

`--gui --no-firmware` keeps the older step-playback window (program list + kinematic preview, no live CPU).

The first GUI build pulls in `eframe`/`wgpu` dependencies.

```bash
maxx simulate Cartridge/Examples/MaxxOS/Firmware/Binary/MaxxOS.532 --gui
```

`maxx list --json` emits program-only JSON for tools that do not need firmware CPU state.

The same patched image can be loaded in [masswerk virtual6502](https://www.masswerk.at/6502/) for interactive single-step debugging.

## Library

```rust
use maxxbas::{compile, decode_cart, CartImage, Copyright};

let rom = compile(source, Copyright::UltraMaxx)?;
let trace = decode_cart(&CartImage::from_bytes(rom)?)?;
```

Python reference: [`../tinybasic_maxx.py`](../tinybasic_maxx.py)