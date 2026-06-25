use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PatchSet {
    pub rom_patches: Vec<MemPatch>,
    pub traps: Vec<Trap>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MemPatch {
    pub addr: String,
    pub bytes: Vec<u8>,
    #[allow(dead_code)]
    pub purpose: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Trap {
    pub addr: String,
    pub name: String,
    #[allow(dead_code)]
    pub note: Option<String>,
}

impl PatchSet {
    pub fn embedded() -> Self {
        serde_json::from_str(include_str!("../../../../Simulator/patches.json"))
            .expect("Simulator/patches.json must parse")
    }

    pub fn trap_addrs(&self) -> Vec<(u16, String)> {
        self.traps
            .iter()
            .filter_map(|t| parse_addr(&t.addr).ok().map(|a| (a, t.name.clone())))
            .collect()
    }
}

pub fn parse_addr(s: &str) -> Result<u16, String> {
    let s = s.trim().strip_prefix("0x").unwrap_or(s);
    u16::from_str_radix(s, 16).map_err(|e| format!("bad address {s:?}: {e}"))
}

pub fn apply_patches(mem: &mut [u8; 65536], patches: &PatchSet) -> Result<(), String> {
    for patch in &patches.rom_patches {
        let addr = parse_addr(&patch.addr)? as usize;
        for (i, &byte) in patch.bytes.iter().enumerate() {
            let off = addr + i;
            if off >= mem.len() {
                return Err(format!("patch @{addr:04X}+{i} out of range"));
            }
            mem[off] = byte;
        }
    }
    Ok(())
}