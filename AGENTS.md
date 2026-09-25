# Agent instructions

Same rule as [`.cursor/rules/rebuild-before-run.mdc`](.cursor/rules/rebuild-before-run.mdc). It applies to every session.

## Rebuild `maxx` before asking the user to run it

After editing `tools/maxxbas/` or any other file that affects the `maxx` binary, do not tell the user to run `maxx`, `maxx simulate`, or `maxx simulate --gui` until a fresh build has been verified.

Run this preflight yourself, every time, from the repository root:

```bash
python3 tools/ensure_maxx_built
```

- The exit code must be `0`.
- Note the printed version (for example `ok: maxx 0.2.40`) and include it when telling the user to relaunch.
- If the command fails, fix the build and rerun until it passes.

`tools/maxx` also rebuilds when sources are newer than the release binary. That does not replace this check. Run `ensure_maxx_built` before suggesting the user run the simulator.
