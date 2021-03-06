mod one_dot_one;
mod one_dot_two;
mod one_dot_three;
mod one_dot_four;
mod one_dot_five;

fn main() {
    // 1.1
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(one_dot_one::hex_to_base64(input), result);
    // 1.2
    let input_1 = "1c0111001f010100061a024b53535009181c";
    let input_2 = "686974207468652062756c6c277320657965";
    let result = "746865206b696420646f6e277420706c6179";
    assert_eq!(one_dot_two::xor_strings(input_1, input_2), result);
    // 1.3
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let result = "Cooking MC\'s like a pound of bacon";
    assert_eq!(one_dot_three::decrypt_xor_chiper(input), result);

    // 1.4
    let input = include_str!("files/4.txt");
    let result = "Now that the party is jumping\n";
    assert_eq!(one_dot_four::find_xor_chiper(input, result), result);

    // 1.5
    let key = "ICE";
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let result = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    assert_eq!(one_dot_five::xor_chiper(key, input), result);

    println!("All exercises are OK!");
}
