const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_from_string(input: &String) -> String {
    let input_bytes: &[u8] = input.as_bytes();
    encode_from_bytes(input_bytes)
}

fn encode_from_bytes(input: &[u8]) -> String {
    // This code is taken from: https://www.youtube.com/watch?v=4ZFypGpGfAo
    // I found this implementation a bit better than what I thought of.
    let mut output: Vec<u8> = Vec::new();
    let mut temp: u32 = 0;
    let mut temp_len: u32 = 0;

    // iterate the vector for each byte
    for &byte in input {
        temp = (temp << 8) | byte as u32; // store the byte character temporarily in `temp` along with other bits
        temp_len += 8; // we appended 8 bits to temp, track its changes with `temp_len`

        // chunk the bits by 6 instead of 8
        while temp_len >= 6 {
            temp_len -= 6; // used for removing remaining bits not part of the 6 bits group
            let chunk: u32 = temp >> temp_len; // contains all 6 bits group
            let char_idx: u32 = chunk & 0x3F; // index
            let character: u8 = CHARSET[char_idx as usize]; // we get the current character
            output.push(character); // append the new character
        }
    }

    // push the last group of bits
    if temp_len > 0 {
        let idx: u32 = temp << (6 - temp_len) & 0x3F;
        let character: u8 = CHARSET[idx as usize];
        output.push(character);
    }

    // push `=` if the encoded bytes is not divisible by 4
    while output.len() % 4 != 0 {
        output.push(b'=');
    }

    String::from_utf8(output).unwrap()
}

fn main() {
    let x = encode_from_string(&String::from("Birb of all feathers."));
    println!("{}", x);
}
