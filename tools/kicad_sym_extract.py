"""Extract symbol definitions from KiCad .kicad_sym libraries."""

from __future__ import annotations

from pathlib import Path

KICAD_SYM_DIR = Path("/usr/share/kicad/symbols")


def extract_symbol_block(lib_id: str) -> str:
    """Return lib_symbols entry for Library:SymbolName."""
    lib_name, sym_name = lib_id.split(":", 1)
    lib_path = KICAD_SYM_DIR / f"{lib_name}.kicad_sym"
    if not lib_path.exists():
        raise FileNotFoundError(lib_path)

    text = lib_path.read_text()
    needle = f'(symbol "{sym_name}"'
    start = text.find(needle)
    if start < 0:
        raise ValueError(f"{sym_name} not found in {lib_path}")

    depth = 0
    end = start
    for i, ch in enumerate(text[start:], start):
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                end = i + 1
                break

    block = text[start:end]
    return block.replace(f'(symbol "{sym_name}"', f'(symbol "{lib_id}"', 1)