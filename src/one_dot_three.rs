use std::str;

pub fn decrypt_xor_chiper(input: &str) -> String{
    let mut max_value: i32 = 0;
    let mut final_bytes = Vec::new();
    final_bytes.push(1);
    
    for i in 0..255 {
        let mut bytes = Vec::new();
        for j in 0..(input.len() / 2) {
            let b = u8::from_str_radix(&input[2 * j..2 * j + 2], 16).unwrap();
            let single_bite = i as u8;
            let xored = single_bite ^ b;
            bytes.push(xored);
        }
        let num_of_most_common_chars: i32 = count_printable_chars(bytes.as_slice());
        if num_of_most_common_chars>max_value {
            max_value = num_of_most_common_chars;
            final_bytes = bytes
        }
    }
    match str::from_utf8(final_bytes.as_slice()){
        Ok(valid) => {
            return valid.to_owned()
        }
        Err(_) => {
            return String::from("Error")
        }
    }


    //return str::from_utf8(final_bytes.as_slice()).unwrap().to_owned();

}
fn count_printable_chars(slice: &[u8]) -> i32 {
    let mut num_of_most_common_chars: i32 = 0;
    let most_common_chars_mayus: [char; 13] = ['E','T','A','O','I','N',' ','S','H','R','D','L','U'];
    let most_common_chars_minus: [char; 12] = ['e','t','a','o','i','n','s','h','r','d','l','u'];
    for j in slice {
        let c: char = *j as char;
        if most_common_chars_mayus.contains(&c) {
            num_of_most_common_chars+=1
        }
        if most_common_chars_minus.contains(&c) {
            num_of_most_common_chars+=1
        }
    }
    return num_of_most_common_chars;
}