use std::collections::HashMap;

pub fn decode_message(hex: &str) -> (String, f32) {
    let mut most_valuable_key = 0;
    let mut temp_max_score = 0.0;
    for i in 0..255 {
        let encoded_message = single_xor_encode(&hex, i);
        let message_as_string = hex_to_string(&encoded_message.unwrap()).unwrap_or_default();
        let score = score_message(&message_as_string);
        if score > temp_max_score {
            temp_max_score = score;
            most_valuable_key = i;
        }
    }

    let decoded_message = single_xor_encode(&hex, most_valuable_key).unwrap();
    (
        hex_to_string(&decoded_message).unwrap_or(String::from("Null")),
        temp_max_score,
    )
}

pub fn single_xor_encode(hex: &str, byte: u8) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = (0..hex.len())
        .step_by(2)
        .map(|octet| {
            let byte_chunk = u8::from_str_radix(&hex[octet..octet + 2], 16)?;
            Ok(byte_chunk ^ byte)
        })
        .collect::<Result<Vec<u8>, _>>();

    bytes.map(|vec| vec.iter().map(|b| format!("{:02x}", b)).collect())
}

pub fn score_message(message: &str) -> f32 {
    let frequency_hashmap = get_frequency_hashmap();
    message.to_ascii_lowercase().chars().fold(0.0, |acc, c| {
        acc + *frequency_hashmap.get(&c).unwrap_or(&-10.0)
    })
}

pub fn hex_to_string(hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = (0..hex.len())
        .step_by(2)
        .map(|byte| u8::from_str_radix(&hex[byte..byte + 2], 16))
        .collect::<Result<Vec<u8>, _>>()?;

    String::from_utf8(bytes).map_err(Into::into)
}

pub fn get_frequency_hashmap() -> HashMap<char, f32> {
    let mut frequency_score = HashMap::with_capacity(26);
    frequency_score.insert('e', 12.02);
    frequency_score.insert('t', 9.10);
    frequency_score.insert('a', 8.12);
    frequency_score.insert('o', 7.68);
    frequency_score.insert('i', 7.31);
    frequency_score.insert('n', 6.95);
    frequency_score.insert('s', 6.28);
    frequency_score.insert('r', 6.02);
    frequency_score.insert('h', 5.92);
    frequency_score.insert('d', 4.32);
    frequency_score.insert('l', 3.98);
    frequency_score.insert('u', 2.88);
    frequency_score.insert('c', 2.71);
    frequency_score.insert('m', 2.61);
    frequency_score.insert('f', 2.30);
    frequency_score.insert('y', 2.11);
    frequency_score.insert('w', 2.09);
    frequency_score.insert('g', 2.03);
    frequency_score.insert('p', 1.82);
    frequency_score.insert('b', 1.49);
    frequency_score.insert('v', 1.11);
    frequency_score.insert('k', 0.69);
    frequency_score.insert('x', 0.17);
    frequency_score.insert('q', 0.11);
    frequency_score.insert('j', 0.10);
    frequency_score.insert('\'', 0.00);
    frequency_score.insert('\"', 0.00);
    frequency_score.insert('?', 0.00);
    frequency_score.insert('!', 0.00);
    frequency_score.insert(' ', 5.00);
    frequency_score
}
