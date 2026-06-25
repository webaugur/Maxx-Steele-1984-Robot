# maxxbas (Rust library + `maxx` CLI)

Rust implementation of the Maxx Steele toolchain. The **`maxx`** binary is the primary command-line entry point.

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
maxx simulate cart.532 --gui              # interactive step playback
maxx simulate cart.532 --image-out sim.bin # 64 KB image for masswerk virtual6502
maxx simulate hello.532 --cycles 30000     # more cycles to reach key-loop trap
```

`--gui` opens an egui window with step controls, program list, plan/front robot views, and opcode/action labels. The first build pulls in `eframe`/`wgpu` dependencies.

`maxx list --json` emits program-only JSON for tools that do not need firmware CPU state.

The same patched image can be loaded in [masswerk virtual6502](https://www.masswerk.at/6502/) for interactive single-step debugging.

## Library

```rust
use maxxbas::{compile, decode_cart, CartImage, Copyright};

let rom = compile(source, Copyright::UltraMaxx)?;
let trace = decode_cart(&CartImage::from_bytes(rom)?)?;
```

Python reference: [`../tinybasic_maxx.py`](../tinybasic_maxx.py)