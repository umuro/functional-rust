//! Caesar Cipher — Functional Encryption
//!
//! OCaml: `let caesar n s = String.map (shift_char n) s`
//! Rust: `fn caesar(n: u8, s: &str) -> String` using `.chars().map().collect()`
//!
//! A Caesar cipher shifts each letter by a fixed number of positions
//! in the alphabet, wrapping around. Non-letter characters pass through unchanged.

//! Shifts a single character by `n` positions.
//!
//! OCaml: pattern matches on char ranges with `>=` and `<=`.
//! Rust: uses range patterns in match arms.
fn shift_char(n: u8, c: char) -> char {
    match c {
        'a'..='z' => {
            let shifted = (c as u8 - b'a' + n) % 26 + b'a';
            shifted as char
        }
        'A'..='Z' => {
            let shifted = (c as u8 - b'A' + n) % 26 + b'A';
            shifted as char
        }
        _ => c,
    }
}

/// Encrypts a string using Caesar cipher with shift `n`.
///
/// OCaml: `let caesar n s = String.map (shift_char n) s`
/// Rust uses iterator chain: `.chars().map().collect()`.
pub fn caesar(n: u8, s: &str) -> String {
    s.chars().map(|c| shift_char(n, c)).collect()
}

/// Decrypts by shifting in the opposite direction.
///
/// OCaml: `let decrypt n = caesar (26 - n)`
/// Rust: same idea — shift by `26 - n`.
pub fn decrypt(n: u8, s: &str) -> String {
    caesar(26 - (n % 26), s)
}

/// ROT13: the classic self-inverse Caesar cipher (shift by 13).
/// Applying it twice returns the original.
pub fn rot13(s: &str) -> String {
    caesar(13, s)
}

/// Iterator-based approach: returns a lazy iterator of shifted chars.
pub fn caesar_iter(n: u8, s: &str) -> impl Iterator<Item = char> + '_ {
    s.chars().map(move |c| shift_char(n, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_encryption() {
        assert_eq!(caesar(13, "Hello World"), "Uryyb Jbeyq");
    }

    #[test]
    fn test_decryption_reverses() {
        let msg = "Hello World";
        let encrypted = caesar(13, msg);
        let decrypted = decrypt(13, &encrypted);
        assert_eq!(decrypted, msg);
    }

    #[test]
    fn test_rot13_self_inverse() {
        let msg = "The Quick Brown Fox";
        assert_eq!(rot13(&rot13(msg)), msg);
    }

    #[test]
    fn test_preserves_non_alpha() {
        assert_eq!(caesar(5, "Hello, World! 123"), "Mjqqt, Btwqi! 123");
    }

    #[test]
    fn test_zero_shift() {
        assert_eq!(caesar(0, "unchanged"), "unchanged");
    }

    #[test]
    fn test_full_rotation() {
        assert_eq!(caesar(26, "abc"), "abc");
    }

    #[test]
    fn test_wraparound() {
        assert_eq!(caesar(1, "xyz"), "yza");
        assert_eq!(caesar(1, "XYZ"), "YZA");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(caesar(5, ""), "");
    }
}
