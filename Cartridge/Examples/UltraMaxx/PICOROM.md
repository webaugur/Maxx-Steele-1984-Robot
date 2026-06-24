# UltraMaxx cartridge on PicoROM

Use a [PicoROM P28](https://github.com/wickerwaka/PicoROM) (28-pin DIP ROM emulator) instead of burning U1 on the **CBSDemo-class** cartridge PCB. UltraMaxx shares the same board layout and glue logic as [`../CBSDemo/`](../CBSDemo/); only the 4 KB ROM image (copyright string) differs.

## Hardware (from CBSDemo rev 0.1)

| Ref | Factory part | PicoROM adaptation |
|-----|--------------|------------------|
| **U1** | 27C512 (28-pin DIP) | **PicoROM P28** in the U1 socket |
| U2 | 74HC14? (glue) | Unchanged |
| U3 | 5085-TBD (mapper) | Unchanged |
| J1 | 44-pos card edge | Unchanged |

PicoROM P28 is a JEDEC 28-pin DIP emulator. It fits the DIP-28 footprint documented in [`../../CBSDemo/KiCAD/`](../CBSDemo/KiCAD/) and the EPROM envelope in [`../../Model3D/`](../../Model3D/). Use the **P28** firmware image when flashing PicoROM (not P32/POG).

Upstream KiCad footprints: [`wickerwaka/PicoROM/hardware/PicoROM.pretty`](https://github.com/wickerwaka/PicoROM/tree/main/hardware/PicoROM.pretty)

## ROM image

| File | CPU map | Notes |
|------|---------|-------|
| [`Firmware/Binary/UltraMaxx.532`](Firmware/Binary/UltraMaxx.532) | `$A000` | 4096-byte image; copyright `(c) UltraMaxx    ` |

Validate before upload:

```bash
python3 tools/maxx_rom.py validate Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532
```

## Install PicoROM host tool (2.x)

Download `picorom` for your platform from [PicoROM releases](https://github.com/wickerwaka/PicoROM/releases) and place it on your `PATH`. Version 2.x is required (USB protocol changed from 1.x).

Linux udev (non-root USB access): see [INSTALL.md](https://github.com/wickerwaka/PicoROM/blob/main/INSTALL.md).

## Socket and bring-up

1. Power off the robot; remove the cartridge.
2. Desolder or socket U1 (27C512). A **low-profile DIP socket** helps when stacking PicoROM above neighboring parts.
3. Insert **PicoROM P28** oriented per silkscreen pin 1 (same as EPROM).
4. Connect USB-C to PicoROM for development uploads, or rely on standalone flash after `picorom upload -s`.

**Power:** PicoROM can run from cartridge **VCC (pin 28)** alone after the image is committed to flash. USB is only required for upload and firmware updates.

## Upload UltraMaxx image

Rename your device once (optional):

```bash
picorom list
picorom rename E66138528361BB3 maxx_cart   # use ID from list
```

Show image info and example commands:

```bash
python3 tools/picorom_cart.py info --cart ultramaxx
```

Upload (validates structure, then calls `picorom`):

```bash
python3 tools/picorom_cart.py upload --cart ultramaxx --device maxx_cart --size 4kb
```

Persist across power cycles:

```bash
python3 tools/picorom_cart.py upload --cart ultramaxx --device maxx_cart --size 4kb --persist
```

### ROM size selection

| `--size` | PicoROM token | When to use |
|----------|---------------|-------------|
| `4kb` (default) | `32KBit` | Rebuild using KM2365-class 4 KB EPROM footprint or when only the `$A000` window is decoded |
| `27c512` | `512KBit` | Direct replacement in factory 27C512 socket (image at offset 0; upper addresses unused) |

If the cart is not detected after upload, try the other size token. Glue IC **U3** may decode only a 4 KB window regardless of U1 capacity.

## MaxxBAS — compile BASIC-like programs

[MaxxBAS](https://github.com/webaugur/Maxx-Steele-1984-Robot) is a line-oriented language that compiles to the same bytecode layout as the factory demo. No firmware fork: compile on the host, upload the resulting `.532` with stock PicoROM.

| Statement | Bytecode | Notes |
|-----------|----------|-------|
| `DELAY n` | `0C nn` | Seconds (0–255) |
| `FORWARD n` / `BACK n` / `LEFT n` / `RIGHT n` | motion opcodes | Distance or angle |
| `LAMP ON` / `LAMP OFF` | `0A 01` / `0A 00` | Head lamp |
| `HOME` | `0B 00` | All joints home |
| `PLAY n` | `81 nn` | Tune index in music table |
| `SPEAK n` | `82 nn` | Built-in ROM phrase |
| `SAY n` | `83 nn` | RAM phrase slot (needs phrase table bytes) |
| `END` | `FF FF` | Required terminator |

v1 has **no** variables, `GOTO`, or `IF`. Use `SPEAK` for factory speech; custom `SAY` text requires phoneme authoring (future work).

Sample source: [`Firmware/Basic/hello.bas`](Firmware/Basic/hello.bas)

```bash
# Compile + validate (Python)
python3 tools/tinybasic_maxx.py compile Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas \
  -o Cartridge/Examples/UltraMaxx/Firmware/Binary/hello.532

# Or Rust crate (Phase 2 — PicoROM-aligned host tool)
cargo build --release --manifest-path tools/maxxbas/Cargo.toml
tools/maxxbas/target/release/maxxbas compile Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas \
  -o Cartridge/Examples/UltraMaxx/Firmware/Binary/hello.532

# Makefile shortcuts
make -C Cartridge/Examples/UltraMaxx/Firmware compile        # Python
make -C Cartridge/Examples/UltraMaxx/Firmware compile-rust   # Rust
make -C Cartridge/Examples/UltraMaxx/Firmware test-rust

# Upload compiled image (not the stock UltraMaxx binary)
python3 tools/picorom_cart.py upload --device maxx_cart \
  --rom Cartridge/Examples/UltraMaxx/Firmware/Binary/hello.532

# Tests
python3 tools/test_tinybasic_maxx.py
```

Rust library API: [`tools/maxxbas/`](../../../tools/maxxbas/). Intended for future `picorom upload program.maxx` integration.

## Develop / iterate workflow

```bash
# Edit MaxxBAS source or raw program bytes, then re-validate
make -C Cartridge/Examples/UltraMaxx/Firmware compile
python3 tools/maxx_rom.py validate Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532

# Push to socketed PicoROM (RAM — fast iteration)
python3 tools/picorom_cart.py upload --cart ultramaxx --device maxx_cart

# Disassemble against R. Wind listing
python3 tools/maxx_rom.py disasm Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532 \
  --compare-dsm Cartridge/Examples/UltraMaxx/Firmware/Assembly/ultramaxx_ROM_532.dsm
```

Fork from CBSDemo firmware: copy [`../CBSDemo/Firmware/Binary/CBSDemo.532`](../CBSDemo/Firmware/Binary/CBSDemo.532), patch bytes 2–18 with the UltraMaxx copyright, or use `tools/maxx_rom.py template` and customize tables.

## CBSDemo image

The factory demo uploads the same way:

```bash
python3 tools/picorom_cart.py upload --cart cbsdemo --device maxx_cart --size 4kb
```

## References

- PicoROM project: https://github.com/wickerwaka/PicoROM
- CBSDemo schematic: [`../CBSDemo/KiCAD/CBSDemo.kicad_pro`](../CBSDemo/KiCAD/CBSDemo.kicad_pro)
- Cartridge programming: [`../../PROGRAMMING.md`](../../PROGRAMMING.md)
- `tools/maxxbas/` (Rust), `tools/tinybasic_maxx.py` (Python), `tools/picorom_cart.py`, `tools/maxx_rom.py`