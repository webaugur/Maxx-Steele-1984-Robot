# Maxx Steele Simulator

Unified robot simulation for cartridge programs and internal ROM firmware debugging.

## Tool

Everything runs through the main toolchain:

```bash
export PATH="$(git rev-parse --show-toplevel)/tools/bin:$PATH"

# Full simulation: ASCII visual storyboard + robot model + patched 65C02 firmware
maxx simulate program.bas
maxx simulate UltraMaxx.532 --json   # JSON includes per-step `visual` frames

# Text-only (no ASCII art)
maxx simulate hello.532 --plain

# Export a 64 KB image for masswerk virtual6502 (optional)
maxx simulate UltraMaxx.532 --image-out maxx_sim.bin

# Program-only (skip CPU boot)
maxx simulate hello.532 --no-firmware

# More cycles to reach the key-loop trap (~24k on hello.532)
maxx simulate hello.532 --cycles 30000

# Interactive GUI (step playback, robot pose, opcode banner)
maxx simulate hello.532 --gui
maxx simulate hello.532 --gui --no-firmware   # robot model only
```

## What it simulates

| Layer | Description |
|-------|-------------|
| **Program** | Decodes the cart bytecode table (`ProgramTrace`) |
| **Robot** | Steps through motion, lamp, delay, speech, and music with a simple kinematic model |
| **Firmware** | Runs the patched internal ROM in an embedded 65C02 (Joe's virtual6502 bypass patches) |

Patches are defined in [`patches.json`](patches.json), derived from
`MaxxSteeleFirmwarePatchesForVirtual6502Simulator.xlsx` (Andy-in-Indy / Joe, 2022).

## JSON report

`maxx simulate --json` emits one document with `program`, `robot`, and `firmware` sections.
Use it for tests, CI, and a future vector-graphics front end.

## External reference

The same patched image can be loaded manually in [masswerk virtual6502](https://www.masswerk.at/6502/)
for interactive single-step debugging. Trap addresses are listed in `patches.json`.