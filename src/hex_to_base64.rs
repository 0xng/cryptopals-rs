use base64::{engine::general_purpose, Engine as _};

pub fn hex_to_base64(hex: String) -> String {
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let hex_to_u8 = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap();
        bytes.push(hex_to_u8);
    }
    general_purpose::STANDARD.encode(&bytes)
}
