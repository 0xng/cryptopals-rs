use hex;

pub fn encrypt_repeating_xor(message: &str, key: &str) -> String {
    let key_bytes = key.bytes().cycle();
    let message_bytes = message.bytes();

    let xored_bytes: Vec<u8> = message_bytes
        .zip(key_bytes)
        .map(|(byte, key)| byte ^ key)
        .collect();

    hex::encode(xored_bytes)
}
