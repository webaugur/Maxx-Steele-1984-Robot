# MaxxOS cartridge

Extended-bootstrap example for the Maxx Steele 4 KB cartridge slot. MaxxOS **never returns to `$E0B6`**; it runs a cart-resident 6502 loop and calls internal mask-ROM drivers for the LED display, speech, and keypad.

## What it does

On boot the internal ROM still performs warm start and cartridge detection, then jumps into the MaxxOS stub at **`$A013`**. That stub:

1. Sets status bytes `$02` / `$03` (speech enabled)
2. Enables IRQ (`CLI`) and primes keypad ready (`$75` ← `$80`)
3. **`JMP $A080`** into the math quiz — no bytecode tables, no `JMP $E0B6`

### Math quiz (remote keypad)

- Random **addition** (A + B ≤ 9) or **subtraction** (A ≥ B, operands 1–9)
- LED shows operator, then operands, then `?` for your answer
- Enter a digit with orange **0–9** keys, **ENTER** to submit, **CLEAR** to re-enter
- Correct: score increments, *"Good play."* Wrong: **notE** on the display, retry

## Files

| Path | Role |
|------|------|
| [`Firmware/Binary/MaxxOS.532`](Firmware/Binary/MaxxOS.532) | 4 KB EPROM image @ `$A000` |
| [`Firmware/Assembly/maxxos_ROM_532.dsm`](Firmware/Assembly/maxxos_ROM_532.dsm) | Annotated listing |
| [`Firmware/build_maxxos.py`](Firmware/build_maxxos.py) | Python assembler / image generator |

## Build

```bash
python3 Cartridge/Examples/MaxxOS/Firmware/build_maxxos.py
python3 tools/maxx_rom.py validate Cartridge/Examples/MaxxOS/Firmware/Binary/MaxxOS.532
```

## Simulate

```bash
export PATH="$(git rev-parse --show-toplevel)/tools/bin:$PATH"
maxx simulate Cartridge/Examples/MaxxOS/Firmware/Binary/MaxxOS.532 --cycles 50000
maxx simulate Cartridge/Examples/MaxxOS/Firmware/Binary/MaxxOS.532 --gui
```

**Live GUI:** click orange digit keys to enter your answer, **ENTER** to submit, **CLEAR** to re-enter. The remote panel drives the real ROM keypad path (`$E60D`); RF is modeled as a wire to `$75`.

## Upload (PicoROM)

```bash
make -C Cartridge/Examples/MaxxOS/Firmware upload
```

Hardware reference: same cartridge PCB as [`UltraMaxx/KiCAD/`](../UltraMaxx/KiCAD/).

## Technical reference

- Boot model: [Technical Manual Ch 5 — Extending vs replacing the OS](../../Docs/Technical/05-Cartridge-Bootstrap-and-Internal-ROM.md#extending-vs-replacing-the-os)
- ROM routines used: `$E60D`, `$E3EC`, `$ED48`/`$ED4F`, `$F475`/`$F47E`, `$F684`, `$F8BE`