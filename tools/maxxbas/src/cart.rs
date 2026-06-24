use std::fs;
use std::path::Path;

use crate::emit::CART_SIZE;

pub const BASE_ADDR: u16 = 0xA000;

#[derive(Debug, Clone)]
pub struct CartImage {
    pub data: Vec<u8>,
    pub base_addr: u16,
}

impl CartImage {
    pub fn load(path: &Path) -> Result<Self, String> {
        let data = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Self::from_bytes(data)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Self, String> {
        if data.len() != CART_SIZE {
            return Err(format!("expected {CART_SIZE} bytes, got {}", data.len()));
        }
        Ok(Self {
            data,
            base_addr: BASE_ADDR,
        })
    }

    pub fn entry_vector(&self) -> u16 {
        u16::from_le_bytes([self.data[0], self.data[1]])
    }

    pub fn copyright(&self) -> &[u8] {
        &self.data[2..19]
    }

    pub fn copyright_str(&self) -> String {
        String::from_utf8_lossy(self.copyright()).to_string()
    }
}