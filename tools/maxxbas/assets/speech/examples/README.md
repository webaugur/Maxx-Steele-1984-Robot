# `say` example texts (nursery rhymes)

Short public-domain rhymes for file input demos. Punctuation is set up so
`say --boop` can pick emotive flourishes per line.

```bash
# From repo root (or any cwd — use the full path)
EX=tools/maxxbas/assets/speech/examples

say -f "$EX/twinkle.txt"
say --boop -f "$EX/humpty.txt"
say -v robot --boop -f "$EX/baa-baa.txt"
say -v lady -f "$EX/mary-lamb.txt"
say --boop -o /tmp/spider.wav -f "$EX/itsy-bitsy.txt"
say -v lady --boop -f "$EX/little-moira-bots.txt"   # original Maxx rhyme
```

| File | Rhyme |
|------|--------|
| `twinkle.txt` | Twinkle, Twinkle, Little Star |
| `baa-baa.txt` | Baa, Baa, Black Sheep |
| `humpty.txt` | Humpty Dumpty |
| `mary-lamb.txt` | Mary Had a Little Lamb |
| `itsy-bitsy.txt` | Itsy Bitsy Spider |
| `little-moira-bots.txt` | Little Moira Lost Her Bots (original) |

Bash tip: if you pass rhyme text on the command line with `!`, use **single quotes**.
File input (`-f`) avoids that problem entirely.
