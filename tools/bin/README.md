# Maxx Steele command-line tools

Add this directory to your `PATH` once:

```bash
export PATH="/path/to/Maxx-Steele/tools/bin:$PATH"
```

Or from a clone of this repo:

```bash
export PATH="$(git -C /path/to/Maxx-Steele rev-parse --show-toplevel)/tools/bin:$PATH"
```

## Commands

| Command | Wraps | Examples |
|---------|-------|----------|
| `maxx` | `tools/maxx` | `maxx compile hello.bas`, `maxx upload hello.bas --device maxx_cart` |
| `maxxbas` | alias → `maxx` | `maxxbas check hello.bas` |
| `maxx-compile` | `maxx compile` | `maxx-compile hello.bas -o hello.532` |
| `maxx-rom` | `tools/maxx_rom.py` | `maxx-rom disasm UltraMaxx.532`, `maxx-rom validate hello.532` |
| `picorom-cart` | `tools/picorom_cart.py` | `picorom-cart upload --rom hello.bas --device maxx_cart` |

Requires `python3` on `PATH`. The `maxx` command builds the Rust release binary on first use.