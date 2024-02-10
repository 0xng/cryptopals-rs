use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::decode_single_xor::decode_message;

pub fn find_single_xor<T>(filepath: T) -> String
where
    T: AsRef<Path>,
{
    let hexes = read_file(filepath).unwrap();
    let mut temp_high_score = 0.0;
    let mut temp_message = String::from("Placeholder");
    for hex in hexes {
        let (decoded_hex, score) = decode_message(&hex);
        if score > temp_high_score {
            temp_high_score = score;
            temp_message = decoded_hex;
        }
    }
    temp_message
}

pub fn read_file<T>(filepath: T) -> io::Result<Vec<String>>
where
    T: AsRef<Path>,
{
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    reader.lines().collect()
}
