#!/usr/bin/env python3
"""Integration tests for unified maxx simulate."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
HELLO = ROOT / "Cartridge/Examples/UltraMaxx/Firmware/Binary/hello.532"
MAXX = ROOT / "tools/bin/maxx"


def run(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [str(MAXX), *args],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )


def test_simulate_json_has_all_layers() -> None:
    proc = run("simulate", str(HELLO), "--json", "--cycles", "18000")
    assert proc.returncode == 0, proc.stderr
    data = json.loads(proc.stdout)
    assert "program" in data
    assert "robot" in data
    assert "firmware" in data
    assert len(data["robot"]["steps"]) == len(data["program"]["steps"])
    assert data["firmware"]["cycles"] > 1000
    assert "visual" in data["robot"]["steps"][0]
    assert "display" in data["robot"]["steps"][0]


def test_simulate_human_output() -> None:
    proc = run("simulate", str(HELLO), "--no-firmware")
    assert proc.returncode == 0, proc.stderr
    assert "Maxx Steele Simulator" in proc.stdout
    assert "Visual program trace" in proc.stdout
    assert "display [F]" in proc.stdout or "display [d]" in proc.stdout


def main() -> int:
    test_simulate_json_has_all_layers()
    test_simulate_human_output()
    print("simulator tests OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())