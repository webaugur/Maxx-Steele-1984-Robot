#!/usr/bin/env python3
"""Prepare and upload Maxx Steele cartridge ROM images via PicoROM.

PicoROM (https://github.com/wickerwaka/PicoROM) is an RP2040-based DIP ROM
emulator.  Use PicoROM P28 (28-pin) as a drop-in for U1 on the CBSDemo-class
cartridge board documented in Cartridge/Examples/CBSDemo/KiCAD/.

Accepts .532 ROM images or .bas / .maxx MaxxBAS source (compiled automatically).

Requires the PicoROM 2.x host tool: https://github.com/wickerwaka/PicoROM/releases

Preferred unified CLI:  python3 tools/maxx upload FILE --device maxx_cart
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path

from maxx_rom import CartImage, validate_cart
from maxx_toolchain import is_maxxbas_source, resolve_rom_input, run_maxx
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
DEFAULT_COPYRIGHT = "ultramaxx"


def resolve_cart(name: str) -> Path:
    if name not in CARTS:
        raise SystemExit(f"unknown cart {name!r}; choose from: {', '.join(CARTS)}")
    return resolve_from_root(CARTS[name], must_exist=True)


def resolve_rom_path(args: argparse.Namespace) -> tuple[Path, object | None]:
    if getattr(args, "rom", None):
        path = resolve_from_root(args.rom, must_exist=True)
        copyright = getattr(args, "copyright", DEFAULT_COPYRIGHT)
        try:
            return resolve_rom_input(path, copyright=copyright)
        except FileNotFoundError as exc:
            raise SystemExit(str(exc)) from exc
    return resolve_cart(args.cart), None


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
    path, tmp = resolve_rom_path(args)
    try:
        cart = check_cart(path)
        print(f"Image:     {path}")
        if args.rom and is_maxxbas_source(resolve_from_root(args.rom)):
            print(f"Source:    {resolve_from_root(args.rom)} (compiled for inspect)")
        print(f"Size:      {len(cart.data)} bytes (4 KB @ ${cart.base_addr:04X})")
        print(f"Entry:     ${cart.entry_vector:04X}")
        print(f"Copyright: {cart.copyright.decode('ascii', errors='replace')!r}")
        print()
        print("PicoROM hardware: P28 (28-pin DIP) in U1 socket — same as CBSDemo board")
        print("Upstream:         https://github.com/wickerwaka/PicoROM")
        print("Toolchain:        python3 tools/maxx upload …")
        print()
        for key, token in PICOROM_SIZES.items():
            cmd = upload_command(path, args.device, key, args.persist)
            print(f"  {key:8} → {' '.join(cmd)}")
        return 0
    finally:
        if tmp is not None:
            Path(tmp.name).unlink(missing_ok=True)


def cmd_upload(args: argparse.Namespace) -> int:
    # Delegate to unified maxx upload when a source/rom file is provided
    if args.rom:
        path = resolve_from_root(args.rom)
        maxx_argv = [
            "upload",
            str(path),
            "--device",
            args.device,
            "--size",
            args.size,
            "--copyright",
            args.copyright,
        ]
        if args.persist:
            maxx_argv.append("--persist")
        if args.dry_run:
            maxx_argv.append("--dry-run")
        return run_maxx(maxx_argv, check=False).returncode

    path, _ = resolve_rom_path(args)
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
        print("Or use: python3 tools/maxx upload --dry-run …")
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
        help="path to .532, .bas, or .maxx (overrides --cart; sources are compiled)",
    )
    parser.add_argument(
        "--copyright",
        default=DEFAULT_COPYRIGHT,
        choices=("cbs", "ultramaxx"),
        help="copyright for MaxxBAS compile (default: ultramaxx)",
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