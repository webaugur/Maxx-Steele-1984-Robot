# Maxx Steele remote keypad

Faceplate labels from [`Photos/Product/Remote-Front.jpg`](Photos/Product/Remote-Front.jpg), cross-checked against [`Stickers/Remote-Sticker-Sheet.svg`](Stickers/Remote-Sticker-Sheet.svg) and the matrix map in [`Photos/ReverseEngineering/keyboard-matrix-reference-1.png`](Photos/ReverseEngineering/keyboard-matrix-reference-1.png).

Diagram: [`Photos/Product/Remote-Front.svg`](Photos/Product/Remote-Front.svg).

---

## Matrix key map (A–Y)

Each matrix position has an internal scan label (**A–Y**). Orange digits **0–9** and letters **A–B** on the faceplate double as music-entry keys on several buttons.

| Key | Faceplate | Orange key | Notes |
|-----|-----------|------------|-------|
| **A** | **DRIVE** | **U** | Left-turn / U-turn drive icon on faceplate; sticker sheet uses **0** + note **C** |
| **B** | **DRIVE** | **1** | Forward drive; note **C#** |
| **C** | **DRIVE** | **2** | Reverse drive; note **D** |
| **D** | **DRIVE** | **3** | Right drive; note **D#** |
| **E** | **WRIST** | **4** | Wrist joint; note **E** |
| **F** | **WRIST** | **5** | Wrist joint; note **F** |
| **G** | **ARMS** | **6** | Arms joint; note **F#** |
| **H** | **ARMS** | **7** | Arms joint; note **G** |
| **I** | **CLAW** | **8** | Claw joint; note **G#** |
| **J** | **CLAW** | **9** | Claw joint; note **A** |
| **K** | **LAMP** | **A** | Head lamp on/off; note **A#** |
| **L** | **HOME** | **B** | All joints home; orange/yellow highlight; note **B** |
| **M** | **NOTE REST** | — | Small **WAIT** label above key |
| **N** | **SHIFT OCTAVE** | — | Blue indicator |
| **O** | **CLEAR** | — | Orange indicator |
| **P** | **ENTER** | — | Yellow indicator |
| **Q** | **SONG** / **NOTES** | — | **SONG** above, **NOTES** below (or **OR** on sticker art) |
| **R** | **CLOCK** / **STATUS** | — | **CLOCK** above, **STATUS:** below |
| **S** | **SPEECH** | — | Blue indicator |
| **T** | **MOTION** | — | Blue indicator |
| **U** | **GAME** | — | Blue indicator; sets mode `$0D` = 4 (game) |
| **V** | **PROGRAM** | — | Blue indicator; sets mode `$0D` = 2 |
| **W** | **LEARN** | — | Blue indicator; sets mode `$0D` = 1 |
| **X** | **EXECUTE** | — | Blue indicator; runs stored program |
| **Y** | **POWER/STOP** | — | Wide bottom key; matrix **PK** column, **Gnd** row |

---

## Faceplate layout (top to bottom)

```
┌────────┬────────┬────────┬────────┐
│ U      │ 1      │ 2      │ 3      │  DRIVE (×4)
│ DRIVE  │ DRIVE  │ DRIVE  │ DRIVE  │
├────────┼────────┼────────┼────────┤
│ 4      │ 5      │ 6      │ 7      │
│ WRIST  │ WRIST  │ ARMS   │ ARMS   │
├────────┼────────┼────────┼────────┤
│ 8      │ 9      │ A      │ B      │
│ CLAW   │ CLAW   │ LAMP   │ HOME   │
├────────┼────────┼────────┼────────┤
│ WAIT   │        │        │        │
│ NOTE   │ SHIFT  │ CLEAR  │ ENTER  │
│ REST   │ OCTAVE │        │        │
├────────┼────────┼────────┼────────┤
│ SONG   │ CLOCK  │ SPEECH │ MOTION │
│ NOTES  │ STATUS │        │        │
├────────┼────────┼────────┼────────┤
│ GAME   │PROGRAM │ LEARN  │EXECUTE │
├────────┴────────┴────────┴────────┤
│           POWER/STOP              │
└───────────────────────────────────┘
```

Indicator colors on the physical remote: **blue** (song/speech/mode group), **orange** (CLEAR), **yellow** (ENTER, HOME).

---

## Related docs

- Matrix wiring: [`Photos/ReverseEngineering/keyboard-matrix-reference-1.png`](Photos/ReverseEngineering/keyboard-matrix-reference-1.png)
- Opcode mapping: [`Cartridge/PROGRAMMING.md`](../Cartridge/PROGRAMMING.md) §7–8, [`Docs/Technical/06-Input-Output-Guide.md`](../Docs/Technical/06-Input-Output-Guide.md)
- Internal ROM key table: `$E6B5` in [`maxx_internal_ROM.dsm`](../Mainboard/Firmware/Assembly/maxx_internal_ROM.dsm)