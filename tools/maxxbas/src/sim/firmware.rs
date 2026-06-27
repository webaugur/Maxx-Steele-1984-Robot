use mos6502::cpu::CPU;
use mos6502::instruction::Cmos6502;
use mos6502::memory::Bus;
use serde::Serialize;

use super::patches::{apply_patches, PatchSet};
use crate::cart::BASE_ADDR;
use crate::CartImage;

const INTERNAL_ROM_BASE: usize = 0xE000;
const CART_BASE: usize = 0xA000;
const PROG_RAM: usize = 0x0200;

#[derive(Debug, Clone)]
pub struct FirmwareOptions {
    pub max_cycles: u64,
    pub inject_key: Option<u8>,
    pub run_cart_bootstrap: bool,
    pub cart: Option<CartImage>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TrapHit {
    pub name: String,
    pub addr: u16,
    pub cycle: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FirmwareResult {
    pub cycles: u64,
    pub final_pc: u16,
    pub traps_hit: Vec<TrapHit>,
    pub status_02: u8,
    pub status_03: u8,
    pub mode_0d: u8,
    pub program_steps_in_ram: usize,
    pub stopped_reason: String,
}

pub fn build_memory_image(cart: Option<&CartImage>, patches: &PatchSet) -> Result<[u8; 65536], String> {
    let rom = include_bytes!("../../../../Mainboard/Firmware/Binary/Maxxrom.64");
    let mut mem = [0u8; 65536];

    if rom.len() != 8192 {
        return Err(format!("internal ROM must be 8192 bytes, got {}", rom.len()));
    }

    mem[INTERNAL_ROM_BASE..INTERNAL_ROM_BASE + rom.len()].copy_from_slice(rom);
    mem[0xFFFA..0x10000].copy_from_slice(&rom[0x1FFA..0x2000]);

    if let Some(cart) = cart {
        if cart.data.len() != 4096 {
            return Err("cart image must be 4096 bytes".into());
        }
        mem[CART_BASE..CART_BASE + 4096].copy_from_slice(&cart.data);
    }

    apply_patches(&mut mem, patches)?;
    Ok(mem)
}

pub fn run_firmware(
    mem: &mut [u8; 65536],
    patches: &PatchSet,
    options: &FirmwareOptions,
) -> FirmwareResult {
    if options.inject_key.is_some() {
        mem[0x75] = options.inject_key.unwrap();
        mem[0x15] = options.inject_key.unwrap();
    }

    if options.run_cart_bootstrap {
        if let Some(ref cart) = options.cart {
            maybe_bootstrap_cart_tables(mem, cart);
        }
    }

    let traps = patches.trap_addrs();
    let mut cpu = CPU::new(MaxxBus { mem }, Cmos6502);
    cpu.reset();

    let mut traps_hit = Vec::new();
    let mut stopped_reason = "cycle_limit".to_string();

    while cpu.cycles < options.max_cycles {
        let pc = cpu.registers.program_counter;
        for (addr, name) in &traps {
            if pc == *addr && !traps_hit.iter().any(|t: &TrapHit| t.addr == *addr) {
                traps_hit.push(TrapHit {
                    name: name.clone(),
                    addr: *addr,
                    cycle: cpu.cycles,
                });
            }
        }

        if !cpu.single_step() {
            stopped_reason = "cpu_wait".to_string();
            break;
        }
    }

    if cpu.cycles >= options.max_cycles {
        stopped_reason = "cycle_limit".to_string();
    }

    let cycles = cpu.cycles;
    let final_pc = cpu.registers.program_counter;
    drop(cpu);

    let program_steps_in_ram = count_program_steps(&mem[PROG_RAM..PROG_RAM + 512]);

    FirmwareResult {
        cycles,
        final_pc,
        traps_hit,
        status_02: mem[0x02],
        status_03: mem[0x03],
        mode_0d: mem[0x0D],
        program_steps_in_ram,
        stopped_reason,
    }
}

/// ROM copyright bytes at `$E000` (17 bytes) — warm reset checks `$0100` against this.
const WARM_SIGNATURE_LEN: usize = 17;

/// RAM vector table copied from ROM `$E01C` on warm start (`$72`–`$96`, 37 bytes).
const IRQ_VECTOR_TABLE_LEN: usize = 37;

/// Match warm-reset RAM so ROM copies the `$72` vector table instead of cold-zeroing ZP.
pub fn seed_warm_start_signature(mem: &mut [u8; 65536]) {
    for i in 0..WARM_SIGNATURE_LEN {
        mem[0x0100 + i] = mem[0xE000 + i];
    }
}

/// Install IRQ/NMI/dispatch vectors at `$72` before the CPU can take an interrupt.
///
/// Cold start leaves `$0078` at `$0000`; the hardware IRQ stub at `$FDC5` does `JMP ($0078)`,
/// which traps the CPU at `$0000` executing `BRK` forever.
pub fn bootstrap_irq_vectors(mem: &mut [u8; 65536]) {
    for i in 0..IRQ_VECTOR_TABLE_LEN {
        mem[0x72 + i] = mem[0xE01C + i];
    }
}

pub fn irq_vector(mem: &[u8; 65536]) -> u16 {
    u16::from_le_bytes([mem[0x78], mem[0x79]])
}

pub fn ensure_irq_vectors(mem: &mut [u8; 65536]) {
    if irq_vector(mem) != IRQ_VECTOR_FDC8 {
        bootstrap_irq_vectors(mem);
    }
}

/// ROM keypad poll loop — safe recovery target if PC falls into zero page.
pub const ROM_KEYPAD_POLL: u16 = 0xE617;

/// IRQ handler pointer installed at `$0078` on warm start.
pub const IRQ_VECTOR_FDC8: u16 = 0xFDC8;

pub fn prepare_interactive_memory(mem: &mut [u8; 65536], cart: &CartImage) {
    maybe_bootstrap_cart_tables(mem, cart);
    seed_warm_start_signature(mem);
    bootstrap_irq_vectors(mem);
}

/// Factory cart bootstrap copies bytecode tables before `JMP $E0B6`.
pub fn maybe_bootstrap_cart_tables(mem: &mut [u8; 65536], cart: &CartImage) {
    if !cart_returns_to_main_loop(cart) {
        return;
    }
    mem[0x02] = 0x02;
    mem[0x03] = 0x82;

    for x in 0u16..=255 {
        let i = x as usize;
        mem[0x0200 + i] = mem[CART_BASE + 0x035 + i];
        mem[0x0500 + i] = mem[CART_BASE + 0x081 + i];
        mem[0x0400 + i] = mem[CART_BASE + 0x0BB + i];
    }
}

pub fn cart_returns_to_main_loop(cart: &CartImage) -> bool {
    let off = cart.entry_vector().wrapping_sub(BASE_ADDR) as usize;
    if off >= cart.data.len() {
        return false;
    }
    let end = (off + 64).min(cart.data.len());
    cart.data[off..end]
        .windows(3)
        .any(|w| w == [0x4C, 0xB6, 0xE0])
}

fn count_program_steps(prog: &[u8]) -> usize {
    let mut count = 0;
    let mut i = 0;
    while i + 1 < prog.len() {
        if prog[i] == 0xFF && prog[i + 1] == 0xFF {
            return count + 1;
        }
        i += 2;
        count += 1;
    }
    count
}

struct MaxxBus<'a> {
    mem: &'a mut [u8; 65536],
}

impl Bus for MaxxBus<'_> {
    fn get_byte(&mut self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
}