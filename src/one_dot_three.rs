use std::str;

pub fn decrypt_xor_chiper(input: &str) {

    for i in 0..255 {
        let mut bytes = Vec::new();
        for j in 0..(input.len() / 2) {
            let b = u8::from_str_radix(&input[2 * j..2 * j + 2], 16).unwrap();
            let single_bite = i as u8;
            let xored = single_bite ^ b;
            bytes.push(xored);
        }
        println!("{}",str::from_utf8(bytes.as_slice()).unwrap())
    }

}