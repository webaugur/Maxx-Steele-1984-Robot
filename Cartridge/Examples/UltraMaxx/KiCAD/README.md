# UltraMaxx — KiCad

UltraMaxx uses the **same cartridge hardware** as the factory [`CBSDemo`](../../CBSDemo/) board. Only the ROM copyright string differs.

Hardware schematic, BOM, and trace notes:

**[`../../CBSDemo/KiCAD/CBSDemo.kicad_pro`](../../CBSDemo/KiCAD/CBSDemo.kicad_pro)**

## PicoROM adaptation (U1)

For development without EPROM burns, replace **U1** (27C512, DIP-28) with a [**PicoROM P28**](https://github.com/wickerwaka/PicoROM) in the same socket. Glue ICs U2/U3 and edge connector J1 are unchanged.

Full procedure: [`../PICOROM.md`](../PICOROM.md)

Firmware for this example: [`../Firmware/`](../Firmware/)