// Atbash cipher: maps a→z, b→y, c→x, ..., z→a. Digits pass through unchanged.
// Encoded output is grouped into 5-character chunks separated by spaces.

// Solution 1: Idiomatic Rust — iterator pipeline with chunking
pub fn encode(input: &str) -> String {
    let chars: Vec<char> = input
        .to_lowercase()
        .chars()
        .filter_map(atbash_char)
        .collect();

    chars
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode(input: &str) -> String {
    // Decoding is the same transformation — atbash is its own inverse
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter_map(atbash_char)
        .collect()
}

// Maps a single character: letters are transposed, digits pass through, others filtered
fn atbash_char(c: char) -> Option<char> {
    if c.is_ascii_lowercase() {
        // 'a' <-> 'z', 'b' <-> 'y', etc.
        Some((b'z' - (c as u8 - b'a')) as char)
    } else if c.is_ascii_digit() {
        Some(c)
    } else {
        None
    }
}

// Solution 2: Functional/recursive — mirrors the OCaml group recursion
pub fn encode_recursive(input: &str) -> String {
    let chars: Vec<char> = input
        .to_lowercase()
        .chars()
        .filter_map(atbash_char)
        .collect();

    fn group(chars: &[char]) -> Vec<String> {
        if chars.is_empty() {
            return vec![];
        }
        let (chunk, rest) = chars.split_at(chars.len().min(5));
        let mut result = vec![chunk.iter().collect::<String>()];
        result.extend(group(rest));
        result
    }

    group(&chars).join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- encode tests ---

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(""), "");
    }

    #[test]
    fn test_encode_single_letter() {
        assert_eq!(encode("a"), "z");
        assert_eq!(encode("z"), "a");
        assert_eq!(encode("m"), "n");
    }

    #[test]
    fn test_encode_digits_pass_through() {
        assert_eq!(encode("1 2 3"), "123");
    }

    #[test]
    fn test_encode_strips_punctuation() {
        assert_eq!(encode("Hello, World!"), "svool dliow");
    }

    #[test]
    fn test_encode_groups_of_five() {
        assert_eq!(encode("Testing, 1 2 3, testing."), "gvhgr mt123 gvhgr mt");
    }

    #[test]
    fn test_encode_exactly_five() {
        assert_eq!(encode("abcde"), "zyxwv");
    }

    #[test]
    fn test_encode_more_than_five() {
        assert_eq!(encode("abcdefghij"), "zyxwv utsrq");
    }

    #[test]
    fn test_encode_uppercase_input() {
        assert_eq!(encode("RUST"), "ifhg");
    }

    // --- decode tests ---

    #[test]
    fn test_decode_empty() {
        assert_eq!(decode(""), "");
    }

    #[test]
    fn test_decode_strips_spaces() {
        // "gvhgr mt" is encoded "testing" — spaces are ignored, letters re-transposed
        assert_eq!(decode("gvhgr mt"), "testing");
    }

    #[test]
    fn test_decode_roundtrip() {
        let original = "hello";
        assert_eq!(decode(&encode(original).replace(' ', "")), original);
    }

    #[test]
    fn test_decode_known() {
        // "svool" encodes back to "hello"
        assert_eq!(decode("svool"), "hello");
    }

    // --- recursive variant ---

    #[test]
    fn test_encode_recursive_matches_idiomatic() {
        let cases = ["", "abc", "Testing, 1 2 3, testing.", "abcdefghij"];
        for &s in &cases {
            assert_eq!(encode(s), encode_recursive(s), "mismatch for {:?}", s);
        }
    }
}
