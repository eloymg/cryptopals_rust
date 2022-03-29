pub fn xor_chiper(key: &str, input: &str) -> String {
    let mut bytes = Vec::new();
    for (index,i) in input.bytes().enumerate() {
        let key_index = index % key.len();
        let b = key.as_bytes()[key_index];
        bytes.push(b ^ i)
    }
    return hex::encode(bytes)
}