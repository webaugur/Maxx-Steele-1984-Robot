"""Shared pin maps, safety limits, and ASCII wiring diagrams."""

from __future__ import annotations

import argparse
import sys

# Official operating limits. The Saleae +/-15 V protection is for transients only.
SALEAE_VMAX = 5.25
SALEAE_VIH = 2.0
SALEAE_VIL = 0.8
BP_ADC_VMAX = 6.0
BP_IO_VMAX = 5.5
VSW_OK = (4.5, 5.5)

# Seeed Studio / firmware color labels for Bus Pirate v3.x (the 'v' command).
# SparkFun cables are often the reverse of this table.
BP_COLOR = {
    "GND": "brown",
    "3.3V": "red",
    "5.0V": "orange",
    "ADC": "yellow",
    "VPU": "green",
    "AUX": "blue",
    "CLK": "purple",
    "MOSI": "grey",
    "CS": "white",
    "MISO": "black",
}

# Original Saleae Logic 1x9 harness: resistor color code 0-7, grey ground.
# Confirm the grey lead lines up with the ground symbol on the back of the unit.
SALEAE_COLOR = {
    "GND": "grey (labeled GROUND)",
    0: "black",
    1: "brown",
    2: "red",
    3: "orange",
    4: "yellow",
    5: "green",
    6: "blue",
    7: "violet",
}

# Guide pinout for J400. Orientation and polarity are unverified.
# Pin 39 is VBAT, about 10 V — never clip the Saleae or the Bus Pirate ADC to it.
J400 = {
    1: "GND",
    2: "A15",
    3: "A14",
    4: "A13",
    5: "A12",
    6: "A11",
    7: "A7",
    8: "A8",
    9: "A6",
    10: "A9",
    11: "A5",
    12: "OEB",
    13: "A4",
    14: "WRB",
    15: "A3",
    16: "A10",
    17: "A2",
    18: "A1",
    19: "D7",
    20: "A0",
    21: "D6",
    22: "D0",
    23: "D5",
    24: "D1",
    25: "D4",
    26: "D2",
    27: "Audio Input",
    28: "D3",
    29: "F2",
    30: "SA0",
    31: "WEB",
    32: "SA1",
    33: "BIGSPACEB",
    34: "SA2",
    35: "MNMIB",
    36: "SA3",
    37: "RESETSYSB",
    38: "CT",
    39: "VBAT (~10 V) — DO NOT CONNECT Saleae or Bus Pirate ADC",
    40: "DOUT",
    41: "VSW (nominal 5 V)",
    42: "DREQ",
    43: "GND",
    44: "ASTRB",
}

UNUSED_DC_PINS = (
    (27, "Audio Input"),
    (29, "F2"),
    (30, "SA0"),
    (32, "SA1"),
    (33, "BIGSPACEB"),
    (34, "SA2"),
    (35, "MNMIB"),
    (36, "SA3"),
    (37, "RESETSYSB"),
    (38, "CT"),
    (40, "DOUT"),
    (42, "DREQ"),
    (44, "ASTRB"),
)

SAFETY = """
SAFETY
- Robot on the gel cell only. Unplug any charger. The Saleae shares ground
  with the laptop and is not for mains-referenced gear.
- Hook ground first (J400 pin 1 and/or 43). Unhook ground last.
- J400 pin 39 is VBAT, about 10 V. Never clip the Saleae or the Bus Pirate
  ADC (max 6 V) to it. Use a regular meter.
- Measure VSW (pin 41) before any Saleae hookup. Saleae limit is 5.25 V
  continuous. Neither tool will catch brief spikes; that needs a scope.
- Do not probe XTAL0 / XTAL1 (CPU pins 55 and 56). A probe can stall the
  oscillator.
- Leave Bus Pirate 3.3 V, 5 V, and pull-ups off. Do not drive a pin with
  the Bus Pirate until you know it is safe to do so.
- Do not insert a PicoROM with /CE grounded until the OEB test says that
  is safe.
- Original Saleae inputs have a weak pull-up and float around 2.4 V, so a
  disconnected pin can look high. Logic 2 does not support this unit; use
  Logic 1.2.18 or PulseView / sigrok-cli.
""".strip()


def box(title: str, body: str) -> str:
    lines = [title, ""] + body.rstrip().splitlines()
    width = max(len(line) for line in lines)
    rule = "+" + "-" * (width + 2) + "+"
    out = [rule]
    for line in lines:
        out.append("| " + line.ljust(width) + " |")
    out.append(rule)
    return "\n".join(out)


def saleae_row(ch, signal: str, where: str) -> str:
    color = SALEAE_COLOR[ch] if ch != "GND" else SALEAE_COLOR["GND"]
    label = "GND" if ch == "GND" else f"CH{ch}"
    return f"  {label:<5} {color:<24} ->  {where:<28}  {signal}"


DIAGRAMS = {}

DIAGRAMS["01_continuity"] = box(
    "Test 1a — J400 orientation (meter, power OFF)",
    """
  Regular continuity meter (not the Saleae, not the Bus Pirate ADC)

  Meter black  ----------------->  battery negative / chassis ground
  Meter red    ----------------->  each edge finger, one at a time

  The two fingers that beep should be J400 pin 1 and pin 43.
  That tells you which end is pin 1 and which face is odd-numbered.

  Do not power the robot for this step.
""",
)

DIAGRAMS["01_vsw"] = box(
    "Test 1b — VSW voltage (Bus Pirate ADC, power ON)",
    f"""
  Bus Pirate v3.5c (HiZ, supplies off)          J400

  GND  {BP_COLOR['GND']:<8}  ----------------->  pin 1 or 43   GND
  ADC  {BP_COLOR['ADC']:<8}  ----------------->  pin 41        VSW (nominal 5 V)

  Leave disconnected:
    pin 39  VBAT  (~10 V)  — meter only, never the ADC or Saleae
    3.3 V, 5.0 V, VPU, AUX, and all other Bus Pirate pins

  Expected: about 5 V, acceptable {VSW_OK[0]}–{VSW_OK[1]} V.
  If VSW is above {SALEAE_VMAX} V, do not connect the Saleae.
""",
)

DIAGRAMS["02_f2"] = box(
    "Test 2a — F2 frequency (Bus Pirate AUX)",
    f"""
  Bus Pirate v3.5c (HiZ, supplies off)          J400

  GND  {BP_COLOR['GND']:<8}  ----------------->  pin 1 or 43   GND
  AUX  {BP_COLOR['AUX']:<8}  ----------------->  pin 29        F2

  Do not probe CPU pins 55/56 (XTAL0/XTAL1).
  F2 is an unknown clock. This step just records whatever is there.
""",
)

DIAGRAMS["02_f244"] = box(
    "Test 2b — F244 frequency (Bus Pirate AUX)",
    f"""
  Bus Pirate v3.5c (HiZ, supplies off)          Mainboard CPU

  GND  {BP_COLOR['GND']:<8}  ----------------->  J400 pin 1 or 43, or CPU Vss
  AUX  {BP_COLOR['AUX']:<8}  ----------------->  CPU pin 54     labeled F244 Hz

  455 kHz / 2048 is about 222 Hz.  500 kHz / 2048 is about 244 Hz.
  Skip this step if you cannot reach CPU pin 54 safely.
""",
)

DIAGRAMS["02_f499"] = box(
    "Test 2c — F499 frequency (Bus Pirate AUX)",
    f"""
  Bus Pirate v3.5c (HiZ, supplies off)          Mainboard CPU

  GND  {BP_COLOR['GND']:<8}  ----------------->  J400 pin 1 or 43, or CPU Vss
  AUX  {BP_COLOR['AUX']:<8}  ----------------->  CPU pin 57     labeled F499 kHz

  Expect about 455 kHz if the crystal marking is right, or about 500 kHz
  if the labels were written for a 500 kHz clock.
  Skip this step if you cannot reach CPU pin 57 safely.
  Do not probe CPU pins 55/56 (XTAL0/XTAL1).
""",
)

DIAGRAMS["03_oeb"] = box(
    "Test 3 — OEB versus address (Saleae original, 12 or 24 MHz)",
    f"""
  This is the go/no-go for PR55. No PicoROM in the slot.
  Use the cut proto board as a breakout. Robot running (internal ROM).

  Saleae original 1x9 harness                   J400

{saleae_row('GND', 'GND', 'pin 1 and/or 43')}
{saleae_row(0, 'OEB', 'pin 12')}
{saleae_row(1, 'A15', 'pin 2')}
{saleae_row(2, 'A14', 'pin 3')}
{saleae_row(3, 'A13', 'pin 4')}
{saleae_row(4, 'A12', 'pin 5')}
{saleae_row(5, 'WRB', 'pin 14')}
{saleae_row(6, 'WEB', 'pin 31')}
{saleae_row(7, 'optional F2', 'pin 29  (or leave open)')}

  Sample rate: 12 MHz or 24 MHz.  Capture 2–5 seconds while the robot
  is on and running (already booted, fetching from $E000–$FFFF).

  Internal ROM fetches have A15, A14, and A13 all high.
  If OEB is active in that region, a cart with /CE tied to ground will
  fight the internal ROM. If OEB is only active in $2000–$B000, it is
  acting like a chip select and is much safer.

  Export CSV and run:  python3 analyze_oeb.py capture.csv
""",
)

DIAGRAMS["04_reset"] = box(
    "Test 4 — Reset and NMI (Saleae original)",
    f"""
  Saleae original 1x9 harness                   J400 / CPU

{saleae_row('GND', 'GND', 'pin 1 and/or 43')}
{saleae_row(0, 'RESETSYSB', 'J400 pin 37')}
{saleae_row(1, 'MNMIB', 'J400 pin 35')}
{saleae_row(2, 'OEB (activity)', 'J400 pin 12')}
{saleae_row(3, 'BIGSPACEB (opt.)', 'J400 pin 33')}
{saleae_row(4, 'ResetSwB (opt.)', 'CPU pin 45, head stop')}
  CH5–CH7  unused

  Do not pull pin 37 with the Bus Pirate or a switch on this capture.
  Take three short captures, or one long one that includes all three:
    1. Idle / running
    2. Press the head stop button
    3. Power-on (clip leads before applying power; VSW must already
       have been measured in test 1 and be under {SALEAE_VMAX} V)

  Note idle levels and whether pin 37 or 35 moves. The Saleae pull-up
  will make a truly open pin sit around 2.4 V (reads high).

  Optional:  python3 analyze_timing.py capture.csv --channels RESETSYSB,MNMIB,OEB
""",
)

DIAGRAMS["05_radio"] = box(
    "Test 5 — RadioIn (Saleae original)",
    f"""
  Saleae original 1x9 harness                   Robot

{saleae_row('GND', 'GND', 'J400 pin 1 or 43, or CPU Vss')}
{saleae_row(0, 'RadioIn', 'CPU pin 25')}
{saleae_row(1, 'OEB (optional)', 'J400 pin 12')}
  CH2–CH7  unused

  Sample rate: 1 MHz is plenty (bit period is expected around 1.5–1.7 ms).
  Capture while pressing keys on the 27 MHz transmitter.

  You want a burst of bits on CPU pin 25 and a measured bit period.

  Optional:  python3 analyze_timing.py capture.csv --channels RadioIn
""",
)

DIAGRAMS["06_unused"] = box(
    "Test 6 — Unused J400 DC levels (Bus Pirate ADC)",
    f"""
  Bus Pirate v3.5c (HiZ, supplies off)          J400

  GND  {BP_COLOR['GND']:<8}  ----------------->  pin 1 or 43   GND
  ADC  {BP_COLOR['ADC']:<8}  ----------------->  the pin the script names next

  NEVER move the ADC lead to pin 39 (VBAT, ~10 V).
  The script walks the unused fingers one at a time and records the
  voltage. A mid level can mean the net is floating (the ADC loads the
  pin with about 20 kΩ).

  Do this after test 3. A meter is also fine if you prefer.
""",
)


def pause(prompt: str = "Press Enter when this is hooked up (or type s to skip)...") -> str:
    try:
        return input(prompt).strip().lower()
    except EOFError:
        return "s"


def add_common_args(parser: argparse.ArgumentParser, need_port: bool = True) -> None:
    if need_port:
        parser.add_argument(
            "--port",
            help="Bus Pirate serial port, for example /dev/ttyUSB0 or COM3",
        )
    parser.add_argument(
        "--wiring",
        action="store_true",
        help="Print the wiring diagram and exit",
    )


def die(message: str, code: int = 1) -> None:
    print(message, file=sys.stderr)
    raise SystemExit(code)
