/// Apply a function to every character of a string, producing a new string.
/// Mirrors OCaml's `String.map : (char -> char) -> string -> string`.
pub fn string_map(s: &str, f: impl Fn(char) -> char) -> String {
    s.chars().map(f).collect()
}

/// Build a string of length `n` where position `i` is determined by `f(i)`.
/// Mirrors OCaml's `String.init : int -> (int -> char) -> string`.
pub fn string_init(n: usize, f: impl Fn(usize) -> char) -> String {
    (0..n).map(f).collect()
}

/// ROT-13 cipher — rotate alphabetic characters by 13 positions.
/// Non-alphabetic characters are passed through unchanged.
pub fn rot13(c: char) -> char {
    match c {
        'a'..='z' => (b'a' + (c as u8 - b'a' + 13) % 26) as char,
        'A'..='Z' => (b'A' + (c as u8 - b'A' + 13) % 26) as char,
        _ => c,
    }
}

/// ROT-13 a whole string using `string_map`.
pub fn rot13_string(s: &str) -> String {
    string_map(s, rot13)
}

/// Build the lowercase alphabet using `string_init`.
pub fn alphabet_lowercase() -> String {
    string_init(26, |i| (b'a' + i as u8) as char)
}

/// Build the uppercase alphabet using `string_init`.
pub fn alphabet_uppercase() -> String {
    string_init(26, |i| (b'A' + i as u8) as char)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- string_map tests ---

    #[test]
    fn test_string_map_empty() {
        assert_eq!(string_map("", |c| c), "");
    }

    #[test]
    fn test_string_map_to_uppercase() {
        assert_eq!(string_map("hello", |c| c.to_ascii_uppercase()), "HELLO");
    }

    #[test]
    fn test_string_map_to_lowercase() {
        assert_eq!(string_map("WORLD", |c| c.to_ascii_lowercase()), "world");
    }

    #[test]
    fn test_string_map_identity() {
        assert_eq!(string_map("abc 123!", |c| c), "abc 123!");
    }

    // --- string_init tests ---

    #[test]
    fn test_string_init_empty() {
        assert_eq!(string_init(0, |_| 'x'), "");
    }

    #[test]
    fn test_string_init_alphabet() {
        assert_eq!(
            string_init(26, |i| (b'a' + i as u8) as char),
            "abcdefghijklmnopqrstuvwxyz"
        );
    }

    #[test]
    fn test_string_init_digits() {
        assert_eq!(string_init(10, |i| (b'0' + i as u8) as char), "0123456789");
    }

    #[test]
    fn test_string_init_repeated_char() {
        assert_eq!(string_init(5, |_| '*'), "*****");
    }

    // --- rot13 tests ---

    #[test]
    fn test_rot13_hello_world() {
        assert_eq!(rot13_string("Hello World"), "Uryyb Jbeyq");
    }

    #[test]
    fn test_rot13_is_its_own_inverse() {
        let original = "Hello World";
        let encoded = rot13_string(original);
        let decoded = rot13_string(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_rot13_non_alpha_unchanged() {
        assert_eq!(rot13_string("123 !@#"), "123 !@#");
    }

    #[test]
    fn test_rot13_wraps_around() {
        // 'z' -> 'm', 'Z' -> 'M'
        assert_eq!(rot13('z'), 'm');
        assert_eq!(rot13('Z'), 'M');
        // 'a' -> 'n', 'A' -> 'N'
        assert_eq!(rot13('a'), 'n');
        assert_eq!(rot13('A'), 'N');
    }

    // --- alphabet helpers ---

    #[test]
    fn test_alphabet_lowercase() {
        assert_eq!(alphabet_lowercase(), "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn test_alphabet_uppercase() {
        assert_eq!(alphabet_uppercase(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
}
