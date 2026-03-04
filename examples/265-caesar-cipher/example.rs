/// Caesar Cipher — Functional Encryption

fn shift_char(n: u8, c: char) -> char {
    match c {
        'a'..='z' => ((c as u8 - b'a' + n) % 26 + b'a') as char,
        'A'..='Z' => ((c as u8 - b'A' + n) % 26 + b'A') as char,
        _ => c,
    }
}

pub fn caesar(n: u8, s: &str) -> String {
    s.chars().map(|c| shift_char(n, c)).collect()
}

pub fn decrypt(n: u8, s: &str) -> String {
    caesar(26 - (n % 26), s)
}

pub fn rot13(s: &str) -> String {
    caesar(13, s)
}

fn main() {
    let msg = "Hello World";
    let enc = caesar(13, msg);
    println!("Encrypted: {}", enc);
    println!("Decrypted: {}", decrypt(13, &enc));
    println!("ROT13 twice: {}", rot13(&rot13(msg)));
}

/* Output:
   Encrypted: Uryyb Jbeyq
   Decrypted: Hello World
   ROT13 twice: Hello World
*/
