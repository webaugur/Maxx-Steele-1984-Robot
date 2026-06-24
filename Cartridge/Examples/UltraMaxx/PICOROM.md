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

## Develop / iterate workflow

```bash
# Edit program bytes, then re-validate
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
- `tools/picorom_cart.py`, `tools/maxx_rom.py`