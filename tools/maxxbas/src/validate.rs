use crate::cart::CartImage;
use crate::decode::{decode_cart, find_program_table};
use crate::emit::{COPYRIGHT_CBS, COPYRIGHT_ULTRAMAXX, CART_SIZE};

pub fn validate_cart(data: &[u8], base_addr: u16) -> Vec<String> {
    let mut issues = Vec::new();

    if data.len() != CART_SIZE {
        issues.push(format!("expected {CART_SIZE} bytes, got {}", data.len()));
        return issues;
    }

    let copyright = &data[2..19];
    if copyright != COPYRIGHT_CBS.as_slice() && copyright != COPYRIGHT_ULTRAMAXX.as_slice() {
        issues.push(format!(
            "copyright mismatch: {:?}",
            String::from_utf8_lossy(copyright)
        ));
    }

    let entry_vector = u16::from_le_bytes([data[0], data[1]]);
    let entry_off = entry_vector as i32 - base_addr as i32;
    if entry_off < 0 || entry_off as usize >= CART_SIZE {
        issues.push(format!("entry vector ${entry_vector:04X} out of range"));
    } else {
        let chunk = &data[entry_off as usize..entry_off as usize + 4];
        if chunk != [0xA9, 0x02, 0x85, 0x02] {
            issues.push(format!(
                "unexpected entry code at ${entry_vector:04X}: {}",
                hex_encode(chunk)
            ));
        }
    }

    if let Some(prog_start) = find_program_table(data) {
        let prog_end = data[prog_start..]
            .chunks(2)
            .position(|pair| pair == [0xFF, 0xFF]);
        if prog_end.is_none() {
            issues.push("program table missing FF FF terminator".into());
        }
    } else {
        issues.push("could not locate program table".into());
    }

    issues
}

pub fn validate_cart_image(cart: &CartImage) -> Vec<String> {
    let mut issues = validate_cart(&cart.data, cart.base_addr);
    if issues.is_empty() {
        if let Err(err) = decode_cart(cart) {
            issues.push(err);
        }
    }
    issues
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::{compile_source, Copyright};

    #[test]
    fn hello_bas_validates() {
        let src = include_str!("../../../Cartridge/Examples/UltraMaxx/Firmware/Basic/hello.bas");
        let image = compile_source(src, Copyright::UltraMaxx).unwrap();
        assert_eq!(validate_cart(&image, 0xA000), Vec::<String>::new());
    }
}