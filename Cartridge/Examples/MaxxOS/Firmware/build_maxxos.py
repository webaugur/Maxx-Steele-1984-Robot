#!/usr/bin/env python3
"""Build MaxxOS cartridge ROM — extended bootstrap + math quiz."""

from __future__ import annotations

import struct
import sys
from dataclasses import dataclass, field
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[4]
sys.path.insert(0, str(REPO_ROOT / "tools"))
from maxx_rom import COPYRIGHT_MAXXOS, CART_SIZE  # noqa: E402

BASE = 0xA000
ENTRY = 0xA013
MAIN = 0xA080

# Internal ROM entry points
ROM = {
    "ED48": 0xED48,
    "ED4F": 0xED4F,
    "E3EC": 0xE3EC,
    "E60D": 0xE60D,
    "EF63": 0xEF63,
    "F475": 0xF475,
    "F47E": 0xF47E,
    "F684": 0xF684,
    "F8BE": 0xF8BE,
}

# Zero page game state
ZP_SCORE = 0x24
ZP_A = 0x30
ZP_B = 0x31
ZP_EXPECT = 0x32
ZP_OP = 0x33
ZP_LFSR = 0x34  # incremented each rand_byte; EOR with $3A for entropy
ZP_ANS = 0x35

# Cart segment glyphs (7-segment patterns tuned against ROM tables)
SEG_DATA = 0xA1F0


@dataclass
class Asm:
    org: int = BASE
    data: dict[int, int] = field(default_factory=dict)
    labels: dict[str, int] = field(default_factory=dict)
    comments: dict[int, str] = field(default_factory=dict)
    _fixups: list[tuple[int, str]] = field(default_factory=list)

    def set_org(self, addr: int) -> None:
        self.org = addr

    def label(self, name: str) -> None:
        self.labels[name] = self.org

    def comment(self, text: str) -> None:
        self.comments[self.org] = text

    def emit(self, *values: int) -> None:
        for value in values:
            self.data[self.org] = value & 0xFF
            self.org += 1

    def addr(self, name: str) -> int:
        return self.labels[name]

    def _queue_branch(self, opcode: int, target: str | int) -> None:
        self.emit(opcode, 0x00)
        self._fixups.append((self.org - 1, target if isinstance(target, str) else f"@{target:04X}"))

    def resolve(self) -> None:
        for fix_addr, target in self._fixups:
            if target.startswith("@"):
                addr = int(target[1:], 16)
            else:
                addr = self.labels[target]
            offset = addr - (fix_addr + 1)
            if not -128 <= offset <= 127:
                raise ValueError(f"branch out of range at ${fix_addr:04X} -> ${addr:04X}")
            self.data[fix_addr] = offset & 0xFF

    def jmp(self, target: str | int) -> None:
        if isinstance(target, str):
            addr = self.labels[target]
        else:
            addr = target
        self.emit(0x4C, addr & 0xFF, (addr >> 8) & 0xFF)

    def jsr(self, name: str) -> None:
        if name in ROM:
            addr = ROM[name]
        else:
            addr = self.addr(name)
        self.emit(0x20, addr & 0xFF, (addr >> 8) & 0xFF)

    def beq(self, target: str | int) -> None:
        self._queue_branch(0xF0, target)

    def bne(self, target: str | int) -> None:
        self._queue_branch(0xD0, target)

    def bcc(self, target: str | int) -> None:
        self._queue_branch(0x90, target)

    def bcs(self, target: str | int) -> None:
        self._queue_branch(0xB0, target)

    def bpl(self, target: str | int) -> None:
        self._queue_branch(0x10, target)

    def bmi(self, target: str | int) -> None:
        self._queue_branch(0x30, target)


def build_bootstrap(asm: Asm) -> None:
    asm.set_org(ENTRY)
    asm.comment("MaxxOS bootstrap — never JMP $E0B6")
    asm.emit(0xA9, 0x02)  # LDA #$02
    asm.emit(0x85, 0x02)  # STA $02
    asm.emit(0xA9, 0x82)  # LDA #$82
    asm.emit(0x85, 0x03)  # STA $03
    asm.emit(0x58)  # CLI
    asm.emit(0xA9, 0x80)  # LDA #$80
    asm.emit(0x85, 0x75)  # STA $75
    asm.jmp(MAIN)


def build_subroutines(asm: Asm) -> None:
    """Leaf subroutines first, then callers (single-pass assembly)."""
    asm.set_org(0xA0E0)

    asm.label("rand_byte")
    asm.emit(0xE6, ZP_LFSR)  # INC $34
    asm.emit(0xA5, ZP_LFSR)
    asm.emit(0x45, 0x3A)  # EOR $3A (simulator / IRQ stir)
    asm.emit(0x60)

    asm.label("rand_digit")
    asm.jsr("rand_byte")
    asm.emit(0x29, 0x07)  # digit 1..8 (keeps addition sums <= 9)
    asm.emit(0x18)  # CLC
    asm.emit(0x69, 0x01)
    asm.emit(0x60)

    asm.label("calc_add")
    asm.emit(0xA5, ZP_A)
    asm.emit(0x18)
    asm.emit(0x65, ZP_B)
    asm.emit(0x85, ZP_EXPECT)
    asm.emit(0x60)

    asm.label("calc_sub")
    asm.emit(0xA5, ZP_A)
    asm.emit(0x38)
    asm.emit(0xE5, ZP_B)
    asm.emit(0x85, ZP_EXPECT)
    asm.emit(0x60)

    asm.label("delay")
    asm.jsr("E3EC")
    asm.emit(0x60)

    asm.label("disp_a_op")
    asm.jsr("ED48")
    asm.emit(0xA5, ZP_A)
    asm.emit(0xAA)
    asm.emit(0xBD, 0xBE, 0xF8)
    asm.jsr("ED4F")
    asm.emit(0xA5, ZP_OP)
    asm.emit(0xAA)
    asm.emit(0xBD, SEG_DATA & 0xFF, SEG_DATA >> 8)  # LDA $A1F0,X (+/−)
    asm.jsr("ED4F")
    asm.emit(0xA9, 0xE2)
    asm.jsr("ED4F")
    asm.emit(0x60)

    asm.label("disp_a_b")
    asm.jsr("ED48")
    asm.emit(0xA5, ZP_A)
    asm.emit(0xAA)
    asm.emit(0xBD, 0xBE, 0xF8)
    asm.jsr("ED4F")
    asm.emit(0xA5, ZP_B)
    asm.emit(0xAA)
    asm.emit(0xBD, 0xBE, 0xF8)
    asm.jsr("ED4F")
    asm.emit(0xA9, 0xE2)
    asm.jsr("ED4F")
    asm.emit(0x60)

    asm.label("disp_prompt")
    asm.jsr("ED48")
    asm.emit(0xAD, SEG_DATA + 2, (SEG_DATA + 2) >> 8)
    asm.jsr("ED4F")
    asm.emit(0xAD, SEG_DATA + 3, (SEG_DATA + 3) >> 8)
    asm.jsr("ED4F")
    asm.emit(0xA9, 0xE2)
    asm.jsr("ED4F")
    asm.emit(0x60)

    asm.label("disp_answer")
    asm.jsr("ED48")
    asm.emit(0xA5, ZP_ANS)
    asm.emit(0xAA)
    asm.emit(0xBD, 0xBE, 0xF8)
    asm.jsr("ED4F")
    asm.emit(0xAD, SEG_DATA + 3, (SEG_DATA + 3) >> 8)
    asm.jsr("ED4F")
    asm.emit(0xA9, 0xE2)
    asm.jsr("ED4F")
    asm.emit(0x60)

    asm.label("show_score")
    asm.jsr("ED48")
    asm.emit(0xA5, ZP_SCORE)
    asm.emit(0xAA)
    asm.emit(0xBD, 0xBE, 0xF8)
    asm.jsr("ED4F")
    asm.emit(0xAD, SEG_DATA + 3, (SEG_DATA + 3) >> 8)
    asm.jsr("ED4F")
    asm.emit(0xA9, 0xE2)
    asm.jsr("ED4F")
    asm.jsr("delay")
    asm.emit(0x60)

    asm.label("show_problem")
    asm.jsr("disp_a_op")
    asm.jsr("delay")
    asm.jsr("disp_a_b")
    asm.jsr("delay")
    asm.jsr("disp_prompt")
    asm.emit(0x60)

    asm.label("input_answer")
    asm.emit(0xA9, 0xFF)
    asm.emit(0x85, ZP_ANS)
    asm.label("input_loop")
    asm.jsr("E60D")
    asm.emit(0xC9, 0x0E)
    asm.beq("input_clear")
    asm.emit(0xC9, 0x0F)
    asm.beq("input_done")
    asm.emit(0xC9, 0x0A)
    asm.bpl("input_loop")
    asm.emit(0x85, ZP_ANS)
    asm.jsr("disp_answer")
    asm.jmp("input_loop")
    asm.label("input_clear")
    asm.emit(0xA9, 0xFF)
    asm.emit(0x85, ZP_ANS)
    asm.jsr("disp_prompt")
    asm.jmp("input_loop")
    asm.label("input_done")
    asm.emit(0xA5, ZP_ANS)
    asm.emit(0xC9, 0xFF)
    asm.beq("input_loop")
    asm.emit(0x60)

    asm.label("new_problem")
    asm.jsr("rand_byte")
    asm.emit(0x29, 0x01)
    asm.emit(0x85, ZP_OP)
    asm.jsr("rand_digit")
    asm.emit(0x85, ZP_A)
    asm.jsr("rand_digit")
    asm.emit(0x85, ZP_B)
    asm.emit(0xA5, ZP_OP)
    asm.beq("chk_add")
    asm.emit(0xA5, ZP_A)
    asm.emit(0xC5, ZP_B)
    asm.bcs("do_sub")
    asm.emit(0xA5, ZP_A)
    asm.emit(0x85, ZP_ANS)
    asm.emit(0xA5, ZP_B)
    asm.emit(0x85, ZP_A)
    asm.emit(0xA5, ZP_ANS)
    asm.emit(0x85, ZP_B)
    asm.label("do_sub")
    asm.jsr("calc_sub")
    asm.emit(0x60)
    asm.label("chk_add")
    asm.emit(0xA5, ZP_A)
    asm.emit(0x18)
    asm.emit(0x65, ZP_B)
    asm.emit(0xC9, 0x0A)
    asm.bcc("do_add")
    asm.emit(0xA9, 0x03)
    asm.emit(0x85, ZP_A)
    asm.emit(0xA9, 0x04)
    asm.emit(0x85, ZP_B)
    asm.label("do_add")
    asm.jsr("calc_add")
    asm.emit(0x60)

    asm.set_org(SEG_DATA)
    asm.comment("segment glyphs: + - ? blank")
    asm.emit(0x2A, 0x10, 0xEE, 0x10)


def build_main_loop(asm: Asm) -> None:
    asm.set_org(MAIN)

    asm.label("main")
    asm.comment("score=0, show PLAy, speak ready")
    asm.emit(0xA9, 0x00)
    asm.emit(0x85, ZP_SCORE)
    asm.emit(0xA9, 0x37)  # non-zero LFSR seed (0 traps rand_digit)
    asm.emit(0x85, ZP_LFSR)
    asm.emit(0xA2, 0x09)
    asm.jsr("F684")
    asm.emit(0xA2, 0x20)
    asm.jsr("F475")
    asm.jsr("F47E")

    asm.label("game_loop")
    asm.jsr("new_problem")
    asm.label("round_loop")
    asm.jsr("show_problem")
    asm.jsr("input_answer")
    asm.emit(0xA5, ZP_ANS)
    asm.emit(0xC5, ZP_EXPECT)
    asm.bne("wrong_answer")
    asm.emit(0xE6, ZP_SCORE)
    asm.emit(0xA2, 0x13)
    asm.jsr("F475")
    asm.jsr("F47E")
    asm.jsr("show_score")
    asm.jmp("game_loop")
    asm.label("wrong_answer")
    asm.emit(0xA2, 0x04)
    asm.jsr("F684")
    asm.jmp("round_loop")


def build_all_routines(asm: Asm) -> None:
    build_subroutines(asm)
    build_main_loop(asm)
    asm.resolve()


def pack_image(asm: Asm) -> bytearray:
    img = bytearray(b"\xFF" * CART_SIZE)
    struct.pack_into("<H", img, 0, ENTRY)
    img[2 : 2 + len(COPYRIGHT_MAXXOS)] = COPYRIGHT_MAXXOS
    for addr, byte in asm.data.items():
        off = addr - BASE
        if 0 <= off < CART_SIZE:
            img[off] = byte
    return img


def format_dsm(img: bytearray, asm: Asm) -> str:
    lines = [
        "// MaxxOS cartridge — extended bootstrap + math quiz",
        "// located at $A000",
        "",
        f"{BASE:06X}: 13A0  entry jump vector to ${ENTRY:04X}",
        "",
        "\tcopyright characters",
    ]
    cr = COPYRIGHT_MAXXOS
    for i in range(0, len(cr), 4):
        chunk = cr[i : i + 4]
        hexpart = "".join(f"{b:02X}" for b in chunk)
        asc = chunk.decode("ascii", errors="replace")
        lines.append(f"{BASE + 2 + i:06X}: {hexpart}  {asc}")
    lines.append("")
    lines.append("\tcode executed in cartridge")

    addrs = sorted(asm.data.keys())
    i = 0
    while i < len(addrs):
        addr = addrs[i]
        if addr < ENTRY:
            i += 1
            continue
        comment = asm.comments.get(addr)
        if comment:
            lines.append(f"\t{comment}")
        # group up to 3 opcode bytes per line when no comments interrupt
        chunk_addrs = [addr]
        j = i + 1
        while j < len(addrs) and addrs[j] == addrs[j - 1] + 1 and len(chunk_addrs) < 3:
            if asm.comments.get(addrs[j]):
                break
            chunk_addrs.append(addrs[j])
            j += 1
        hexbytes = " ".join(f"{asm.data[a]:02X}" for a in chunk_addrs)
        lines.append(f"{addr:06X}: {hexbytes}")
        i = j

    lines.append("")
    lines.append(f"\tsegment data @ ${SEG_DATA:04X}")
    lines.append(f"{SEG_DATA:06X}: 2A 10 EE 10  + - ? blank")
    return "\n".join(lines) + "\n"


def main() -> None:
    asm = Asm()
    build_all_routines(asm)
    build_bootstrap(asm)
    img = pack_image(asm)
    out_dir = Path(__file__).resolve().parent
    bin_path = out_dir / "Binary" / "MaxxOS.532"
    dsm_path = out_dir / "Assembly" / "maxxos_ROM_532.dsm"
    bin_path.parent.mkdir(parents=True, exist_ok=True)
    dsm_path.parent.mkdir(parents=True, exist_ok=True)
    bin_path.write_bytes(img)
    dsm_path.write_text(format_dsm(img, asm))
    print(f"wrote {bin_path} ({len(img)} bytes)")
    print(f"wrote {dsm_path}")


if __name__ == "__main__":
    main()