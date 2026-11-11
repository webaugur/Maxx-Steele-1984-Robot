"""Load a Saleae Logic 1.x or sigrok CSV export of digital channels."""

from __future__ import annotations

import csv
import re
from pathlib import Path


def _clean_name(name: str) -> str:
    name = name.strip().lstrip("\ufeff")
    name = re.sub(r"\[.*?\]", "", name).strip()
    return name


def _is_time_header(name: str) -> bool:
    lower = name.lower()
    return lower.startswith("time") or lower in {"t", "timestamp"}


def _is_index_header(name: str) -> bool:
    return name.lower() in {"sample", "index", "n", "#"}


def _to_bit(value: str) -> int:
    text = value.strip().lower()
    if text in {"1", "high", "h", "true"}:
        return 1
    if text in {"0", "low", "l", "false"}:
        return 0
    try:
        return 1 if float(text) >= 0.5 else 0
    except ValueError as exc:
        raise ValueError(f"Not a logic level: {value!r}") from exc


def _channel_key(name: str, index: int) -> list[str]:
    keys = [_clean_name(name)]
    # Saleae often uses "Channel 0"; sigrok uses "D0" or a renamed label.
    match = re.search(r"(?:channel\s*|ch|d)?\s*(\d+)$", keys[0], re.I)
    if match:
        keys.append(f"CH{match.group(1)}")
        keys.append(f"D{match.group(1)}")
        keys.append(match.group(1))
    keys.append(f"CH{index}")
    keys.append(f"D{index}")
    keys.append(str(index))
    return keys


def load_csv(path: Path) -> tuple[list[float], dict[str, list[int]]]:
    """Return (times, {channel_name: levels}).

    Accepts a time column plus one column per channel, either at every sample
    or only when a value changes. Comment lines starting with # or ; are skipped.
    """
    raw_lines = path.read_text(encoding="utf-8-sig", errors="replace").splitlines()
    rows: list[list[str]] = []
    for line in raw_lines:
        stripped = line.strip()
        if not stripped or stripped.startswith("#") or stripped.startswith(";"):
            continue
        rows.append(next(csv.reader([line])))
    if not rows:
        raise ValueError(f"No data in {path}")

    header = [_clean_name(cell) for cell in rows[0]]
    # If the first row does not look like a header, invent names.
    looks_header = any(
        _is_time_header(h) or re.search(r"[A-Za-z]", h) for h in header
    )
    data_rows = rows[1:] if looks_header else rows
    if not looks_header:
        header = [f"CH{i}" for i in range(len(rows[0]))]
        # Many sigrok CSV files put a sample index in column 0.
        if header:
            header[0] = "sample"

    time_idx = next((i for i, h in enumerate(header) if _is_time_header(h)), None)
    index_idx = next((i for i, h in enumerate(header) if _is_index_header(h)), None)
    if time_idx is None and index_idx is None:
        # First column is time if it has a decimal in the first data row.
        if data_rows and re.search(r"[.eE]", data_rows[0][0]):
            time_idx = 0
        else:
            index_idx = 0

    channel_indices = [
        i
        for i, h in enumerate(header)
        if i != time_idx and i != index_idx
    ]
    if not channel_indices:
        raise ValueError(f"No channel columns in {path}")

    times: list[float] = []
    series: dict[str, list[int]] = {}
    aliases: dict[str, str] = {}
    for n, i in enumerate(channel_indices):
        primary = _clean_name(header[i]) or f"CH{n}"
        series[primary] = []
        for key in _channel_key(primary, n):
            aliases[key.upper()] = primary

    last_time = 0.0
    implied_dt = None
    for row_n, row in enumerate(data_rows):
        if not row or all(not cell.strip() for cell in row):
            continue
        if time_idx is not None and time_idx < len(row) and row[time_idx].strip():
            t = float(row[time_idx].replace(",", ""))
        elif index_idx is not None and index_idx < len(row) and row[index_idx].strip():
            t = float(row[index_idx].replace(",", ""))
            # Sample index; dt unknown until we have a rate. Keep the index.
        else:
            t = last_time if times else 0.0
        times.append(t)
        if len(times) == 2:
            implied_dt = times[1] - times[0]
        last_time = t
        for n, i in enumerate(channel_indices):
            primary = _clean_name(header[i]) or f"CH{n}"
            value = row[i] if i < len(row) else "0"
            series[primary].append(_to_bit(value))

    return times, series, aliases


def pick_channel(series: dict[str, list[int]], aliases: dict[str, str], name: str) -> str:
    if name in series:
        return name
    key = aliases.get(name.upper())
    if key:
        return key
    available = ", ".join(series)
    raise SystemExit(f"No column named {name!r}. Available: {available}")
