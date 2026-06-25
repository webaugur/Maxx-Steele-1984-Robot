# Chapter 2 — Opcode Vocabulary

This chapter lists every known Maxx Steele program opcode. Display names come from the internal ROM segment table at **`$F878`** (see [Appendix B](Appendices.md#b-display-segment-table)).

Opcodes are described in **hex order** within each group. For a one-page summary, see [Quick reference](Quick-Reference.md).

Export machine-readable opcode JSON ([`tools/maxx_rom.py`](../tools/maxx_rom.py); output path is arbitrary, e.g. under [`Cartridge/`](../Cartridge/)):

```bash
python3 tools/maxx_rom.py opcodes Cartridge/opcodes.json
```

---

## Motion and I/O opcodes (`$00`–`$10`)

| Opcode | Display | Name | Operand |
|--------|---------|------|---------|
| `$00` | L | Turn / drive left | Distance or angle |
| `$01` | F | Drive forward | Distance |
| `$02` | b | Drive reverse | Distance |
| `$03` | r | Turn / drive right | Angle |
| `$04` | Uu | Wrist up | Value |
| `$05` | Ud | Wrist down | Value |
| `$06` | Au | Arms up | Value |
| `$07` | Ad | Arms down | Value |
| `$08` | Cr | Claw rotate | Value |
| `$09` | Cc | Claw open/close | `$00`=open, `$01`=close |
| `$0A` | HL | Lamp | `$00`=off, `$01`=on |
| `$0B` | init | Home (all joints) | Must be `$00` |
| `$0C` | d | Delay | Seconds |
| `$0D` | Sn | Song number reference | Index |
| `$0E` | S | Speech (ROM phrase) | Phrase # |
| `$0F` | SS | Speech (shift mode) | Phrase # |
| `$10` | PS | Program speech capture | Slot # |

**Examples**

```
01 14    ; forward
0A 01    ; lamp on
0C 0A    ; delay 10 seconds
82 3F    ; speak ROM phrase $3F
```

---

## Extended opcodes (`$80`–`$FF`)

Opcodes with bit 7 set (`$80+`) are remapped through the display table for execution. The executor treats `$80` as an alias mapping to display index `$0C`.

| Opcode | Display | Name | Operand |
|--------|---------|------|---------|
| `$81` | PLAY | Play tune from music RAM | Tune # (`$0400` table) |
| `$82` | SPEE | Speak ROM phrase | Phrase # |
| `$83` | SS | Speak RAM phrase | Phrase # in `$0500` |
| `$84` | CLr | Clear speech phrase slot | Slot # |
| `$FE` | beg | Program begin marker | Display only |
| `$FF` | End | End of program | Must be `$FF` (pair `FF FF`) |

---

## Keypad-mapped opcodes

The internal ROM maps remote key scan codes to opcodes via table **`$E6B5`**. Faceplate names: [`Transmitter/remote-keypad.md`](../Transmitter/remote-keypad.md). Documented mapping (from [`maxx_internal_ROM.dsm`](../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm)):

```
Key opcodes:  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
              41 46 43 13 84 82 81 83 80
```

| Opcode | Typical keypad use |
|--------|-------------------|
| `$41` | Game-related step |
| `$43` | Click |
| `$46` | Motion step |
| `$13` | (mapped key — see learn/program mode) |
| `$80`–`$84` | Extended speech/music/clear group |

In **Program mode**, pressing a key stores the mapped opcode and prompts for an operand byte.

---

## Alphabetical index

| Name | Opcode |
|------|--------|
| arms_down | `$07` |
| arms_up | `$06` |
| claw_open_close | `$09` |
| claw_rotate | `$08` |
| delay | `$0C` |
| drive_forward | `$01` |
| drive_reverse | `$02` |
| end | `$FF` |
| home | `$0B` |
| lamp | `$0A` |
| play_tune | `$81` |
| song_number | `$0D` |
| speak_ram | `$83` |
| speak_rom | `$82` |
| speech_clear | `$84` |
| speech_program | `$10` |
| speech_rom | `$0E` |
| speech_shift | `$0F` |
| turn_left | `$00` |
| turn_right | `$03` |
| wrist_down | `$05` |
| wrist_up | `$04` |

---

## Unknown opcodes

If the executor encounters an undefined opcode, behavior is undefined in this documentation. Stick to opcodes listed above and validated with [`tools/maxx_rom.py`](../tools/maxx_rom.py).

---

**Previous:** [Chapter 1](01-Bytecode-Programming-Rules.md) · **Next:** [Chapter 3 — Motion and display](03-Programming-Motion-and-Display.md)