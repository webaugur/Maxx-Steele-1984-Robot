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
| `simulate` | `.532` | Step preview (vector renderer TBD) |

## Simulator integration

`maxx list --json` emits a `ProgramTrace` with typed steps (`forward`, `delay`, `lamp`, …). A future robot simulator should consume this JSON to drive vector graphics.

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