# UltraMaxx — community branding fork

Fork of the factory [`CBSDemo`](../CBSDemo/) cartridge with **UltraMaxx** community branding. The bootstrap stub, program table, phrase table, and music table are **byte-identical** to CBSDemo; only the 17-byte copyright field at `$A002` differs.

| Path | Description |
|------|-------------|
| [`KiCAD/`](KiCAD/) | KiCad project (placeholder) |
| [`Firmware/Binary/UltraMaxx.532`](Firmware/Binary/UltraMaxx.532) | 4 KB EPROM image (`$A000`) |
| [`Firmware/Assembly/ultramaxx_ROM_532.dsm`](Firmware/Assembly/ultramaxx_ROM_532.dsm) | Listing derived from R. Wind `maxx_demo_ROM_532.dsm` |

Copyright string: `(c) UltraMaxx    ` (17 ASCII bytes, space-padded)

Use this image as a starting point when building custom community cartridges while keeping the standard bootstrap layout.