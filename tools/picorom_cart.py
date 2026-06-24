#!/usr/bin/env python3
"""Prepare and upload Maxx Steele cartridge ROM images via PicoROM.

PicoROM (https://github.com/wickerwaka/PicoROM) is an RP2040-based DIP ROM
emulator.  Use PicoROM P28 (28-pin) as a drop-in for U1 on the CBSDemo-class
cartridge board documented in Cartridge/Examples/CBSDemo/KiCAD/.

Requires the PicoROM 2.x host tool: https://github.com/wickerwaka/PicoROM/releases
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path

from maxx_rom import CartImage, validate_cart
from project_paths import resolve_from_root

# Named cartridge images in this repository
CARTS: dict[str, Path] = {
    "ultramaxx": Path("Cartridge/Examples/UltraMaxx/Firmware/Binary/UltraMaxx.532"),
    "cbsdemo": Path("Cartridge/Examples/CBSDemo/Firmware/Binary/CBSDemo.532"),
}

# PicoROM upload size tokens (see PicoROM README — sizes in megabits)
PICOROM_SIZES = {
    "4kb": "32KBit",  # 4 KB — KM2365-class / logical cart image
    "27c512": "512KBit",  # 512 Kbit (64 KB) — factory U1 silkscreen
}

DEFAULT_DEVICE = "maxx_cart"
DEFAULT_SIZE = "4kb"
DEFAULT_CART = "ultramaxx"


def resolve_cart(name: str) -> Path:
    if name not in CARTS:
        raise SystemExit(f"unknown cart {name!r}; choose from: {', '.join(CARTS)}")
    return resolve_from_root(CARTS[name], must_exist=True)


def resolve_rom_path(args: argparse.Namespace) -> Path:
    if getattr(args, "rom", None):
        return resolve_from_root(args.rom, must_exist=True)
    return resolve_cart(args.cart)


def check_cart(path: Path) -> CartImage:
    cart = CartImage.load(path)
    issues = validate_cart(cart)
    if issues:
        for issue in issues:
            print(f"FAIL: {issue}", file=sys.stderr)
        raise SystemExit(1)
    return cart


def picorom_bin() -> str | None:
    return shutil.which("picorom")


def upload_command(
    rom_path: Path,
    device: str,
    size_key: str,
    persist: bool,
) -> list[str]:
    size_token = PICOROM_SIZES.get(size_key)
    if not size_token:
        raise SystemExit(f"unknown size {size_key!r}; choose from: {', '.join(PICOROM_SIZES)}")
    cmd = ["picorom", "upload", device, str(rom_path), size_token]
    if persist:
        cmd.append("-s")
    return cmd


def cmd_info(args: argparse.Namespace) -> int:
    path = resolve_rom_path(args)
    cart = check_cart(path)
    print(f"Image:     {path}")
    print(f"Size:      {len(cart.data)} bytes (4 KB @ ${cart.base_addr:04X})")
    print(f"Entry:     ${cart.entry_vector:04X}")
    print(f"Copyright: {cart.copyright.decode('ascii', errors='replace')!r}")
    print()
    print("PicoROM hardware: P28 (28-pin DIP) in U1 socket — same as CBSDemo board")
    print("Upstream:         https://github.com/wickerwaka/PicoROM")
    print()
    for key, token in PICOROM_SIZES.items():
        cmd = upload_command(path, args.device, key, args.persist)
        print(f"  {key:8} → {' '.join(cmd)}")
    return 0


def cmd_upload(args: argparse.Namespace) -> int:
    path = resolve_rom_path(args)
    check_cart(path)
    cmd = upload_command(path, args.device, args.size, args.persist)

    if args.dry_run:
        print(" ".join(cmd))
        return 0

    picorom = picorom_bin()
    if picorom is None:
        print("picorom not found in PATH. Install from:", file=sys.stderr)
        print("  https://github.com/wickerwaka/PicoROM/releases", file=sys.stderr)
        print()
        print("Run manually:")
        print("  " + " ".join(cmd))
        return 1

    cmd[0] = picorom
    print(" ".join(cmd))
    return subprocess.call(cmd)


def _common_args(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--cart",
        choices=sorted(CARTS),
        default=DEFAULT_CART,
        help=f"named cartridge image (default: {DEFAULT_CART})",
    )
    parser.add_argument(
        "--rom",
        help="path to a .532 image (overrides --cart; use after MaxxBAS compile)",
    )
    parser.add_argument(
        "--device",
        default=DEFAULT_DEVICE,
        help=f"PicoROM device name (default: {DEFAULT_DEVICE}; rename with picorom rename)",
    )
    parser.add_argument(
        "--size",
        choices=sorted(PICOROM_SIZES),
        default=DEFAULT_SIZE,
        help="emulated ROM size (4kb for KM2365-class; 27c512 for factory socket)",
    )
    parser.add_argument(
        "-s",
        "--persist",
        action="store_true",
        help="pass -s to picorom upload (store image in PicoROM flash)",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="print upload command without executing",
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="cmd", required=True)
    p_info = sub.add_parser("info", help="show image metadata and example upload commands")
    _common_args(p_info)
    p_upload = sub.add_parser("upload", help="validate and upload to PicoROM")
    _common_args(p_upload)

    args = parser.parse_args(argv)
    if args.cmd == "info":
        return cmd_info(args)
    if args.cmd == "upload":
        return cmd_upload(args)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())