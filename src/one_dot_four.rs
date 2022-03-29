use crate::one_dot_three;

pub fn find_xor_chiper(input: &str,result: &str) -> String {
    let reader = input.split("\n");
    for line in reader {
        match one_dot_three::decrypt_xor_chiper(&line[..]).as_str(){
            "Error" => {
            }
            x => {
                if x == result{   
                    let ret = x.to_owned();
                    return ret             
                }
            }
        }
        
    }
    return String::from("No result")
}