pub fn encrypt(plaintext: &str, key: &str) -> String {
    plaintext
        .chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| match c {
            'A'..='Z' => {
                ((((c as u8 - b'A') + (k.to_ascii_uppercase() as u8 - b'A')) % 26) + b'A') as char
            }
            'a'..='z' => {
                ((((c as u8 - b'a') + (k.to_ascii_lowercase() as u8 - b'a')) % 26) + b'a') as char
            }
            _ => c,
        })
        .collect()
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let mut plaintext = String::new();
    let key_chars = key.chars().cycle();
    let mut key_iter = key_chars.into_iter();

    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let k = key_iter.next().unwrap();
            let shift = if k.is_ascii_uppercase() {
                k as u8 - b'A'
            } else {
                k as u8 - b'a'
            };
            let decrypted_char = match c {
                'A'..='Z' => ((((c as u8 - b'A') + 26 - shift) % 26) + b'A') as char,
                'a'..='z' => ((((c as u8 - b'a') + 26 - shift) % 26) + b'a') as char,
                _ => c,
            };
            plaintext.push(decrypted_char);
        } else {
            plaintext.push(c);
        }
    }
    plaintext
}
