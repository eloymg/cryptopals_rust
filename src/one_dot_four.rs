use crate::one_dot_three;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn find_xor_chiper(input: &str,result: &str) -> String {

    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        match one_dot_three::decrypt_xor_chiper(&line[..]).as_str(){
            "Error" => {
            }
            x => {
                if x.replace('\n', "") == result{
                    let ret = x.to_owned().replace('\n', "");
                    return ret             
                }
            }
        }
        
    }
    return String::from("No result")
}