pub fn encrypt(plaintext: &str, shift: u32) -> String {
    plaintext
        .chars()
        .map(|c| match c {
            'A'..='Z' => ((((c as u8 - b'A') + shift as u8) % 26) + b'A') as char,
            'a'..='z' => ((((c as u8 - b'a') + shift as u8) % 26) + b'a') as char,
            _ => c,
        })
        .collect()
}

pub fn decrypt(ciphertext: &str, shift: u32) -> String {
    encrypt(ciphertext, 26 - shift)
}
