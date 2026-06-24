# Chapter 4 — Programming Speech and Music

This chapter describes how the Maxx Steele robot produces speech and music under program control.

Speech uses a parallel phoneme interface at **`$1400`**. The speech driver subroutine **`$F3D5`** clocks 4-bit nybbles to the synthesizer. Music uses byte pairs in RAM at **`$0400`** and an IRQ handler near **`$F1D8`**.

---

## Speech overview

| Mechanism | Opcode | Data source |
|-----------|--------|-------------|
| ROM phrase table | `$82`, `$0E` | Internal ROM phoneme strings |
| RAM phrase table | `$83` | `$0500` (copied from cartridge or recorded) |
| Program speech capture | `$10` | User-recorded slots |
| Clear phrase slot | `$84` | Clears RAM slot |

---

## ROM phrases (`$82` / `$0E`)

Phrases are indexed phoneme strings in internal ROM. The driver at **`$F3D5`** outputs sounds with index **X** (see `JSR $F3D5` in [`maxx_internal_ROM.dsm`](../Chassis/Firmware/Assembly/maxx_internal_ROM.dsm)).

Phoneme code tables reside at **`$F567`** / **`$F4DB`** (140 entries). Until the full **`$F4DB`** table is transcribed, treat phrase indices as opaque IDs.

Demo examples:

```
82 3F    ; "Ha ha ha ha ha" (ROM)
83 16    ; uses ROM index via RAM phrase indirection in demo — see cart listing
```

---

## RAM phrases (`$83`)

Cartridges embed phrase data at offset **`$81`** (address **`$A081`** for demo cart). The bootstrap copies bytes to **`$0500`**.

Each phrase is a sequence of **16-bit phoneme tokens**; **`$FF`** pads unused slots.

Demo cart phrase table (from [`maxx_demo_ROM_532.dsm`](../Cartridge/Firmware/Assembly/maxx_demo_ROM_532.dsm)):

| Slot | Content (abbrev.) |
|------|-------------------|
| 0 | "Hello, I am Maxx Steele" / greeting tokens |
| 1 | "I am ready when you are" |
| 2 | "I am a great match for humans" |
| 3 | "Goodbye for now, have a good day" |

Example program bytes:

```
83 10    ; speak RAM phrase 0 (demo uses operand $10 for table indexing — verify in .dsm)
0C 02    ; delay
83 00    ; another RAM phrase
```

**Note:** Demo listing uses operands `$10`, `$00`, `$01`, etc. — cross-check against your cart image with `maxx_rom.py disasm`.

---

## Music overview

Opcode **`$81`** plays a tune from the music RAM table at **`$0400`**.

Tune selection calls **`$EF01`** to resolve pointers for tune number in the operand (multiple call sites in internal ROM).

### Music byte pairs

Stored in `$0400` (and cartridge offset **`$BB`** / `$A0BB`):

```
70 12    ; note
70 11    ; note
38 0F
54 0D
E0 0F
00 00    ; end of tune
```

The music IRQ handler at **`$F1D8`** reads duration data from **`$F15B`** and frequency/note data from **`$0400`**.

Demo: `81 06` plays tune #6 (Reveille); `81 00` plays tune #0 at end of demo sequence.

---

## Song number opcode `$0D`

References a song index for the internal song table (distinct from `$81` play-tune in some keypad contexts). Used in learn/program flows.

---

## Advanced notes

- **Phoneme authoring** requires transcribing `$F4DB` / `$F567` — listed as future work in [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) §10.
- **Speech error flag** (**SEr** in `$02`) — set when speech subsystem reports fault; demo cart entry sets related status.
- Face/speech hardware datasheets: [`DataSheets/`](../DataSheets/) (Sanyo LC3100/LC8100, ET9420 family).

---

**Previous:** [Chapter 3](03-Programming-Motion-and-Display.md) · **Next:** [Chapter 5 — Cartridge and internal ROM](05-Cartridge-Bootstrap-and-Internal-ROM.md)