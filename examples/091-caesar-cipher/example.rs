//! # Caesar Cipher — Functional Encryption
//!
//! Character-level string transformation using modular arithmetic.
//! OCaml's `String.map` maps to Rust's iterator `.map().collect()`.

// ---------------------------------------------------------------------------
// Approach A: Idiomatic Rust — iterators + collect
// ---------------------------------------------------------------------------

pub fn shift_char(n: u8, c: char) -> char {
    match c {
        'a'..='z' => (b'a' + (c as u8 - b'a' + n) % 26) as char,
        'A'..='Z' => (b'A' + (c as u8 - b'A' + n) % 26) as char,
        _ => c,
    }
}

pub fn caesar(n: u8, s: &str) -> String {
    s.chars().map(|c| shift_char(n, c)).collect()
}

pub fn decrypt(n: u8, s: &str) -> String {
    caesar(26 - n, s)
}

// ---------------------------------------------------------------------------
// Approach B: Fold-based — build string with fold
// ---------------------------------------------------------------------------

pub fn caesar_fold(n: u8, s: &str) -> String {
    s.chars().fold(String::with_capacity(s.len()), |mut acc, c| {
        acc.push(shift_char(n, c));
        acc
    })
}

// ---------------------------------------------------------------------------
// Approach C: In-place on bytes (for ASCII-only)
// ---------------------------------------------------------------------------

pub fn caesar_bytes(n: u8, s: &str) -> String {
    let mut bytes = s.as_bytes().to_vec();
    for b in bytes.iter_mut() {
        *b = match *b {
            b'a'..=b'z' => b'a' + (*b - b'a' + n) % 26,
            b'A'..=b'Z' => b'A' + (*b - b'A' + n) % 26,
            _ => *b,
        };
    }
    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13() {
        assert_eq!(caesar(13, "Hello World"), "Uryyb Jbeyq");
    }

    #[test]
    fn test_roundtrip() {
        let msg = "Hello World";
        assert_eq!(decrypt(13, &caesar(13, msg)), msg);
    }

    #[test]
    fn test_rot0() {
        assert_eq!(caesar(0, "abc"), "abc");
    }

    #[test]
    fn test_rot26() {
        assert_eq!(caesar(26, "abc"), "abc");
    }

    #[test]
    fn test_non_alpha() {
        assert_eq!(caesar(5, "Hello, World! 123"), "Mjqqt, Btwqi! 123");
    }

    #[test]
    fn test_fold_matches() {
        assert_eq!(caesar(7, "Test"), caesar_fold(7, "Test"));
    }

    #[test]
    fn test_bytes_matches() {
        assert_eq!(caesar(7, "Test"), caesar_bytes(7, "Test"));
    }
}

fn main() {
    println!("{:?}", caesar(13, "Hello World"), "Uryyb Jbeyq");
    println!("{:?}", decrypt(13, &caesar(13, msg)), msg);
    println!("{:?}", caesar(0, "abc"), "abc");
}
