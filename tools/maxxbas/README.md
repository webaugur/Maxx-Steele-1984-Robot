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

## Simulator

[`Simulator/`](../../Simulator/) — Joe's virtual6502 bypass patches, trap table, and docs.

`maxx simulate` runs all three layers in one command:

1. **Program** — decode cart bytecode (`ProgramTrace`)
2. **Robot** — kinematic preview with **ASCII visual storyboard** per opcode (display label, action glyph, pose)
3. **Firmware** — patched 65C02 internal ROM boot (`mos6502` crate)

```bash
maxx simulate program.bas --json
maxx simulate cart.532 --image-out maxx_sim.bin   # masswerk virtual6502 image
maxx simulate cart.532 --no-firmware              # robot model only
maxx simulate cart.532 --gui                      # interactive robot status window
```

`--gui` opens an egui window with step controls, program list, plan/front robot views, and opcode/action labels. The first build pulls in `eframe`/`wgpu` dependencies.

`maxx list --json` still emits program-only JSON for tools that do not need firmware CPU state.

```bash
maxx list program.532 --json > program.trace.json
```

## Library

```rust
use maxxbas::{compile, decode_cart, CartImage, Copyright};

let rom = compile(source, Copyright::UltraMaxx)?;
let trace = decode_cart(&CartImage::from_bytes(rom)?)?;
```

Python reference: [`../tinybasic_maxx.py`](../tinybasic_maxx.py)