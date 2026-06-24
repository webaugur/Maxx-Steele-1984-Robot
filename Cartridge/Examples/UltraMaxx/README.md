# UltraMaxx — community branding fork

Fork of the factory [`CBSDemo`](../CBSDemo/) cartridge with **UltraMaxx** community branding. The bootstrap stub, program table, phrase table, and music table are **byte-identical** to CBSDemo; only the 17-byte copyright field at `$A002` differs.

| Path | Description |
|------|-------------|
| [`PICOROM.md`](PICOROM.md) | **PicoROM P28** drop-in for U1 (based on CBSDemo hardware) |
| [`KiCAD/`](KiCAD/) | Hardware pointer → CBSDemo schematic |
| [`Firmware/Binary/UltraMaxx.532`](Firmware/Binary/UltraMaxx.532) | 4 KB EPROM image (`$A000`) |
| [`Firmware/Assembly/ultramaxx_ROM_532.dsm`](Firmware/Assembly/ultramaxx_ROM_532.dsm) | Listing derived from R. Wind `maxx_demo_ROM_532.dsm` |
| [`Firmware/Makefile`](Firmware/Makefile) | Validate + `picorom` upload targets |

Copyright string: `(c) UltraMaxx    ` (17 ASCII bytes, space-padded)

## Quick: PicoROM upload

```bash
python3 tools/picorom_cart.py upload --cart ultramaxx --device maxx_cart
# or: make -C Cartridge/Examples/UltraMaxx/Firmware upload
```

See [`PICOROM.md`](PICOROM.md) for socket prep, ROM size options, and [PicoROM](https://github.com/wickerwaka/PicoROM) install.

Use this image as a starting point when building custom community cartridges while keeping the standard bootstrap layout.