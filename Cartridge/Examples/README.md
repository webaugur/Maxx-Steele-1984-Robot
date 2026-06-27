# Example cartridges

Reference 4 KB ROM images mapped at **`$A000`**. Each example includes a binary (`.532`) and an annotated assembly listing (`.dsm`).

| Example | Binary | Listing | KiCad | Description |
|---------|--------|---------|-------|-------------|
| [`CBSDemo/`](CBSDemo/) | [`CBSDemo.532`](CBSDemo/Firmware/Binary/CBSDemo.532) | [`maxx_demo_ROM_532.dsm`](CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm) | [`KiCAD/`](CBSDemo/KiCAD/) | Factory 1985 CBS Toys demo (38-step sequence) |
| [`UltraMaxx/`](UltraMaxx/) | [`UltraMaxx.532`](UltraMaxx/Firmware/Binary/UltraMaxx.532) | [`ultramaxx_ROM_532.dsm`](UltraMaxx/Firmware/Assembly/ultramaxx_ROM_532.dsm) | [`KiCAD/`](UltraMaxx/KiCAD/) | Community branding fork — same program as CBSDemo |
| [`MaxxOS/`](MaxxOS/) | [`MaxxOS.532`](MaxxOS/Firmware/Binary/MaxxOS.532) | [`maxxos_ROM_532.dsm`](MaxxOS/Firmware/Assembly/maxxos_ROM_532.dsm) | [`KiCAD/`](MaxxOS/KiCAD/) | Extended 6502 OS — math quiz, never `JMP $E0B6` |

Programming reference: [`../PROGRAMMING.md`](../PROGRAMMING.md). Tools: [`tools/maxx_rom.py`](../../tools/maxx_rom.py).

```bash
python3 tools/maxx_rom.py validate Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532
python3 tools/maxx_rom.py disasm Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532 \
  --compare-dsm Cartridge/Examples/CBSDemo/Firmware/Assembly/maxx_demo_ROM_532.dsm
```