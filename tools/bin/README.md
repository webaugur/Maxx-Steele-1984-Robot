# Maxx Steele command-line tools

Add this directory to your `PATH` once:

```bash
export PATH="/path/to/Maxx-Steele/tools/bin:$PATH"
```

Or from a clone of this repo:

```bash
export PATH="$(git -C /path/to/Maxx-Steele rev-parse --show-toplevel)/tools/bin:$PATH"
```

## Commands

| Command | Wraps | Examples |
|---------|-------|----------|
| `maxx` | `tools/maxx` | `maxx compile hello.bas`, `maxx upload hello.bas --device maxx_cart` |
| `maxxbas` | alias → `maxx` | `maxxbas check hello.bas` |
| `maxx-compile` | `maxx compile` | `maxx-compile hello.bas -o hello.532` |
| `maxx-rom` | `tools/maxx_rom.py` | `maxx-rom disasm UltraMaxx.532`, `maxx-rom validate hello.532` |
| `picorom-cart` | `tools/picorom_cart.py` | `picorom-cart upload --rom hello.bas --device maxx_cart` |
| `say` | `maxx say` | `say "Hello. I am Maxx Steele."`, `say -p 0x10`, `say -o out.wav hi` |
| `sim` | `maxx simulate` | `sim --gui` |

### `say` — Maxx SAM speech

Drop-in style replacement for macOS `say`, using Software Automatic Mouth
(rustsam). Default voice is **Little Robot** (same as the interactive simulator).

```bash
say "Hello. I am Maxx Steele."
say -v ?                            # list voices (macOS-style)
say -v elf "Hello there."           # classic SAM preset
say -v Alex "Hello."                # macOS name → SAM approximation
say -v Zarvox -s "I am a robot"     # novelty + sing mode
say --phrase 0x10                   # built-in ROM phrase
say -f notes.txt
echo "I'm ready." | say
say -o clip.wav "Good play."
```

**Important:** Audio is always **SAM / rustsam**, not Apple TTS. macOS voice
names (`Alex`, `Samantha`, `Zarvox`, …) are accepted for CLI compatibility and
mapped to different formant presets. They will not sound like macOS.

**Classic SAM** (`-v` / `--voice`, default `robot`): `sam`, `elf`, `robot`,
`stuffy`, `lady`, `alien`.

**macOS-style names:** dozens of common `say -v` names (English + many locale
voice *names*) are registered — see `say -v ?`. Reciter is English-only.

**`-s` / `--sing`**: SAM sing mode (steadier pitch for melodic speech).

**`--boop`**: After each statement (split on `.` `!` `?`), play a Maxx-style
emotive beep-boop. Pattern is chosen from **punctuation** (with light word cues);
falls back to random when there’s no useful mark.

| Ending | Boops (pick among) |
|--------|--------------------|
| `?` | curious, whistle |
| `!` | happy, spark, giggle |
| `!!`+ | spark, alert |
| `!?` / `?!` | alert, curious, spark |
| `...` / `…` | think, scan |
| `.` | affirm, soft, greet (+ bye/greet word cues) |
| none | random |

```bash
# Correct: single quotes when the text contains ! (bash history expansion)
say --boop 'Hello! How are you? I want to play with your circuits!!'
say --boop-pattern greet 'Ready!'
say --boop "Hello. Ready?"           # no ! — double quotes are fine
say --boop-pattern ?                 # list patterns
```

**Bash / `!`:** Interactive bash expands `!!` (and similar) **even inside double quotes**,
before `say` runs. **The correct workaround is single quotes** around the spoken text.

Patterns: `greet`, `curious`, `happy`, `affirm`, `think`, `alert`, `soft`,
`spark`, `bye`, `whistle`, `giggle`, `scan`.

Requires `python3` on `PATH`. The `maxx` command builds the Rust release binary on first use.