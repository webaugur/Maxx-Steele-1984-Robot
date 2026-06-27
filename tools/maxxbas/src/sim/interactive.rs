//! Live 65C02 firmware runner with direct-wired keypad injection.

use mos6502::cpu::CPU;
use mos6502::instruction::Cmos6502;
use mos6502::memory::Bus;

use super::display::LedDisplay;
use super::keypad::RemoteKey;
use super::patches::{MemPatch, PatchSet};
use super::trace::TraceBuffer;
use crate::CartImage;

const ROM_ED4F: u16 = 0xED4F;

#[derive(Debug, Clone)]
pub struct InteractiveOptions {
    pub cycles_per_frame: u64,
}

/// 455 kHz 6502 stepped at ~60 GUI frames/s ≈ one real-time second per second.
pub const CYCLES_PER_FRAME_REALTIME: u64 = 455_000 / 60;

impl Default for InteractiveOptions {
    fn default() -> Self {
        Self {
            cycles_per_frame: CYCLES_PER_FRAME_REALTIME,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FirmwareStatus {
    pub pc: u16,
    pub cycles: u64,
    pub key_ready: u8,
    pub last_key: u8,
    pub mode: u8,
    pub running: bool,
    /// Sticky: CPU has entered the ROM `$E60D` keypad wait at least once.
    pub keypad_waiting: bool,
    /// MaxxOS answer digit latched at `$35` (`$FF` = none).
    pub answer: u8,
    /// GUI/sim has a key awaiting ROM consumption.
    pub key_pending: bool,
    /// `$35` holds a digit but cart `input_loop` is still waiting (needs ENTER).
    pub needs_enter: bool,
}

/// Heap-backed keypad/IRQ hooks — `LiveBus` holds a raw pointer that must survive `Self` moves.
struct LiveBusState {
    radio_pending: Option<u8>,
    pending_digit: Option<u8>,
    /// GUI digit held until cart `$35` stores it (survives premature `pending_digit` clears).
    latched_digit: Option<u8>,
    irq_pending: bool,
}

struct LiveBus {
    mem_ptr: *mut [u8; 65536],
    state_ptr: *mut LiveBusState,
}

impl LiveBus {
    fn new(mem: &mut [u8; 65536], state: &mut LiveBusState) -> Self {
        Self {
            mem_ptr: std::ptr::from_mut(mem),
            state_ptr: std::ptr::from_mut(state),
        }
    }
}

impl Bus for LiveBus {
    fn get_byte(&mut self, address: u16) -> u8 {
        if address == 0x1000 {
            return 0x80;
        }
        if address == 0x75 {
            unsafe {
                let state = &*self.state_ptr;
                if let Some(k) = state.radio_pending {
                    if k < 0x80 {
                        return k;
                    }
                }
                if let Some(k) = state.pending_digit {
                    if k < 0x20 {
                        return k;
                    }
                }
                if let Some(k) = state.latched_digit {
                    if k < 10 && (*self.mem_ptr)[0x35] >= 0x0A {
                        return k;
                    }
                }
                return (*self.mem_ptr)[0x75];
            }
        }
        unsafe { (*self.mem_ptr)[address as usize] }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        unsafe {
            // Cold start `STY $00,X` clears `$0078`; IRQ stub `JMP ($0078)` would trap at `$0000`.
            if (address == 0x78 || address == 0x79) && value == 0 {
                return;
            }
            // `$E6B2` `STX $15` stores `$80` when the RF wire is idle — must not erase a latched digit.
            if address == 0x15 && value >= 0x80 && (*self.mem_ptr)[0x15] < 0x20 {
                return;
            }
            // ROM clears the RF wire after `$E6AC`; keep the GUI key visible until consumed.
            if address == 0x75 && value >= 0x80 {
                let state = &*self.state_ptr;
                if let Some(k) = state.pending_digit {
                    if k < 0x20 {
                        return;
                    }
                }
                if let Some(k) = state.latched_digit {
                    if k < 10 && (*self.mem_ptr)[0x35] >= 0x0A {
                        return;
                    }
                }
            }
            (*self.mem_ptr)[address as usize] = value;
            // Drop the pending wire key only once firmware copied it into `$15`.
            if address == 0x15 {
                if let Some(k) = (*self.state_ptr).radio_pending {
                    if k == value {
                        (*self.state_ptr).radio_pending = None;
                    }
                }
            }
        }
    }

    fn irq_pending(&mut self) -> bool {
        unsafe {
            let state = &mut *self.state_ptr;
            let pending = state.irq_pending;
            state.irq_pending = false;
            pending
        }
    }
}

pub struct InteractiveFirmware {
    /// Heap-backed so `LiveBus`'s raw pointer stays valid after `Self` is moved.
    mem: Box<[u8; 65536]>,
    display: LedDisplay,
    cpu: CPU<LiveBus, Cmos6502>,
    pub options: InteractiveOptions,
    running: bool,
    pub label: String,
    cart: CartImage,
    irq_phase: u64,
    bus_state: Box<LiveBusState>,
    /// Set once firmware enters `$E60D`/`$E617` wait; stays set until reset.
    keypad_waiting: bool,
    trace: TraceBuffer,
    trace_enabled: bool,
    /// Last mirrored MaxxOS answer digit at `$35`.
    last_answer_digit: u8,
    /// Live GUI: one digit press submits answer (auto-ENTER after `$A1A5`).
    auto_submit_enter: bool,
    queue_auto_enter: bool,
}

fn interactive_patches() -> PatchSet {
    let mut patches = PatchSet::embedded();
    patches.rom_patches.extend([
        MemPatch {
            addr: "0xEDAF".into(),
            bytes: vec![0x60],
            purpose: Some("interactive sim: skip motor stall poll before keypad".into()),
        },
        MemPatch {
            addr: "0xE959".into(),
            bytes: vec![0x60],
            purpose: Some("interactive sim: skip RF scan; keypad wired to $75".into()),
        },
        MemPatch {
            addr: "0xF475".into(),
            bytes: vec![0x60],
            purpose: Some("interactive sim: skip speech phrase setup ($5B)".into()),
        },
        MemPatch {
            addr: "0xF47E".into(),
            bytes: vec![0x60],
            purpose: Some("interactive sim: skip speech busy wait ($5B)".into()),
        },
        MemPatch {
            addr: "0xE3EC".into(),
            bytes: vec![0xA9, 0x00, 0x85, 0x2A, 0x60],
            purpose: Some("interactive sim: instant ROM delay ($2A)".into()),
        },
        MemPatch {
            addr: "0xE9E8".into(),
            bytes: vec![0x60],
            purpose: Some("interactive sim: skip short $2A spin delay".into()),
        },
        MemPatch {
            addr: "0xEF63".into(),
            bytes: vec![0xA9, 0x00, 0x4C, 0x67, 0xEF],
            purpose: Some("interactive sim: skip motor talkback spin at $EF63".into()),
        },
        MemPatch {
            addr: "0xEC1B".into(),
            bytes: vec![0xA9, 0xFF, 0xEA, 0xEA, 0xEA],
            purpose: Some("interactive sim: skip clock display BIT $1000 loop".into()),
        },
        // Embedded patches.json uses opcode $00 (BRK) at these sites — fatal once IRQ runs.
        MemPatch {
            addr: "0xE9E9".into(),
            bytes: vec![0xEA],
            purpose: Some("interactive sim: NOP instead of BRK at $E9E9".into()),
        },
        MemPatch {
            addr: "0xF438".into(),
            bytes: vec![0xEA],
            purpose: Some("interactive sim: NOP instead of BRK at $F438".into()),
        },
        // `$ED4F` display serializer spins on `BIT $1000` with static `0x80` (V clear) forever.
        MemPatch {
            addr: "0xED5F".into(),
            bytes: vec![0x4C, 0x64, 0xED, 0xEA, 0xEA],
            purpose: Some("interactive sim: skip $1000 stall before LED shift".into()),
        },
        MemPatch {
            addr: "0xED6C".into(),
            bytes: vec![0x4C, 0x71, 0xED, 0xEA, 0xEA],
            purpose: Some("interactive sim: skip $1000 stall after LED shift".into()),
        },
        MemPatch {
            addr: "0xED82".into(),
            bytes: vec![0x4C, 0x87, 0xED, 0xEA, 0xEA],
            purpose: Some("interactive sim: skip $1000 stall in ED7B path".into()),
        },
        MemPatch {
            addr: "0xED8C".into(),
            bytes: vec![0x4C, 0x91, 0xED, 0xEA, 0xEA],
            purpose: Some("interactive sim: skip $1000 stall in ED7B path (2)".into()),
        },
        MemPatch {
            addr: "0xEDA0".into(),
            bytes: vec![0x4C, 0xA5, 0xED, 0xEA, 0xEA],
            purpose: Some("interactive sim: skip $1000 stall in ED7B tail".into()),
        },
        // IRQ entry at `$FDC8`: return immediately — full handler nests `RTS` at `$FDDF` → `$FDD9` spin.
        MemPatch {
            addr: "0xFDC8".into(),
            bytes: vec![0x40, 0xEA, 0xEA, 0xEA, 0xEA],
            purpose: Some("interactive sim: IRQ stub RTI at $FDC8".into()),
        },
    ]);
    patches
}

fn new_cpu(mem: &mut [u8; 65536], bus_state: &mut LiveBusState) -> CPU<LiveBus, Cmos6502> {
    bus_state.irq_pending = false;
    let mut cpu = CPU::new(LiveBus::new(mem, bus_state), Cmos6502);
    cpu.reset();
    cpu
}

fn wire_byte(mem: &[u8; 65536], radio: &Option<u8>, pending: Option<u8>) -> u8 {
    if let Some(k) = radio {
        if *k < 0x80 {
            return *k;
        }
    }
    if let Some(k) = pending {
        if k < 0x20 {
            return k;
        }
    }
    mem[0x75]
}

/// Mask stale ZP wire/latch bytes once execution has left cart `input_loop` / ROM poll.
fn displayed_keypad_wire(
    mem: &[u8; 65536],
    radio: &Option<u8>,
    pending: Option<u8>,
    pc: u16,
    in_poll: bool,
    key_pending: bool,
) -> (u8, u8) {
    let in_input = key_pending
        || in_poll
        || (0xA080..=0xA1FF).contains(&pc)
        || in_keypad_subsystem(pc);
    if in_input {
        return (wire_byte(mem, radio, pending), mem[0x15]);
    }
    let wire = 0x80;
    let latch = if mem[0x15] < 0x20 { 0x80 } else { mem[0x15] };
    (wire, latch)
}

fn apply_radio_wire(mem: &mut [u8; 65536], radio: &Option<u8>, pending: Option<u8>) {
    if let Some(k) = radio {
        if *k < 0x80 {
            mem[0x75] = *k;
        }
        return;
    }
    // GUI key still pending — ROM may have cleared `$15`/`$75` on the prior poll lap.
    if let Some(k) = pending {
        if k < 0x20 {
            mem[0x75] = k;
            return;
        }
    }
    if mem[0x75] < 0x80 {
        // Leave the key visible for `$E6AC` until `$E6A4` clears bit 7 at `$E6B0`.
        if latched_key(mem).is_some() {
            return;
        }
        // ROM sometimes leaves `$00` on the wire; treat as idle, not key `U`.
        mem[0x75] = 0x80;
    }
}



/// True when the CPU is in `$E617` spin-waiting for a key on `$75` (after `$E6A4` returned X=`#$80`).
fn in_keypad_spin(pc: u16) -> bool {
    (0xE617..=0xE642).contains(&pc)
}

/// Inside `$E6A4`..=`$E6B4` (`LDX $75` / `STX $15`).
fn in_keypad_latch(pc: u16) -> bool {
    (0xE6A4..=0xE6B4).contains(&pc)
}

/// Patched RF scan stubs reached from `$E6A7` while `$E617` poll is active.
fn in_keypad_rf_stub(pc: u16) -> bool {
    matches!(pc, 0xEDAF | 0xE959)
}

/// ROM keypad / RF / display path through `$EAFF` (includes patched `$E959` / `$EDAF` stubs).
fn in_keypad_subsystem(pc: u16) -> bool {
    (0xE60D..=0xEAFF).contains(&pc)
}

/// Bytes pushed by 6502 `JSR` (address of next instruction minus one).
const ROM_E60D_JSR_E617_RET: u16 = 0xE60F;
const ROM_E617_JSR_E6A4_RET: u16 = 0xE619;
const CART_JSR_E60D_RET: u16 = 0xA198;
/// First instruction in cart `input_loop` after `JSR $E60D` returns.
const CART_INPUT_LOOP_DISPATCH: u16 = 0xA199;
/// MaxxOS `input_done` — ENTER accepted (`$A1B7`).
const CART_INPUT_DONE: u16 = 0xA1B7;
/// MaxxOS main quiz flow: return after `JSR input_answer` (`$A08F` → `$A092`).
const CART_AFTER_INPUT_ANSWER: u16 = 0xA092;

fn cart_return_after_jsr(ret: u16) -> u16 {
    ret.wrapping_add(1)
}

fn scan_stack_for_cart_return(mem: &[u8; 65536], sp: u8) -> Option<u16> {
    for depth in 0..12 {
        let raw = stack_jsr_return_raw(mem, sp.wrapping_add(depth * 2));
        if (0xA000..=0xAFFF).contains(&raw) {
            return Some(cart_return_after_jsr(raw));
        }
    }
    None
}

/// Uninitialized cart RAM (`$8000`..=`$81FF`) is all `$00` (`BRK`) — IRQ/BRK storms if entered.
fn recover_from_brk_sled(mem: &[u8; 65536], sp: u8, pc: u16) -> Option<u16> {
    if !(0x8000..=0x81FF).contains(&pc) || mem[pc as usize] != 0 {
        return None;
    }
    scan_stack_for_cart_return(mem, sp).or(Some(CART_AFTER_INPUT_ANSWER))
}

fn stack_jsr_return_raw(mem: &[u8; 65536], sp: u8) -> u16 {
    let lo = mem[0x0100usize + usize::from(sp.wrapping_add(1))];
    let hi = mem[0x0100usize + usize::from(sp.wrapping_add(2))];
    u16::from_le_bytes([lo, hi])
}

/// ROM LED serializer (`$ED48`..=`$ED7F`) — not keypad input; must not hijack to `$A199`.
fn in_rom_led_display(pc: u16) -> bool {
    (0xED48..=0xED7F).contains(&pc)
}

/// Poll loop, latch path, or patched RF stub sites used for wire injection.
fn in_keypad_read_path(_mem: &[u8; 65536], _sp: u8, pc: u16) -> bool {
    if in_rom_led_display(pc) {
        return false;
    }
    in_keypad_spin(pc)
        || in_keypad_latch(pc)
        || in_keypad_rf_stub(pc)
        || (0xE60D..=0xE616).contains(&pc)
}

/// True when cart `JSR $E60D` at `$A196` is still waiting on the stack.
fn stack_has_cart_e60d_return(mem: &[u8; 65536], sp: u8) -> bool {
    for depth in 0..6 {
        if stack_jsr_return_raw(mem, sp.wrapping_add(depth * 2)) == CART_JSR_E60D_RET {
            return true;
        }
    }
    false
}

fn note_keypad_wait(pc: u16, keypad_waiting: &mut bool) {
    if in_keypad_subsystem(pc) || (0xA080..=0xA200).contains(&pc) {
        *keypad_waiting = true;
    }
}

fn infer_keypad_ready(cycles: u64, pc: u16, led: &str) -> bool {
    if led.chars().any(|c| c.is_ascii_digit() || c == '?') {
        return true;
    }
    if in_keypad_subsystem(pc)
        || in_keypad_spin(pc)
        || in_keypad_latch(pc)
        || in_keypad_rf_stub(pc)
    {
        return true;
    }
    // MaxxOS boot + first prompt normally finishes well before this.
    cycles > 48_000 && pc >= 0xE000
}

/// Valid latched key in `$15` (distinguish digit `$00` from cold-boot zero / idle).
fn latched_key(mem: &[u8; 65536]) -> Option<u8> {
    let k = mem[0x15];
    if k >= 0x20 {
        return None;
    }
    if k == 0 && mem[0x75] >= 0x80 {
        return None;
    }
    Some(k)
}

/// Copy a pending wire key into `$15` while the ROM keypad path is active.
fn deliver_pending_key(
    mem: &mut [u8; 65536],
    radio: &mut Option<u8>,
    sp: u8,
    pc: u16,
    index_x: &mut u8,
    keypad_waiting: bool,
) {
    let key = match radio {
        Some(k) if *k < 0x80 => *k,
        _ => return,
    };
    if in_keypad_read_path(mem, sp, pc) {
        // Active ROM keypad read — always mirror the wire.
    } else if stack_has_cart_e60d_return(mem, sp) && keypad_waiting {
        // Cart `JSR $E60D` still on stack — latch before the CPU re-enters ROM poll.
    } else {
        return;
    }

    mem[0x15] = key;
    // `$E6AC` `LDX $75` — keep the key on the wire until `$E6A4` consumes it.
    mem[0x75] = key;

    if in_keypad_read_path(mem, sp, pc) {
        *radio = None;
        *index_x = key;
    }
}

/// Re-drive `$75` when the GUI latched a digit into `$15` but ROM cleared the wire (`$80`).
fn represent_pending_digit_on_wire(
    mem: &mut [u8; 65536],
    sp: u8,
    pc: u16,
    pending: Option<u8>,
) {
    let Some(key) = pending else {
        return;
    };
    if key >= 0x20 {
        return;
    }
    if !in_keypad_read_path(mem, sp, pc) {
        return;
    }
    if mem[0x75] >= 0x80 {
        mem[0x75] = key;
    }
}

/// Inject a pending GUI key onto `$75` right before ROM reads it.
fn inject_pending_on_wire(mem: &mut [u8; 65536], sp: u8, pc: u16, pending: Option<u8>) {
    if !matches!(
        pc,
        0xE617 | 0xE61A | 0xE6A4 | 0xE6A7 | 0xE6AC | 0xEDAF | 0xE959
    ) && !in_keypad_read_path(mem, sp, pc)
    {
        return;
    }
    let Some(key) = pending else {
        return;
    };
    if key >= 0x20 {
        return;
    }
    if mem[0x75] >= 0x80 {
        mem[0x75] = key;
    }
}

fn pop_stack_word(cpu: &mut CPU<LiveBus, Cmos6502>) {
    cpu.registers.stack_pointer.0 = cpu.registers.stack_pointer.0.wrapping_add(2);
}

/// Pop nested ROM keypad `JSR` frames until the cart `$E60D` return (`$A199`) is consumed.
fn unwind_keypad_stack_to_cart(cpu: &mut CPU<LiveBus, Cmos6502>, mem: &[u8; 65536]) -> bool {
    let mut found_cart = false;
    loop {
        let ret = stack_jsr_return_raw(mem, cpu.registers.stack_pointer.0);
        match ret {
            ROM_E617_JSR_E6A4_RET | ROM_E60D_JSR_E617_RET => pop_stack_word(cpu),
            CART_JSR_E60D_RET => {
                pop_stack_word(cpu);
                found_cart = true;
                break;
            }
            _ => break,
        }
    }
    found_cart
}

/// GUI digit waiting in `$15` — unwind keypad poll and return to cart `input_loop` at `$A199`.
fn finish_e60d_keypad_wait(
    mem: &mut [u8; 65536],
    pc: u16,
    pending: Option<u8>,
    cpu: &mut CPU<LiveBus, Cmos6502>,
) -> bool {
    let Some(key) = pending else {
        return false;
    };
    if key >= 0x20 {
        return false;
    }
    let sp = cpu.registers.stack_pointer.0;
    if !in_keypad_read_path(mem, sp, pc) {
        return false;
    }
    if !stack_has_cart_e60d_return(mem, sp) {
        return false;
    }
    if !unwind_keypad_stack_to_cart(cpu, mem) {
        return false;
    }
    mem[0x15] = key;
    mem[0x75] = 0x80;
    cpu.registers.program_counter = CART_INPUT_LOOP_DISPATCH;
    cpu.registers.accumulator = key.into();
    cpu.registers.index_x = key;
    true
}

/// Digit already in `$15` — skip `$E617` `JSR $E6A4` and enter post-poll dispatch at `$E61A`.
fn skip_e617_jsr_if_pending(
    mem: &[u8; 65536],
    pc: u16,
    pending: Option<u8>,
    cpu: &mut CPU<LiveBus, Cmos6502>,
) -> bool {
    if pc != 0xE617 {
        return false;
    }
    let Some(key) = pending else {
        return false;
    };
    if key >= 0x20 {
        return false;
    }
    if stack_jsr_return_raw(mem, cpu.registers.stack_pointer.0) != ROM_E60D_JSR_E617_RET {
        return false;
    }
    cpu.registers.program_counter = 0xE61A;
    cpu.registers.index_x = key;
    true
}

/// `$E622` `RTS` must land at `$E610`; repair stack when the sim skipped the poll `JSR`.
fn complete_e617_rts_to_e610(
    mem: &[u8; 65536],
    pc: u16,
    pending: Option<u8>,
    cpu: &mut CPU<LiveBus, Cmos6502>,
) -> bool {
    if pc != 0xE622 {
        return false;
    }
    let Some(key) = pending else {
        return false;
    };
    if key >= 0x20 {
        return false;
    }
    if stack_jsr_return_raw(mem, cpu.registers.stack_pointer.0) == ROM_E617_JSR_E6A4_RET {
        pop_stack_word(cpu);
    }
    cpu.registers.program_counter = 0xE610;
    cpu.registers.accumulator = key.into();
    cpu.registers.index_x = key;
    true
}

/// `$E616` `RTS` must land in cart `input_loop` at `$A199`.
fn complete_e60d_rts_to_cart(
    mem: &[u8; 65536],
    pc: u16,
    pending: Option<u8>,
    cpu: &mut CPU<LiveBus, Cmos6502>,
) -> bool {
    if pc != 0xE616 {
        return false;
    }
    let Some(key) = pending else {
        return false;
    };
    if key >= 0x20 {
        return false;
    }
    let sp = cpu.registers.stack_pointer.0;
    if stack_jsr_return_raw(mem, sp) == CART_JSR_E60D_RET {
        pop_stack_word(cpu);
    }
    cpu.registers.program_counter = CART_INPUT_LOOP_DISPATCH;
    cpu.registers.accumulator = key.into();
    true
}

/// Skip `LDX $75` at `$E6AC` when the GUI already latched a digit (avoids stale bus reads).
fn bypass_ldx75_when_pending(
    mem: &mut [u8; 65536],
    pending: Option<u8>,
    cpu: &mut CPU<LiveBus, Cmos6502>,
) -> bool {
    if cpu.registers.program_counter != 0xE6AC {
        return false;
    }
    let Some(key) = pending else {
        return false;
    };
    if key >= 0x20 {
        return false;
    }
    mem[0x75] = key;
    mem[0x15] = key;
    cpu.registers.index_x = key;
    cpu.registers.program_counter = 0xE6AE;
    true
}

/// While a GUI key is pending, mirror it into X for post-poll dispatch at `$E61A`.
fn sync_keypad_x_from_latch(
    _mem: &[u8; 65536],
    pc: u16,
    pending: Option<u8>,
    index_x: &mut u8,
) {
    let Some(key) = pending else {
        return;
    };
    if key >= 0x20 || !(0xE60D..=0xEAFF).contains(&pc) {
        return;
    }
    *index_x = key;
}

/// Approximate IRQ-side timer decrements so delays and speech paths make progress.
fn tick_irq_services(mem: &mut [u8; 65536], irq_pending: &mut bool) {
    mem[0x3A] = mem[0x3A].wrapping_add(1);
    for zp in [0x2A_u16, 0x28, 0x27] {
        let v = mem[zp as usize];
        if v != 0 {
            mem[zp as usize] = v - 1;
            if zp == 0x28 && mem[0x28] == 0 {
                *irq_pending = true;
            }
        }
    }
    let speech = mem[0x5B];
    if speech != 0 {
        mem[0x5B] = speech.saturating_sub(2);
    }
}

impl InteractiveFirmware {
    pub fn new(cart: CartImage, label: impl Into<String>) -> Result<Self, String> {
        let patches = interactive_patches();
        let mut mem = Box::new(super::firmware::build_memory_image(Some(&cart), &patches)?);
        super::firmware::prepare_interactive_memory(mem.as_mut(), &cart);

        let display = LedDisplay::default();
        let mut bus_state = Box::new(LiveBusState {
            radio_pending: None,
            pending_digit: None,
            latched_digit: None,
            irq_pending: false,
        });
        let cpu = new_cpu(mem.as_mut(), bus_state.as_mut());

        Ok(Self {
            mem,
            display,
            cpu,
            options: InteractiveOptions::default(),
            running: true,
            label: label.into(),
            cart,
            irq_phase: 0,
            bus_state,
            keypad_waiting: false,
            trace: TraceBuffer::default(),
            trace_enabled: true,
            last_answer_digit: 0xFF,
            auto_submit_enter: false,
            queue_auto_enter: false,
        })
    }

    pub fn reset(&mut self) -> Result<(), String> {
        let patches = interactive_patches();
        *self.mem = super::firmware::build_memory_image(Some(&self.cart), &patches)?;
        super::firmware::prepare_interactive_memory(self.mem.as_mut(), &self.cart);
        self.display = LedDisplay::default();
        self.bus_state.radio_pending = None;
        self.bus_state.pending_digit = None;
        self.bus_state.latched_digit = None;
        self.keypad_waiting = false;
        self.trace.clear();
        self.cpu = new_cpu(self.mem.as_mut(), self.bus_state.as_mut());
        self.running = true;
        self.irq_phase = 0;
        self.last_answer_digit = 0xFF;
        self.queue_auto_enter = false;
        Ok(())
    }

    /// Live GUI: digit keys auto-chain ENTER so one click submits the MaxxOS answer.
    pub fn set_auto_submit_enter(&mut self, enabled: bool) {
        self.auto_submit_enter = enabled;
    }

    /// True while the CPU is in the ROM `$E617` keypad poll loop (including `$E959` callees).
    pub fn in_keypad_poll(&self) -> bool {
        let pc = self.cpu.registers.program_counter;
        let sp = self.cpu.registers.stack_pointer.0;
        in_keypad_read_path(&self.mem, sp, pc)
    }

    fn effective_keypad_waiting(&self) -> bool {
        let pc = self.cpu.registers.program_counter;
        let led = self.led_chars();
        self.keypad_waiting || infer_keypad_ready(self.cpu.cycles, pc, &led)
    }

    /// Wire key for ROM poll: `pending_digit`, else sticky GUI latch while `$35` is still empty.
    fn effective_pending_key(&self) -> Option<u8> {
        if let Some(k) = self.bus_state.pending_digit {
            if k < 0x20 {
                return Some(k);
            }
        }
        if let Some(k) = self.bus_state.latched_digit {
            if k < 10 && self.mem[0x35] >= 0x0A {
                return Some(k);
            }
        }
        None
    }

    fn clear_latched_if_answer_stored(&mut self) {
        if let Some(d) = self.bus_state.latched_digit {
            if self.mem[0x35] == d {
                self.bus_state.latched_digit = None;
            }
        }
    }

    /// Press a remote key — RF wire presents keycode at `$75` (bit 7 clear).
    pub fn press_key(&mut self, key: RemoteKey) {
        let code = key.keycode();
        self.bus_state.radio_pending = Some(code);
        self.mem[0x75] = code;
        let pc = self.cpu.registers.program_counter;
        let sp = self.cpu.registers.stack_pointer.0;
        let mut x = self.cpu.registers.index_x;
        let keypad_active = self.in_keypad_poll();
        let waiting = self.effective_keypad_waiting() || keypad_active;
        if waiting {
            self.mem[0x15] = code;
            self.keypad_waiting = true;
        }
        deliver_pending_key(
            &mut self.mem,
            &mut self.bus_state.radio_pending,
            sp,
            pc,
            &mut x,
            waiting,
        );
        self.cpu.registers.index_x = x;
        self.queue_auto_enter = false;
        // MaxxOS input_loop also accepts CLEAR (`$0E`) and ENTER (`$0F`) after digits.
        if code < 10 {
            self.bus_state.pending_digit = Some(code);
            self.bus_state.latched_digit = Some(code);
            self.mem[0x15] = code;
            self.display = LedDisplay::default();
            self.display.mirror_answer_digit(&self.mem, code);
            self.last_answer_digit = code;
            if self.auto_submit_enter {
                self.queue_auto_enter = true;
            }
        } else if code == 0x0E || code == 0x0F {
            self.bus_state.pending_digit = Some(code);
            self.mem[0x15] = code;
            self.mem[0x75] = code;
        }
        self.try_apply_pending_key_now();
    }

    /// Synchronous keypad injection for the GUI (do not wait for the next CPU step).
    fn try_apply_pending_key_now(&mut self) {
        let pending = self.effective_pending_key();
        if pending.is_none() {
            return;
        }
        let pc = self.cpu.registers.program_counter;
        if !self.in_keypad_poll() {
            return;
        }
        if finish_e60d_keypad_wait(
            &mut self.mem,
            pc,
            pending,
            &mut self.cpu,
        ) {
            self.bus_state.radio_pending = None;
            if pending == Some(0x0F) {
                self.bus_state.pending_digit = None;
                self.queue_auto_enter = false;
                self.mem[0x15] = 0x80;
                self.mem[0x75] = 0x80;
            }
            if self.trace_enabled {
                let key = pending.unwrap_or(self.mem[0x15]);
                self.trace.record(
                    &self.mem,
                    CART_INPUT_LOOP_DISPATCH,
                    key,
                    key,
                    self.cpu.registers.index_y,
                );
            }
        }
    }

    /// Run the CPU until a GUI keypress is consumed or `max_frames` is reached.
    pub fn digest_keypress(&mut self, max_frames: u32) {
        let was_running = self.running;
        self.running = true;
        for _ in 0..max_frames {
            self.step_frame();
            if self.effective_pending_key().is_none()
                && !self.queue_auto_enter
                && !self.in_keypad_poll()
            {
                break;
            }
        }
        self.running = was_running;
    }

    pub fn led_chars(&self) -> String {
        self.display.pair()
    }

    fn capture_display_digit(&mut self, pc: u16) {
        if pc != ROM_ED4F {
            return;
        }
        // Keep the sim-mirrored answer digits; ROM `$ED4F` noise would overwrite them.
        if self.last_answer_digit != 0xFF {
            return;
        }
        let seg = u8::from(self.cpu.registers.accumulator);
        self.display.push_segment(seg);
    }

    pub fn step(&mut self, cycles: u64) {
        if !self.running {
            return;
        }
        let limit = self.cpu.cycles + cycles;
        while self.cpu.cycles < limit {
            super::firmware::ensure_irq_vectors(&mut self.mem);
            self.irq_phase = self.irq_phase.wrapping_add(1);
            if self.irq_phase % 64 == 0 {
                tick_irq_services(&mut self.mem, &mut self.bus_state.irq_pending);
            }

            let effective = self.effective_pending_key();
            apply_radio_wire(&mut self.mem, &self.bus_state.radio_pending, effective);
            let mut pc = self.cpu.registers.program_counter;
            let sp = self.cpu.registers.stack_pointer.0;
            if let Some(recover) = recover_from_brk_sled(&self.mem, sp, pc) {
                self.cpu.registers.program_counter = recover;
                pc = recover;
            }
            represent_pending_digit_on_wire(&mut self.mem, sp, pc, effective);
            if pc == 0 {
                super::firmware::bootstrap_irq_vectors(&mut self.mem);
                pc = super::firmware::ROM_KEYPAD_POLL;
                self.cpu.registers.program_counter = pc;
            }
            self.capture_display_digit(pc);
            note_keypad_wait(pc, &mut self.keypad_waiting);
            let mut x = self.cpu.registers.index_x;
            let keypad_waiting = self.keypad_waiting
                || infer_keypad_ready(self.cpu.cycles, pc, &self.led_chars());
            deliver_pending_key(
                &mut self.mem,
                &mut self.bus_state.radio_pending,
                sp,
                pc,
                &mut x,
                keypad_waiting,
            );
            sync_keypad_x_from_latch(&self.mem, pc, effective, &mut x);
            self.cpu.registers.index_x = x;
            inject_pending_on_wire(&mut self.mem, sp, pc, effective);
            if finish_e60d_keypad_wait(
                &mut self.mem,
                pc,
                effective,
                &mut self.cpu,
            ) {
                let key = effective.unwrap_or(self.mem[0x15]);
                self.bus_state.radio_pending = None;
                if key == 0x0F {
                    self.bus_state.pending_digit = None;
                    self.queue_auto_enter = false;
                    self.mem[0x15] = 0x80;
                    self.mem[0x75] = 0x80;
                }
                if self.trace_enabled {
                    self.trace.record(
                        &self.mem,
                        CART_INPUT_LOOP_DISPATCH,
                        key,
                        key,
                        self.cpu.registers.index_y,
                    );
                }
                continue;
            }
            if skip_e617_jsr_if_pending(&self.mem, pc, effective, &mut self.cpu) {
                continue;
            }
            pc = self.cpu.registers.program_counter;
            if complete_e617_rts_to_e610(&self.mem, pc, effective, &mut self.cpu) {
                continue;
            }
            if complete_e60d_rts_to_cart(&self.mem, pc, effective, &mut self.cpu) {
                continue;
            }
            if bypass_ldx75_when_pending(&mut self.mem, effective, &mut self.cpu) {
                continue;
            }
            let pc_before = self.cpu.registers.program_counter;
            if self.trace_enabled {
                self.trace.record(
                    &self.mem,
                    pc_before,
                    u8::from(self.cpu.registers.accumulator),
                    self.cpu.registers.index_x,
                    self.cpu.registers.index_y,
                );
            }
            if !self.cpu.single_step() {
                self.running = false;
                break;
            }
            // Release GUI pending state once cart `input_loop` dispatches the key.
            if pc_before == 0xA1A5 {
                self.bus_state.pending_digit = None;
                let chain_enter = self.queue_auto_enter || self.auto_submit_enter;
                self.queue_auto_enter = false;
                if chain_enter {
                    self.bus_state.pending_digit = Some(0x0F);
                    self.bus_state.radio_pending = Some(0x0F);
                    self.mem[0x15] = 0x0F;
                    self.mem[0x75] = 0x0F;
                }
            }
            if pc_before == 0xE196 && self.bus_state.pending_digit.is_none() && !self.queue_auto_enter {
                // New `input_loop` wait — drop stale GUI-mirrored digits from the prior answer.
                self.last_answer_digit = 0xFF;
            }
            if pc_before == 0xA19F && self.cpu.registers.program_counter == 0xA1B7 {
                // ENTER accepted — do not leave a stale `$A199` stack frame for `input_done` `RTS`.
                self.bus_state.pending_digit = None;
                self.queue_auto_enter = false;
                self.bus_state.radio_pending = None;
                self.mem[0x15] = 0x80;
                self.mem[0x75] = 0x80;
            }
            if matches!(pc_before, 0xA1AD | 0xA1B7 | 0xA1BD) {
                self.bus_state.pending_digit = None;
                self.queue_auto_enter = false;
                self.bus_state.radio_pending = None;
                self.keypad_waiting = false;
                self.last_answer_digit = 0xFF;
                self.mem[0x15] = 0x80;
                self.mem[0x75] = 0x80;
            }
            // `$E6AC` may have read the wire this instruction; ensure `$15` caught it.
            if pc_before == 0xE6AC {
                let wire_key = self.effective_pending_key().or(self.bus_state.radio_pending);
                if let Some(k) = wire_key {
                    if k < 0x20 {
                        self.mem[0x15] = k;
                        self.mem[0x75] = k;
                    }
                }
            }
            self.clear_latched_if_answer_stored();
        }
    }

    pub fn step_frame(&mut self) {
        let n = self.options.cycles_per_frame;
        self.step(n);
    }

    /// Run firmware ahead of the first GUI paint so boot/prompt state is ready immediately.
    pub fn warmup(&mut self, frames: u64) {
        for _ in 0..frames {
            self.step_frame();
        }
        let pc = self.cpu.registers.program_counter;
        let led = self.led_chars();
        if infer_keypad_ready(self.cpu.cycles, pc, &led) {
            self.keypad_waiting = true;
        }
    }

    pub fn status(&self) -> FirmwareStatus {
        let pc = self.cpu.registers.program_counter;
        let led = self.led_chars();
        let keypad_waiting =
            self.keypad_waiting || infer_keypad_ready(self.cpu.cycles, pc, &led);
        let answer = self.mem[0x35];
        let effective = self.effective_pending_key();
        let needs_enter = answer < 0x0A && self.in_keypad_poll() && effective.is_none();
        let key_pending = effective.is_some() || self.queue_auto_enter;
        let (key_ready, last_key) = displayed_keypad_wire(
            &self.mem,
            &self.bus_state.radio_pending,
            effective,
            pc,
            self.in_keypad_poll(),
            key_pending,
        );
        FirmwareStatus {
            pc,
            cycles: self.cpu.cycles,
            key_ready,
            last_key,
            mode: self.mem[0x0D],
            running: self.running,
            keypad_waiting,
            answer,
            key_pending,
            needs_enter,
        }
    }

    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    pub fn set_trace_enabled(&mut self, enabled: bool) {
        self.trace_enabled = enabled;
    }

    pub fn trace_enabled(&self) -> bool {
        self.trace_enabled
    }

    pub fn clear_trace(&mut self) {
        self.trace.clear();
    }

    /// Recent instructions (newest at bottom), ready to copy/paste.
    pub fn trace_text(&self) -> String {
        let st = self.status();
        let irq_vec = super::firmware::irq_vector(&self.mem);
        let pending = self
            .bus_state
            .pending_digit
            .map(|k| format!("{k:02X}"))
            .unwrap_or_else(|| "--".into());
        let latched = self
            .bus_state
            .latched_digit
            .map(|k| format!("{k:02X}"))
            .unwrap_or_else(|| "--".into());
        let mut header = format!(
            "; sim {} | PC=${:04X} | $78=${:04X} | LED=[{}] | $75=${:02X} $15=${:02X} | pending={pending} latched={latched} $35=${:02X} | cycles={}\n",
            env!("CARGO_PKG_VERSION"),
            st.pc,
            irq_vec,
            self.led_chars(),
            st.key_ready,
            st.last_key,
            self.mem[0x35],
            st.cycles
        );
        header.push_str(&self.trace.format_lines(&self.mem));
        header
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CartImage;

    const MAXXOS: &[u8] = include_bytes!(
        "../../../../Cartridge/Examples/MaxxOS/Firmware/Binary/MaxxOS.532"
    );

    #[test]
    fn maxxos_led_shows_prompt_eventually() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        for frame in 0..3000 {
            fw.step_frame();
            let led = fw.led_chars();
            if led.contains('?') || led.chars().any(|c| c.is_ascii_digit()) {
                return;
            }
            if frame == 2999 {
                panic!(
                    "LED never showed a problem (led={} pc=${:04X} waiting={})",
                    led,
                    fw.status().pc,
                    fw.keypad_waiting
                );
            }
        }
    }

    #[test]
    fn maxxos_led_within_two_seconds() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.options.cycles_per_frame = 16_000;
        for _ in 0..120 {
            fw.step_frame();
        }
        let led = fw.led_chars();
        assert!(
            led.contains('?') || led.chars().any(|c| c.is_ascii_digit()),
            "expected digits within ~2s (led={} pc=${:04X})",
            led,
            fw.status().pc
        );
    }

    #[test]
    fn survives_forced_cold_boot_zp_zero() {
        use crate::sim::firmware::{irq_vector, IRQ_VECTOR_FDC8};
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.mem[0x0100..0x0111].fill(0);
        fw.cpu.reset();
        for _ in 0..4000 {
            fw.step_frame();
            assert_ne!(
                fw.status().pc,
                0,
                "PC=$0000 after cold boot (irq=${:04X})",
                irq_vector(&fw.mem)
            );
            assert_eq!(
                irq_vector(&fw.mem),
                IRQ_VECTOR_FDC8,
                "$78 cleared during cold boot"
            );
        }
    }

    #[test]
    fn zp_irq_vector_initialized_at_boot() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        let irq = u16::from_le_bytes([fw.mem[0x78], fw.mem[0x79]]);
        assert_eq!(irq, 0xFDC8, "IRQ vector at $78 should be $FDC8, got ${irq:04X}");
    }

    #[test]
    fn repro_user_16m_cycles_with_key7() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        fw.press_key(RemoteKey::Arms7);
        let target = 16_194_023u64;
        while fw.status().cycles < target {
            fw.step_frame();
            if fw.status().pc == 0 {
                panic!(
                    "PC=0 with key7 irq=${:04X}",
                    u16::from_le_bytes([fw.mem[0x78], fw.mem[0x79]])
                );
            }
        }
    }

    #[test]
    fn repro_user_16m_cycles() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        let target = 16_194_023u64;
        while fw.status().cycles < target {
            fw.step_frame();
            let pc = fw.status().pc;
            if pc == 0 {
                let irq = u16::from_le_bytes([fw.mem[0x78], fw.mem[0x79]]);
                panic!(
                    "PC=0 at cycles={} irq_vec=${irq:04X} $72=${:02X}",
                    fw.status().cycles,
                    fw.mem[0x72]
                );
            }
        }
        let irq = u16::from_le_bytes([fw.mem[0x78], fw.mem[0x79]]);
        eprintln!(
            "done pc=${:04X} irq=${irq:04X} led={}",
            fw.status().pc,
            fw.led_chars()
        );
    }

    #[test]
    fn cpu_never_stuck_at_zero_after_warmup() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        for _ in 0..500 {
            fw.step_frame();
        }
        assert_ne!(
            fw.status().pc,
            0,
            "CPU stuck at $0000 (irq vec ${:04X})",
            u16::from_le_bytes([fw.mem[0x78], fw.mem[0x79]])
        );
    }

    #[test]
    fn zp_irq_vector_initialized_after_warmup() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        let irq = u16::from_le_bytes([fw.mem[0x78], fw.mem[0x79]]);
        assert_eq!(
            irq, 0xFDC8,
            "IRQ vector at $78 should be $FDC8 after boot, got ${irq:04X} (pc=${:04X})",
            fw.status().pc
        );
    }

    #[test]
    fn warmup_leaves_keypad_ready() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        let st = fw.status();
        assert!(
            st.keypad_waiting,
            "warmup should reach keypad (pc=${:04X} led={})",
            st.pc,
            fw.led_chars()
        );
        assert!(
            fw.led_chars().contains('?') || fw.led_chars().chars().any(|c| c.is_ascii_digit()),
            "warmup should show LED prompt (led={})",
            fw.led_chars()
        );
    }

    #[test]
    fn trace_boot_startup() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        for frame in [1_u64, 2, 5, 10, 20, 30, 60, 120, 240, 500, 1000] {
            let target = frame * fw.options.cycles_per_frame;
            while fw.status().cycles < target {
                fw.step_frame();
            }
            let st = fw.status();
            eprintln!(
                "frame~{frame}: pc=${:04X} led={} waiting={} mode=${:02X}",
                st.pc,
                fw.led_chars(),
                st.keypad_waiting,
                st.mode
            );
        }
    }

    #[test]
    fn diagnose_maxxos_run_state() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        let mut in_keypad = 0u64;
        let mut in_cart_input = 0u64;
        let mut in_cart = 0u64;
        for frame in 0..15_000 {
            fw.step_frame();
            let pc = fw.status().pc;
            if (0xE617..=0xE6B4).contains(&pc) {
                in_keypad += 1;
            }
            if (0xA194..=0xA1BB).contains(&pc) {
                in_cart_input += 1;
            }
            if (0xA080..=0xA200).contains(&pc) {
                in_cart += 1;
            }
            if frame == 14_999 {
                eprintln!(
                    "frame {frame}: pc=${pc:04X} led={} $75=${:02X} $15=${:02X} \
                     waiting={} keypad_frames={in_keypad} cart_input_frames={in_cart_input} cart_frames={in_cart}",
                    fw.led_chars(),
                    fw.mem[0x75],
                    fw.mem[0x15],
                    fw.keypad_waiting
                );
            }
        }
        fw.press_key(RemoteKey::Wrist5);
        eprintln!(
            "after press at pc=${:04X}: $75=${:02X} $15=${:02X}",
            fw.status().pc,
            fw.mem[0x75],
            fw.mem[0x15]
        );
        for _ in 0..200 {
            fw.step_frame();
        }
        eprintln!(
            "after 200 frames: pc=${:04X} $75=${:02X} $15=${:02X} led={}",
            fw.status().pc,
            fw.mem[0x75],
            fw.mem[0x15],
            fw.led_chars()
        );
    }

    #[test]
    fn maxxos_reaches_cart_code() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        for _ in 0..200 {
            fw.step_frame();
        }
        let st = fw.status();
        assert!(st.pc >= 0xA080, "expected cart code, got ${:04X}", st.pc);
        assert!(st.cycles > 10_000);
    }

    #[test]
    fn interactive_patches_applied() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        assert_eq!(fw.mem[0xEDAF], 0x60, "EDAF motor stall");
        assert_eq!(fw.mem[0xE959], 0x60, "E959 RF scan");
        assert_eq!(fw.mem[0xF475], 0x60, "F475 speech");
        assert_eq!(fw.mem[0xF47E], 0x60, "F47E speech wait");
        assert_eq!(fw.mem[0xED5F], 0x4C, "ED5F display handshake");
    }

    #[test]
    fn press_key_writes_radio_wire() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.mem[0x75] = 0x80;
        fw.bus_state.radio_pending = None;
        fw.press_key(RemoteKey::Wrist5);
        assert_eq!(fw.bus_state.radio_pending, Some(5));
        assert_eq!(fw.mem[0x75], 5);
    }

    #[test]
    fn pending_digit_holds_wire_after_rom_clears_latch() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.bus_state.pending_digit = Some(5);
        fw.mem[0x75] = 0x80;
        fw.mem[0x15] = 0x80;
        apply_radio_wire(&mut fw.mem, &fw.bus_state.radio_pending, fw.bus_state.pending_digit);
        assert_eq!(fw.mem[0x75], 5, "GUI pending key must stay on $75 for LDX $75");
    }

    /// Exact live GUI flow: warmup, poll, press 6, digest 160 frames.
    #[test]
    fn live_gui_digest_arms6_leaves_poll() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.set_auto_submit_enter(true);
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        assert!(in_keypad_spin(fw.status().pc), "never reached poll");
        fw.press_key(RemoteKey::Arms6);
        fw.digest_keypress(160);
        assert!(
            fw.mem[0x35] == 6 || !in_keypad_spin(fw.status().pc),
            "gui digest stuck (pc=${:04X} $35={} pending={:?} queue={} $75=${:02X})",
            fw.status().pc,
            fw.mem[0x35],
            fw.bus_state.pending_digit,
            fw.queue_auto_enter,
            fw.mem[0x75]
        );
    }

    #[test]
    fn bypass_ldx75_when_pending_skips_idle_read() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.bus_state.pending_digit = Some(6);
        fw.mem[0x75] = 0x80;
        fw.cpu.registers.program_counter = 0xE6AC;
        fw.step(1);
        assert_eq!(fw.cpu.registers.index_x, 6);
        assert_ne!(fw.cpu.registers.program_counter, 0xE6AC);
        assert_eq!(fw.mem[0x15], 6);
    }

    /// Sticky `latched_digit` survives premature `pending_digit` clears (live GUI frame order).
    #[test]
    fn latched_digit_bypasses_ldx75_when_pending_cleared() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.bus_state.pending_digit = None;
        fw.bus_state.latched_digit = Some(6);
        fw.mem[0x35] = 0xFF;
        fw.mem[0x75] = 0x80;
        fw.mem[0x15] = 0x80;
        fw.cpu.registers.program_counter = 0xE6AC;
        fw.step(1);
        assert_eq!(fw.cpu.registers.index_x, 6);
        assert_ne!(fw.cpu.registers.program_counter, 0xE6AC);
        assert_eq!(fw.mem[0x15], 6);
    }

    /// ROM `LDX $75` at `$E6AC` must see GUI `pending_digit` even when mem was cleared to `$80`.
    #[test]
    fn live_bus_ldx_75_sees_pending_digit() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.bus_state.pending_digit = Some(6);
        fw.mem[0x75] = 0x80;
        fw.mem[0x15] = 0x80;
        fw.cpu.registers.program_counter = 0xE6AC;
        assert!(fw.cpu.single_step());
        assert_eq!(
            fw.cpu.registers.index_x,
            6,
            "LDX $75 must read pending_digit through LiveBus (mem[0x75]={:02X})",
            fw.mem[0x75]
        );
    }

    #[test]
    fn radio_wire_survives_rom_idle_overwrite() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.bus_state.radio_pending = Some(0x05);
        fw.mem[0x75] = 0x80;
        apply_radio_wire(&mut fw.mem, &fw.bus_state.radio_pending, None);
        assert_eq!(fw.mem[0x75], 0x05);
    }

    /// egui runs `logic()` (step) before `ui()` (press_key); mimic that ordering.
    #[test]
    fn stale_15_without_pending_keeps_spinning() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        fw.bus_state.pending_digit = None;
        fw.mem[0x15] = 2;
        fw.mem[0x75] = 0x80;
        fw.last_answer_digit = 2;
        fw.clear_trace();
        for _ in 0..200 {
            fw.step_frame();
        }
        assert!(
            fw.bus_state.pending_digit.is_none(),
            "pending should stay clear"
        );
        assert!(
            !fw.trace_text().contains("[CART] $A199"),
            "stale $15 must not auto-submit (pc=${:04X})",
            fw.status().pc
        );
    }

    #[test]
    fn drive2_rts_never_lands_in_cart_data() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        fw.press_key(RemoteKey::Drive2);
        for _ in 0..2000 {
            fw.step_frame();
        }
        let trace = fw.trace_text();
        assert!(
            !trace.contains("$B7B5") && !trace.contains("BRK"),
            "bad RTS target (pc=${:04X})\n{trace}",
            fw.status().pc
        );
        assert_eq!(fw.mem[0x35], 2, "answer not stored (pc=${:04X})", fw.status().pc);
    }

    #[test]
    fn gui_order_drive2_reaches_cart() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        assert!(in_keypad_spin(fw.status().pc));
        // One logic tick, then ui press (same frame order as egui).
        fw.step_frame();
        fw.press_key(RemoteKey::Drive2);
        for _ in 0..8 {
            fw.step_frame();
        }
        for _ in 0..400 {
            fw.step_frame();
            if fw.mem[0x35] == 2 {
                return;
            }
        }
        panic!(
            "gui-order key never accepted (pc=${:04X} $35=${:02X} pending={:?})",
            fw.status().pc,
            fw.mem[0x35],
            fw.bus_state.pending_digit
        );
    }

    #[test]
    fn drive2_not_stuck_after_8m_cycles() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        fw.press_key(RemoteKey::Drive2);
        fw.step(8_000_000);
        let pc = fw.status().pc;
        assert!(
            pc != 0xE617 || fw.trace_text().contains("[CART] $A1"),
            "still spinning at $E617 after 8M cycles (pc=${pc:04X} $75=${:02X} $15=${:02X} pending={:?})",
            fw.mem[0x75],
            fw.mem[0x15],
            fw.bus_state.pending_digit
        );
    }

    #[test]
    fn drive2_leaves_e617_after_press() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        fw.press_key(RemoteKey::Drive2);
        assert_eq!(fw.bus_state.pending_digit, Some(2));
        fw.step(1);
        for _ in 0..200 {
            fw.step_frame();
            if fw.mem[0x35] == 2 {
                return;
            }
        }
        panic!(
            "digit not accepted (pc=${:04X} $35=${:02X} pending={:?})",
            fw.status().pc,
            fw.mem[0x35],
            fw.bus_state.pending_digit
        );
    }

    #[test]
    fn represent_wire_after_rom_clears_75() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.options.cycles_per_frame = 16_000;
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        assert!(in_keypad_spin(fw.status().pc));
        fw.press_key(RemoteKey::Drive2);
        fw.mem[0x75] = 0x80;
        fw.clear_trace();
        let mut e617 = 0u32;
        for _ in 0..400 {
            fw.step_frame();
            if fw.status().pc == 0xE617 {
                e617 += 1;
            }
        }
        assert_eq!(e617, 0, "pc=${:04X} $75=${:02X}", fw.status().pc, fw.mem[0x75]);
        assert_eq!(fw.mem[0x35], 2, "answer not stored");
    }

    #[test]
    fn latched_key_exits_e617_spin_without_loop() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.options.cycles_per_frame = 16_000;
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        assert!(
            in_keypad_spin(fw.status().pc),
            "expected $E617 spin (pc=${:04X})",
            fw.status().pc
        );
        fw.press_key(RemoteKey::Wrist5);
        fw.clear_trace();
        let mut e617_hits = 0u32;
        for _ in 0..400 {
            fw.step_frame();
            if fw.status().pc == 0xE617 {
                e617_hits += 1;
            }
        }
        assert!(
            e617_hits == 0 && fw.mem[0x35] == 5,
            "key did not return to cart (e617_hits={e617_hits} $35=${:02X} pc=${:04X})",
            fw.mem[0x35],
            fw.status().pc
        );
    }

    #[test]
    fn no_fdd9_irq_spin_after_answer() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.options.cycles_per_frame = 16_000;
        fw.set_auto_submit_enter(true);
        fw.warmup(180);
        fw.press_key(RemoteKey::Drive2);
        fw.digest_keypress(400);
        for _ in 0..400 {
            fw.step_frame();
            let pc = fw.status().pc;
            assert!(
                !(0xFDD9..=0xFDDF).contains(&pc),
                "IRQ tail spin at ${pc:04X} ($75=${:02X} $15=${:02X})",
                fw.mem[0x75],
                fw.mem[0x15]
            );
        }
        assert!(
            !fw.status().needs_enter,
            "answer not submitted ($35=${:02X} pc=${:04X})",
            fw.mem[0x35],
            fw.status().pc
        );
        assert!(
            !(0xE60D..=0xEAFF).contains(&fw.status().pc),
            "should run cart code after answer, not ROM poll (pc=${:04X})",
            fw.status().pc
        );
        for _ in 0..200 {
            fw.step_frame();
            let pc = fw.status().pc;
            assert!(
                !matches!(pc, 0xFDC5 | 0xFDC8),
                "IRQ storm at ${pc:04X} after answer"
            );
            assert!(
                !(0x8000..=0x81FF).contains(&pc),
                "stuck in cart BRK sled at ${pc:04X}"
            );
        }
    }

    #[test]
    fn finish_does_not_hijack_led_display() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.set_auto_submit_enter(true);
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        fw.press_key(RemoteKey::Drive2);
        fw.digest_keypress(400);
        let trace = fw.trace_text();
        let a199_after_done = trace
            .lines()
            .skip_while(|l| !l.contains("$A1B7") && !l.contains("$A1BD"))
            .filter(|l| l.contains("$A199") && l.contains("CMP"))
            .count();
        assert!(
            a199_after_done == 0,
            "ENTER/display must not re-enter input_loop with stale digit (pc=${:04X})\n{trace}",
            fw.status().pc
        );
        assert!(!fw.status().needs_enter, "pc=${:04X} $35=${:02X}", fw.status().pc, fw.mem[0x35]);
        let a199_after_done = trace
            .lines()
            .skip_while(|l| !l.contains("$A1B7") && !l.contains("$A1BD"))
            .filter(|l| l.contains("$A199") && l.contains("CMP") && l.contains("A=$02"))
            .count();
        assert_eq!(
            a199_after_done, 0,
            "stale digit must not re-enter input_loop after ENTER"
        );
    }

    #[test]
    fn press_key_sync_finish_at_e959() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        assert!(
            in_keypad_spin(fw.status().pc),
            "expected $E617 poll (pc=${:04X})",
            fw.status().pc
        );
        fw.set_auto_submit_enter(true);
        // User trace had PC=$E959 inside `$E6A4`; seed that site with a live poll stack.
        let sp = fw.cpu.registers.stack_pointer.0;
        let slot = 0x0100usize + usize::from(sp.wrapping_add(1));
        fw.mem[slot] = 0x19;
        fw.mem[slot + 1] = 0xE6;
        fw.mem[slot + 2] = 0x0F;
        fw.mem[slot + 3] = 0xE6;
        fw.mem[slot + 4] = 0x98;
        fw.mem[slot + 5] = 0xA1;
        fw.cpu.registers.program_counter = 0xE959;
        fw.press_key(RemoteKey::Drive2);
        assert_eq!(
            fw.status().pc, CART_INPUT_LOOP_DISPATCH,
            "click during $E959 must synchronously return to cart (pending={:?})",
            fw.bus_state.pending_digit
        );
    }

    #[test]
    fn press_key_sync_finish_without_step() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        assert!(in_keypad_spin(fw.status().pc));
        fw.set_auto_submit_enter(true);
        fw.press_key(RemoteKey::Drive2);
        assert_eq!(
            fw.status().pc, CART_INPUT_LOOP_DISPATCH,
            "press_key must synchronously return to cart (pending={:?})",
            fw.bus_state.pending_digit
        );
        fw.digest_keypress(200);
        assert_eq!(fw.mem[0x35], 2, "digit not stored");
        assert!(
            !fw.status().needs_enter,
            "auto-ENTER did not finish (pc=${:04X} $35=${:02X} pending={:?})",
            fw.status().pc,
            fw.mem[0x35],
            fw.bus_state.pending_digit
        );
    }

    #[test]
    fn auto_submit_enter_one_click() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.set_auto_submit_enter(true);
        fw.warmup(180);
        fw.press_key(RemoteKey::Drive2);
        fw.digest_keypress(200);
        assert!(
            !fw.status().needs_enter,
            "still waiting for ENTER after auto-submit (pc=${:04X} $35=${:02X} pending={:?})",
            fw.status().pc,
            fw.mem[0x35],
            fw.bus_state.pending_digit
        );
    }

    #[test]
    fn gui_digest_keypress_leaves_poll() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.set_auto_submit_enter(true);
        fw.warmup(180);
        for _ in 0..8000 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        fw.step_frame();
        fw.press_key(RemoteKey::Drive2);
        fw.digest_keypress(200);
        assert!(
            !fw.status().needs_enter,
            "gui digest failed (pc=${:04X} $35=${:02X} pending={:?})",
            fw.status().pc,
            fw.mem[0x35],
            fw.bus_state.pending_digit
        );
    }

    #[test]
    fn drive2_then_enter_leaves_keypad_wait() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        fw.press_key(RemoteKey::Drive2);
        for _ in 0..200 {
            fw.step_frame();
            if fw.mem[0x35] == 2 {
                break;
            }
        }
        assert_eq!(fw.mem[0x35], 2, "digit not stored");
        fw.press_key(RemoteKey::Enter);
        for _ in 0..400 {
            fw.step_frame();
            let pc = fw.status().pc;
            // ENTER completes `input_answer`; firmware may run ROM display (`$EDxx`) next.
            if !in_keypad_spin(pc) && !(0xE6A4..=0xE6B4).contains(&pc) && pc != 0xE617 {
                return;
            }
        }
        panic!(
            "ENTER did not leave keypad poll (pc=${:04X} $15=${:02X} pending={:?})",
            fw.status().pc,
            fw.mem[0x15],
            fw.bus_state.pending_digit
        );
    }

    #[test]
    fn maxxos_cart_runs_after_keypress() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        fw.press_key(RemoteKey::Arms7);
        for _ in 0..800 {
            fw.step_frame();
            if fw.mem[0x35] == 7 {
                return;
            }
        }
        panic!(
            "digit 7 not accepted ($35=${:02X} pc=${:04X} pending={:?})",
            fw.mem[0x35],
            fw.status().pc,
            fw.bus_state.pending_digit
        );
    }

    #[test]
    fn maxxos_led_updates_when_digit_pressed() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        fw.warmup(180);
        let before = fw.led_chars();
        fw.press_key(RemoteKey::Arms7);
        for _ in 0..2000 {
            fw.step_frame();
            let led = fw.led_chars();
            if led.contains('7') {
                return;
            }
        }
        panic!(
            "LED never showed pressed digit (before={before} after={} pc=${:04X} $15=${:02X})",
            fw.led_chars(),
            fw.status().pc,
            fw.mem[0x15]
        );
    }

    #[test]
    fn maxxos_accepts_keypad_input() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        let mut saw_keypad_poll = false;
        for _ in 0..800 {
            fw.step_frame();
            let pc = fw.status().pc;
            if in_keypad_spin(pc) || (0xE60D..=0xE616).contains(&pc) {
                saw_keypad_poll = true;
                break;
            }
        }
        assert!(
            saw_keypad_poll,
            "CPU never entered ROM keypad poll (pc=${:04X} led={})",
            fw.status().pc,
            fw.led_chars()
        );
        fw.press_key(RemoteKey::Wrist5);
        for _ in 0..20_000 {
            fw.step(200);
            if fw.mem[0x15] == 5 {
                return;
            }
        }
        panic!(
            "key 5 not latched (pc=${:04X} $75=${:02X} $15=${:02X})",
            fw.status().pc,
            fw.mem[0x75],
            fw.mem[0x15]
        );
    }

    #[test]
    fn latch_at_e6ac_when_key_pending() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        for _ in 0..5000 {
            fw.step_frame();
            if fw.status().pc == 0xE6AC {
                break;
            }
        }
        assert_eq!(fw.status().pc, 0xE6AC, "never hit LDX $75");
        fw.bus_state.radio_pending = Some(0x05);
        fw.mem[0x75] = 0x80;
        for _ in 0..20 {
            fw.step(1);
            if fw.mem[0x15] == 5 {
                return;
            }
        }
        panic!(
            "natural $E6AC path failed ($75=${:02X} $15=${:02X} pc=${:04X})",
            fw.mem[0x75],
            fw.mem[0x15],
            fw.status().pc
        );
    }

    #[test]
    fn gui_order_latches_from_any_keypad_pc() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        for _ in 0..8000 {
            fw.step_frame();
            if (0xE60D..=0xE642).contains(&fw.status().pc)
                || (0xE6A4..=0xE6B4).contains(&fw.status().pc)
            {
                break;
            }
        }
        let pc = fw.status().pc;
        assert!(
            (0xE60D..=0xE642).contains(&pc) || (0xE6A4..=0xE6B4).contains(&pc),
            "not in keypad path (pc=${pc:04X})"
        );
        fw.press_key(RemoteKey::Wrist5);
        for _ in 0..100 {
            fw.step_frame();
            if fw.mem[0x15] == 5 {
                return;
            }
        }
        panic!(
            "key not latched (start_pc=${pc:04X} now_pc=${:04X} $75=${:02X} $15=${:02X})",
            fw.status().pc,
            fw.mem[0x75],
            fw.mem[0x15]
        );
    }

    /// Mimics egui: `logic()` steps the CPU, then `ui()` calls `press_key`.
    #[test]
    fn press_key_at_e6ac_sets_15_immediately() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        for _ in 0..5000 {
            fw.step_frame();
            if fw.status().pc == 0xE6AC {
                break;
            }
        }
        fw.press_key(RemoteKey::Wrist5);
        assert_eq!(
            fw.mem[0x15], 5,
            "pc=${:04X} $75=${:02X}",
            fw.status().pc,
            fw.mem[0x75]
        );
    }

    #[test]
    fn gui_order_latches_key_while_spinning() {
        let cart = CartImage::from_bytes(MAXXOS.to_vec()).unwrap();
        let mut fw = InteractiveFirmware::new(cart, "MaxxOS").unwrap();
        for _ in 0..800 {
            fw.step_frame();
            if in_keypad_spin(fw.status().pc) {
                break;
            }
        }
        assert!(
            in_keypad_spin(fw.status().pc),
            "expected $E617 spin (pc=${:04X})",
            fw.status().pc
        );
        fw.press_key(RemoteKey::Wrist5);
        assert_eq!(
            fw.mem[0x15], 5,
            "press_key should latch while CPU spins (pc=${:04X} $75=${:02X})",
            fw.status().pc,
            fw.mem[0x75]
        );
        for _ in 0..500 {
            fw.step_frame();
            if fw.mem[0x15] == 5 && fw.status().pc != 0xE617 {
                return;
            }
        }
        panic!(
            "latched key never left $E617 spin (pc=${:04X} $75=${:02X} $15=${:02X})",
            fw.status().pc,
            fw.mem[0x75],
            fw.mem[0x15]
        );
    }
}