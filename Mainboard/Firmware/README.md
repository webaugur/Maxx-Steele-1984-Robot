# Mainboard firmware (masked internal ROM)

8 KB factory ROM on the 6502 main logic board, mapped at **`$E000`–`$FFFF`**.

| Path | Description |
|------|-------------|
| [`Binary/Maxxrom.64`](Binary/Maxxrom.64) | ROM dump (8192 bytes; file offset `$0000` = CPU `$E000`) |
| [`Assembly/maxx_internal_ROM.dsm`](Assembly/maxx_internal_ROM.dsm) | Annotated 6502 disassembly (R. Wind, 2002–2006) |
| [`Assembly/maxx_internal_ROM.dsm.pdf`](Assembly/maxx_internal_ROM.dsm.pdf) | PDF of the listing |

Referenced by [`TechnicalManual/`](../TechnicalManual/), [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md), and the [`maxx simulate`](../../tools/maxxbas/) firmware model.