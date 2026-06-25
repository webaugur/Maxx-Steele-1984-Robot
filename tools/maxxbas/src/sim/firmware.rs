use mos6502::cpu::CPU;
use mos6502::instruction::Cmos6502;
use mos6502::memory::Bus;
use serde::Serialize;

use super::patches::{apply_patches, PatchSet};
use crate::CartImage;

const INTERNAL_ROM_BASE: usize = 0xE000;
const CART_BASE: usize = 0xA000;
const PROG_RAM: usize = 0x0200;

#[derive(Debug, Clone)]
pub struct FirmwareOptions {
    pub max_cycles: u64,
    pub inject_key: Option<u8>,
    pub run_cart_bootstrap: bool,
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
    let rom = include_bytes!("../../../../Chassis/Firmware/Binary/Maxxrom.64");
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
        bootstrap_cart(mem);
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

/// Replicate the cart bootstrap stub at `$A013` (copy tables, set status, enter ROM).
fn bootstrap_cart(mem: &mut [u8; 65536]) {
    mem[0x02] = 0x02;
    mem[0x03] = 0x82;

    for x in 0u16..=255 {
        let i = x as usize;
        mem[0x0200 + i] = mem[CART_BASE + 0x035 + i];
        mem[0x0500 + i] = mem[CART_BASE + 0x081 + i];
        mem[0x0400 + i] = mem[CART_BASE + 0x0BB + i];
    }
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