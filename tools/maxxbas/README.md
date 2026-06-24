# maxxbas

Rust compiler for **MaxxBAS** — a line-oriented language that emits standard 4 KB Maxx Steele cartridge images (`.532`).

## Build

```bash
cargo build --release --manifest-path tools/maxxbas/Cargo.toml
```

Binary: `tools/maxxbas/target/release/maxxbas`

## Usage

```bash
maxxbas compile hello.bas -o hello.532
maxxbas check hello.bas
```

## Library

```rust
use maxxbas::{compile, Copyright};

let image = compile(source, Copyright::UltraMaxx)?;
```

See also: Python reference implementation at [`../tinybasic_maxx.py`](../tinybasic_maxx.py).