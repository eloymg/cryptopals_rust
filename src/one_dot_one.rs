extern crate base64;

pub fn hex_to_base64(hex_input: &str) -> String {
    let mut bytes = Vec::new();
    for i in 0..(hex_input.len() / 2) {
        let res = u8::from_str_radix(&hex_input[2 * i..2 * i + 2], 16).unwrap();
        bytes.push(res)
    }
    let result = base64::encode(bytes);
    return result;
}
