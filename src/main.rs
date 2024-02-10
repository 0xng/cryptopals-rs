mod decode_single_xor;
mod find_single_xor;
mod fixed_xor;
mod hex_to_base64;
mod repeating_xor;

// TODO: properly assert results
// TODO: go over the code and find improvements, turn it idiomatic.
fn main() {
    let challenge_one_res = hex_to_base64::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());
    let challenge_two_res = fixed_xor::fixed_xor(
        "1c0111001f010100061a024b53535009181c".to_string(),
        "686974207468652062756c6c277320657965".to_string(),
    );
    let challenge_three_res = decode_single_xor::decode_message(
        &"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string(),
    );
    let challenge_four_res = find_single_xor::find_single_xor("src/files/challenge_four.txt");
    let challenge_five_res = repeating_xor::encrypt_repeating_xor(
        &"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
        "ICE",
    );
    println!("Challenge 1: {}", challenge_one_res);
    println!("Challenge 2: {}", challenge_two_res);
    println!("Challenge 3: {:?}", challenge_three_res);
    println!("Challenge 4: {:?}", challenge_four_res);
    println!("Challenge 5: {:?}", challenge_five_res);
}
