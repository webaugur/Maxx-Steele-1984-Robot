# GNU Radio 27 MHz OOK captures

Reference documentation for raw **IQ recordings** of the Maxx Steele remote transmitter. The `.dat` files themselves are **not stored in git** (several exceed GitHub's 100 MB limit); keep them locally alongside this folder.

## Capture setup

Recorded with `RemoteSpectrum.grc` / `RemoteSpectrum.py` (David L Norris, 2021):

| Setting | Default |
|---------|---------|
| Center frequency | 27.095 MHz |
| Sample rate | 200 kHz |
| Format | GNU Radio `gr_complex` (complex float32, 8 bytes/sample) |
| Content | 27 MHz OOK packets from the COP411L transmitter |

Approximate duration: `file_size ÷ (200 000 × 8)` seconds.

Filenames are **UTC timestamps** from the flowgraph `file_sink` (`YYYY.MM.DD.HH.MM.SS.dat`).

## IQ capture files (local only)

| File | Size | ~Duration | Session / contents |
|------|------|-----------|-------------------|
| `2021.02.13.20.49.59.dat` | 40 MB | ~25 s | First capture session (2021-02-13 evening). Initial 27 MHz spectrum / OOK sniffing. |
| `2021.02.13.20.51.06.dat` | 83 MB | ~52 s | Continuation of Feb 13 session; longer run at same tuning. |
| `2021.02.13.20.53.49.dat` | 678 MB | ~7 min | Extended Feb 13 capture — longest recording from the first session. |
| `2021.02.14.06.20.58.dat` | 592 MB | ~6 min | Feb 14 morning session opener; wide continuous capture before shorter button tests. |
| `2021.02.14.06.23.37.dat` | 124 MB | ~1.3 min | Feb 14 session; medium-length capture during protocol analysis. |
| `2021.02.14.06.28.28.dat` | 23 MB | ~14 s | Shorter Feb 14 capture. |
| `2021.02.14.06.30.07.dat` | 38 MB | ~24 s | Shorter Feb 14 capture. |
| `2021.02.14.06.50.10.dat` | 14 MB | ~9 s | Per-button OOK test (sidecar `2021.02.14.06.50.10.dat.txt`). |
| `2021.02.14.06.51.12.dat` | 11 MB | ~7 s | Per-button OOK test (sidecar `2021.02.14.06.51.12.dat.txt`). |
| `2021.02.14.06.52.42.dat` | 5 MB | ~3 s | Per-button OOK test (sidecar `2021.02.14.06.52.42.dat.txt`). |
| `2021.02.14.06.54.21.dat` | 15 MB | ~9 s | Per-button OOK test (sidecar `2021.02.14.06.54.21.dat.txt`). |
| `2021.02.14.06.59.07.dat` | 44 B | — | Empty / truncated stub (recording failed or was stopped immediately). |
| `2021.02.14.06.59.23.dat` | 8 MB | ~5 s | Short Feb 14 follow-up capture. |
| `2021.02.14.07.02.05.dat` | 5 MB | ~3 s | Short Feb 14 follow-up capture. |
| `2021.02.14.07.02.35.dat` | 21 MB | ~13 s | Final capture of the Feb 14 morning session. |
| `2021.03.02.22.24.41.dat` | 322 MB | ~3.4 min | Follow-up session (2021-03-02); additional OOK packet captures. |

The four `.dat.txt` sidecars are annotation placeholders for the 06:50–06:54 per-button recordings (currently empty).

## Flowgraph files (in git)

| File | Contents |
|------|----------|
| `RemoteSpectrum.grc` | GNU Radio Companion hierarchy block — spectrum display, OsmoSDR source, squelch, OOK-oriented monitoring. Default tuning 27.095 MHz, 200 kHz sample rate. |
| `RemoteSpectrum.py` | Python output generated from `RemoteSpectrum.grc`; runnable standalone. |
| `RemoteSpectrum_1.grc` | Alternate / earlier flowgraph variant with the same general purpose. |

## Replaying a local capture

1. Place the desired `.dat` file in this directory (or note its path).
2. Open `RemoteSpectrum.grc` in GNU Radio Companion, or run `RemoteSpectrum.py`.
3. Set the `recfile` variable to the `.dat` path.
4. Tune for **27 MHz** RF and inspect OOK timing (~1.55 ms/bit, ~29 ms packet repeat).

See [`docs/transmitter-architecture.md`](../docs/transmitter-architecture.md) for how the transmitter MCU is clocked at the **455 kHz IF** reference so OOK serial data stays coherent with the RF envelope the receiver demodulates.