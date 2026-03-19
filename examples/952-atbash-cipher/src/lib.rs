// Atbash cipher: maps a→z, b→y, ..., z→a. Digits pass through. Groups into 5-char chunks.

fn atbash_char(c: char) -> Option<char> {
    if c.is_ascii_lowercase() {
        Some((b'z' - (c as u8 - b'a')) as char)
    } else if c.is_ascii_digit() {
        Some(c)
    } else {
        None
    }
}

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
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter_map(atbash_char)
        .collect()
}

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

/* Output:
   encode("Testing, 1 2 3, testing.") = "gvhgr mt123 gvhgr mt"
   decode("gvhgr mt123 gvhgr mt") = "testing123testing"
   encode_recursive("Testing, 1 2 3, testing.") = "gvhgr mt123 gvhgr mt"
   encode("Hello, World!") = "svool dliow"
   decode("svool") = "hello"
*/
