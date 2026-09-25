Maxx Steele simulator — portable copy
=====================================

Copy this whole MaxxSim folder onto a USB stick. Double-click the launcher
for the computer you are on. It opens the live GUI. No installer.

  Start-Windows.bat
  Start-Mac.command
  Start-Linux.sh          if the stick drops the execute bit:  bash Start-Linux.sh

The internal robot ROM, patches, fonts, logo, and music are inside the
program. You do not copy those separately.

Carts
-----
Put a cartridge image at:

  carts/default.532

The launcher loads it. With no default.532, the simulator runs the internal
ROM only. Any other image:

  linux/maxx simulate --gui carts/some-other.532
  windows\maxx.exe simulate --gui carts\some-other.532

Window size and other GUI state are written to config\ on this stick, not
into the computer's user profile.

Programs
--------
  windows\maxx.exe     Windows 10 or later, 64-bit. Needs a normal OpenGL driver.
  linux\maxx           Needs libasound, and a desktop OpenGL stack
                       (libGL or EGL, libxkbcommon, Wayland or X11).
  macos\maxx           Unsigned. The first launch: right-click, Open.

The same program still has the command-line tools (compile, upload, say).
Only the Start scripts default to the GUI.

HackRF
------
hackrf/ holds hackrf_info and hackrf_transfer for this computer, plus
their license (COPYING, GPL-2.0-or-later). The simulator runs those
programs. It does not link them. If that folder is empty, it uses the
same tools from the computer's PATH.

Linux also needs libusb and libudev, which a desktop already has. The
USB device is root-only until the udev rule is installed once:

  sudo cp hackrf/60-libhackrf.rules /etc/udev/rules.d/
  sudo udevadm control --reload-rules && sudo udevadm trigger

Windows still needs the WinUSB driver (Zadig or the HackRF driver)
the first time the dongle is plugged in. A DLL on the stick does not
register that driver.

With no HackRF plugged in, the terminal prints "HackRF not found" and
the on-screen robot keeps running.

Building the programs
---------------------
Linux, from the git repo:

  sh portable/build_linux.sh

That copies the release binary to linux/maxx.

Windows and macOS binaries come from the GitHub Actions workflow
"Portable simulator". Download the artifacts into windows\ and macos\.
The macOS file is not signed. Gatekeeper will ask for a right-click Open.
