use hex;

pub fn fixed_xor(buffer: String, buffer_two: String) -> String {
    if (buffer.len() != buffer_two.len()) {
        panic!("Buffers are not the same length");
    }

    let mut xored_bytes = Vec::new();
    for i in 0..(buffer.len() / 2) {
        let buffer_byte = u8::from_str_radix(&buffer[2 * i..2 * i + 2], 16).unwrap();
        let buffer_two_byte = u8::from_str_radix(&buffer_two[2 * i..2 * i + 2], 16).unwrap();
        xored_bytes.push(buffer_byte ^ buffer_two_byte);
    }

    hex::encode(xored_bytes)
}
