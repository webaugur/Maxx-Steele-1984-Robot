# CBSDemo — factory demo cartridge

Original **1985 CBS Toys** demonstration program shipped with the Maxx Steele robot.

| Path | Description |
|------|-------------|
| [`KiCAD/`](KiCAD/) | KiCad project (placeholder) |
| [`Firmware/Binary/CBSDemo.532`](Firmware/Binary/CBSDemo.532) | 4 KB EPROM image (`$A000`) |
| [`Firmware/Basic/cbsdemo.bas`](Firmware/Basic/cbsdemo.bas) | MaxxBAS source (translated from factory ROM) |
| [`Firmware/Assembly/maxx_demo_ROM_532.dsm`](Firmware/Assembly/maxx_demo_ROM_532.dsm) | Annotated disassembly (R. Wind, 2002–2006) |
| [`Firmware/Assembly/maxx_demo_ROM_532.dsm.pdf`](Firmware/Assembly/maxx_demo_ROM_532.dsm.pdf) | PDF of the R. Wind listing |

Copyright string: `(c) 1985 CBS Toys`

The cartridge runs a **38-step** bytecode sequence: speech, delays, motion, lamp, tunes, and home. See [`../../PROGRAMMING.md`](../../PROGRAMMING.md) §5 for the full step list.

Compile the MaxxBAS translation (byte-identical when using factory phrase/music tables):

```bash
export PATH="$(git rev-parse --show-toplevel)/tools/bin:$PATH"
maxx compile Firmware/Basic/cbsdemo.bas -o Firmware/Binary/CBSDemo.bas.532 \
  --copyright cbs --tables-from Firmware/Binary/CBSDemo.532
```

Community branding fork: [`../UltraMaxx/`](../UltraMaxx/).