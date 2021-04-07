pub fn xor_strings(input_1: &str, input_2: &str) -> String {
    let mut bytes = Vec::new();
    for i in 0..(input_1.len() / 2) {
        let b1 = u8::from_str_radix(&input_1[2 * i..2 * i + 2], 16).unwrap();
        let b2 = u8::from_str_radix(&input_2[2 * i..2 * i + 2], 16).unwrap();
        bytes.push(b1 ^ b2)
    }
    return hex::encode(bytes);
}
