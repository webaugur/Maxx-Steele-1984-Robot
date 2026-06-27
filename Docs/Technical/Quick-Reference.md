# Maxx Steele Quick Reference Card

Single-page summary. Full detail in chapters 1–7 and [Appendices](Appendices.md).

---

## Modes (`$0D`)

| Val | Mode |
|-----|------|
| 0 | Immediate |
| 1 | Learn |
| 2 | Program |
| 3 | Execute |

---

## Key zero page

| Addr | Use |
|------|-----|
| `$02` | Status A |
| `$03` | Status B |
| `$0D` | Mode |
| `$0F`/`$10` | Program step pointer |
| `$0200` | Program RAM base |
| `$0400` | Music RAM |
| `$0500` | Speech RAM |

---

## Motion opcodes

| Hex | Disp | Action |
|-----|------|--------|
| 00 | L | Left |
| 01 | F | Forward |
| 02 | b | Reverse |
| 03 | r | Right |
| 04 | Uu | Wrist up |
| 05 | Ud | Wrist down |
| 06 | Au | Arms up |
| 07 | Ad | Arms down |
| 08 | Cr | Claw rotate |
| 09 | Cc | Claw open/close |
| 0A | HL | Lamp |
| 0B | init | Home (`00`) |
| 0C | d | Delay (sec) |

---

## Speech / music

| Hex | Disp | Action |
|-----|------|--------|
| 0E | S | Speech ROM |
| 0F | SS | Speech shift |
| 10 | PS | Program speech |
| 81 | PLAY | Play tune |
| 82 | SPEE | Speak ROM |
| 83 | SS | Speak RAM |
| 84 | CLr | Clear phrase |
| FE | beg | Marker |
| FF | End | End (`FF FF`) |

---

## Cartridge header (`$A000`)

| Off | Content |
|-----|---------|
| +0 | Entry vector |
| +2 | `(c) 1985 CBS Toys` |
| +$13 | Bootstrap |
| +$35 | Program |
| +$81 | Phrases |
| +$BB | Music |

---

## Common sequences

```
0C nn    delay nn seconds
0B 00    home
0A 01    lamp on
0A 00    lamp off
FF FF    end program
```

---

## UltraMaxx BASIC (Ch 7)

| Statement | Example |
|-----------|---------|
| Delay | `DELAY 2` |
| Drive | `FORWARD 20` `BACK 20` `LEFT 5` `RIGHT 6` |
| Joints | `ARMS UP 40` `WRIST DOWN 35` `CLAW ROTATE 21` `CLAW OPEN` |
| Lamp | `LAMP ON` / `LAMP OFF` |
| Home | `HOME` |
| Music | `PLAY 6` |
| Speech | `SPEAK 63` (ROM) · `SAY 0` (RAM slot) |
| End | `END` |

**Cart limit:** 38 steps max. **Not supported:** variables, `IF`, `GOTO`.

CLI ([`tools/maxx`](../../tools/maxx)):

```bash
maxx compile FILE.bas -o FILE.532
maxx upload FILE.bas --device maxx_cart
```

---

## Tools

Binaries: [`tools/maxx`](../../tools/maxx), [`tools/maxx_rom.py`](../../tools/maxx_rom.py) (shell wrapper [`tools/bin/maxx-rom`](../../tools/bin/maxx-rom)).

```bash
maxx compile FILE.bas -o FILE.532
maxx-rom validate FILE.532
maxx-rom template FILE.532
```