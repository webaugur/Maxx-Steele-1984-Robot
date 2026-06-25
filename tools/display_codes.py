"""Detect and mark Maxx LED display prompt strings for User Manual output."""

from __future__ import annotations

import re

DISPLAY_QUOTE_RE = re.compile(
    r'[""\'\u201c\u201d\u2018\u2019]([^""\'\u201c\u201d\u2018\u2019\n]{1,16})'
    r'[""\'\u201c\u201d\u2018\u2019]'
)
DISPLAY_BACKTICK_RE = re.compile(r"`([^`]+)`")
BACKTICK_SPAN_RE = re.compile(r"`[^`]+`")

SPEECH_REJECT = frozenset(
    {
        "hello",
        "thank",
        "sorry",
        "please",
        "game",
        "choose",
        "ready",
        "morning",
        "congratulations",
        "energy",
        "over",
    }
)


def is_display_code(text: str) -> bool:
    """Return True when text looks like a Maxx head-display token."""
    code = text.strip()
    if not code or len(code) > 16 or " " in code:
        return False
    if not re.fullmatch(r"[A-Za-z0-9_./:'-]+", code):
        return False
    if any(ch in code for ch in "'\u2019\u2018"):
        return False
    if code.endswith(".") and len(code) > 8:
        return False
    if re.fullmatch(r"[a-z][a-z.!?]*", code) and code not in SPEECH_REJECT:
        return False
    if re.fullmatch(r"\d{1,2}:\d{2}", code):
        return True
    if "__" in code or code.endswith("_"):
        return True
    if re.fullmatch(r"[A-Z]{2,8}", code):
        return True
    if re.search(r"[A-Z]", code) and re.search(r"[a-z]", code):
        return True
    if re.fullmatch(r"[A-Za-z]{2,4}", code) and code[0].isupper():
        return True
    return False


def wrap_display_codes(text: str) -> str:
    """Convert quoted display tokens to backticks for markdown source."""

    def quote_repl(match: re.Match[str]) -> str:
        inner = match.group(1).strip().rstrip(".,;:")
        if is_display_code(inner):
            return f"`{inner}`"
        return match.group(0)

    parts: list[str] = []
    last = 0
    for span in BACKTICK_SPAN_RE.finditer(text):
        before = text[last : span.start()]
        parts.append(DISPLAY_QUOTE_RE.sub(quote_repl, before))
        parts.append(span.group(0))
        last = span.end()
    parts.append(DISPLAY_QUOTE_RE.sub(quote_repl, text[last:]))
    return "".join(parts)


def latex_escape(text: str) -> str:
    escaped = text.replace("\\", "\\textbackslash{}")
    for char, repl in (("_", "\\_"), ("#", "\\#"), ("%", "\\%"), ("&", "\\&")):
        escaped = escaped.replace(char, repl)
    return escaped


def led_latex(code: str) -> str:
    """Render a display token; split on / because DSEG7 lacks that glyph."""
    parts = code.split("/")
    if len(parts) > 1:
        return "/".join(f"\\LED{{{latex_escape(part)}}}" for part in parts)
    return f"\\LED{{{latex_escape(code)}}}"


def convert_display_backticks_to_latex(text: str) -> str:
    """Map `AEon` markdown to \\LED{AEon} for PDF (skip `<KEY>` controller spans)."""

    def repl(match: re.Match[str]) -> str:
        inner = match.group(1)
        if "<" in inner or ">" in inner:
            return match.group(0)
        if is_display_code(inner):
            return led_latex(inner)
        return match.group(0)

    return DISPLAY_BACKTICK_RE.sub(repl, text)