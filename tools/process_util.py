#!/usr/bin/env python3
"""Subprocess helpers — clean exit on SIGINT / KeyboardInterrupt."""

from __future__ import annotations

import signal
import subprocess
import sys
from collections.abc import Sequence
from typing import Any, NoReturn

INTERRUPTED = 130


def interrupted_exit(message: str | None = "Interrupted.") -> NoReturn:
    """Print a short message and exit with the conventional SIGINT code."""
    if message:
        print(f"\n{message}", file=sys.stderr)
    raise SystemExit(INTERRUPTED) from None


def _stop_process(proc: subprocess.Popen[Any], *, sig: int = signal.SIGINT) -> None:
    if proc.poll() is not None:
        return
    try:
        if sys.platform == "win32":
            proc.terminate()
        else:
            proc.send_signal(sig)
    except (ProcessLookupError, PermissionError, OSError):
        try:
            proc.terminate()
        except OSError:
            pass
    try:
        proc.wait(timeout=3)
    except subprocess.TimeoutExpired:
        proc.kill()
        try:
            proc.wait(timeout=1)
        except subprocess.TimeoutExpired:
            pass


def run(
    cmd: Sequence[str],
    *,
    interrupt_msg: str | None = "Interrupted.",
    **kwargs: Any,
) -> int:
    """Run *cmd* to completion; return exit code (130 on user interrupt)."""
    proc = subprocess.Popen(list(cmd), **kwargs)
    try:
        return proc.wait()
    except KeyboardInterrupt:
        _stop_process(proc)
        if interrupt_msg:
            print(f"\n{interrupt_msg}", file=sys.stderr)
        return INTERRUPTED


def run_checked(
    cmd: Sequence[str],
    *,
    check: bool = True,
    interrupt_msg: str | None = "Interrupted.",
    **kwargs: Any,
) -> subprocess.CompletedProcess[Any]:
    """Like :func:`subprocess.run`, without dumping a traceback on Ctrl+C."""
    try:
        completed = subprocess.run(list(cmd), **kwargs)
    except KeyboardInterrupt:
        if interrupt_msg:
            print(f"\n{interrupt_msg}", file=sys.stderr)
        interrupted_exit(interrupt_msg)
    if check and completed.returncode != 0:
        raise subprocess.CalledProcessError(
            completed.returncode,
            completed.args,
            output=completed.stdout,
            stderr=completed.stderr,
        )
    return completed


def main_guard(main_fn, argv: list[str] | None = None) -> NoReturn:
    """Call *main_fn* inside a top-level KeyboardInterrupt guard."""
    try:
        raise SystemExit(main_fn(argv))
    except KeyboardInterrupt:
        interrupted_exit()