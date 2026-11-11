"""Bus Pirate v3.5c helpers for HiZ voltage and frequency measurements."""

from __future__ import annotations

import re
import sys
import time
from typing import Optional

try:
    import serial
    from serial.tools import list_ports
except ImportError:  # pragma: no cover
    serial = None
    list_ports = None


def _require_serial() -> None:
    if serial is None:
        raise SystemExit(
            "This script needs pyserial. Install it with:  pip install pyserial"
        )


def list_serial_ports() -> list[str]:
    _require_serial()
    return [p.device for p in list_ports.comports()]


def guess_port() -> Optional[str]:
    ports = list_serial_ports()
    preferred = []
    for name in ports:
        lower = name.lower()
        if any(tag in lower for tag in ("usb", "usbserial", "usbmodem", "acm", "fti", "com")):
            preferred.append(name)
    if len(preferred) == 1:
        return preferred[0]
    if len(ports) == 1:
        return ports[0]
    return None


class BusPirate:
    """Talk to a Bus Pirate v3.5c in HiZ. Power supplies are left off."""

    def __init__(self, port: str, baud: int = 115200, timeout: float = 2.0):
        _require_serial()
        self.port = port
        self.ser = serial.Serial(port, baud, timeout=timeout)
        time.sleep(0.25)
        banner = self.reset_to_hiz()
        if "HiZ" not in banner and "Bus Pirate" not in banner:
            # One more try after a raw newline, common with FTDI on first open.
            self.ser.write(b"\n")
            time.sleep(0.2)
            banner = self.reset_to_hiz()
        self.banner = banner

    def reset_to_hiz(self) -> str:
        self.ser.reset_input_buffer()
        self.ser.write(b"#\n")
        time.sleep(0.4)
        self.ser.write(b"\n")
        time.sleep(0.2)
        text = self._read_all()
        # Supplies and pull-ups should already be off after a software reset.
        # Send 'w' anyway so a previous session cannot leave 5 V enabled.
        self.ser.write(b"w\n")
        time.sleep(0.2)
        text += self._read_all()
        return text

    def _read_all(self) -> str:
        chunks = []
        idle = 0
        while idle < 3:
            waiting = self.ser.in_waiting
            if waiting:
                chunks.append(self.ser.read(waiting))
                idle = 0
            else:
                time.sleep(0.05)
                idle += 1
        return b"".join(chunks).decode("ascii", errors="replace")

    def cmd(self, command: str, wait: float = 0.25) -> str:
        self.ser.reset_input_buffer()
        self.ser.write((command + "\n").encode("ascii"))
        time.sleep(wait)
        return self._read_all()

    def voltage(self) -> float:
        text = self.cmd("d", wait=0.35)
        match = re.search(r"([0-9]+(?:\.[0-9]+)?)\s*V", text, re.I)
        if not match:
            raise RuntimeError(f"Could not parse a voltage from:\n{text}")
        return float(match.group(1))

    def frequency_hz(self) -> float:
        # The 'f' command counts for about one second, then may autorange
        # and count for a second more. Wait long enough for both.
        text = self.cmd("f", wait=3.5)
        match = re.search(
            r"([0-9][0-9,]*(?:\.[0-9]+)?)\s*Hz", text, re.I
        )
        if not match:
            if re.search(r"frequency", text, re.I):
                return 0.0
            raise RuntimeError(f"Could not parse a frequency from:\n{text}")
        return float(match.group(1).replace(",", ""))

    def close(self) -> None:
        try:
            self.cmd("w", wait=0.2)
        except Exception:
            pass
        try:
            self.ser.close()
        except Exception:
            pass

    def __enter__(self) -> "BusPirate":
        return self

    def __exit__(self, *exc) -> None:
        self.close()


def connect_or_exit(port: Optional[str]) -> BusPirate:
    if not port:
        port = guess_port()
    if not port:
        found = list_serial_ports()
        hint = (
            "No serial ports found."
            if not found
            else "Available ports: " + ", ".join(found)
        )
        raise SystemExit(
            f"{hint}\nPass the Bus Pirate port with --port "
            "(for example --port /dev/ttyUSB0 or --port COM3)."
        )
    try:
        bp = BusPirate(port)
    except SystemExit:
        raise
    except Exception as exc:
        raise SystemExit(f"Could not open {port}: {exc}") from exc
    print(f"Connected to Bus Pirate on {port}.")
    print("Leave the Bus Pirate supplies off. Do not enable pull-ups.")
    print(
        "Seeed cable colors are used below. SparkFun cables are often "
        "reversed — if a color does not match, go by the pin name on "
        "the header, or type v at the HiZ> prompt."
    )
    print()
    return bp
