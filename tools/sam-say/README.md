# sam-say

Shared **SAM (Software Automatic Mouth)** speech helpers for Maxx Steele and other projects.

- **Voices** — classic SamJs presets (`robot`, `elf`, …) plus macOS `say -v` *name* aliases (formant approximations, not Apple TTS)
- **Boops** — emotive post-speech chirps with punctuation-aware selection
- **Mouth** — simple open/close chatter for GUI or terminal (`--mouth`)
- **Playback** (feature) — rodio play, tick loop, WAV write

## Use as a dependency

```toml
# path (this repo)
sam-say = { path = "../sam-say", features = ["playback"] }

# synth only (no audio device)
sam-say = { path = "../sam-say", default-features = false }
```

```rust
use sam_say::{find_voice, synthesize_voice, write_wav};

let voice = find_voice("robot")?;
let samples = synthesize_voice("Hello.", voice, false)?;
write_wav("/tmp/hi.wav", &samples)?;
```

With `playback`:

```rust
use sam_say::play_samples_with_mouth;
play_samples_with_mouth(&samples)?;
```

## Features

| Feature | Default | Provides |
|---------|---------|----------|
| `playback` | yes | `rodio` play, `AudioOutput`, mouth TTY animation |

## Notes

- Engine is [`rustsam`](https://github.com/thedjinn/rustsam) (git dependency).
- Reciter is English-only; non-English macOS voice *names* only change formants.
- SAM reverse-engineering heritage: see rustsam / SamJs licensing notes.
