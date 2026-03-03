/// # Run-Length Encoding
///
/// Compress consecutive runs of characters into count+char pairs.
/// "AABCCCDEEEE" → "2AB3CD4E"

/// Idiomatic Rust using iterators and fold.
pub fn encode(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut count = 1;

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push(chars[i - 1]);
            count = 1;
        }
    }
    // Don't forget the last run
    if count > 1 {
        result.push_str(&count.to_string());
    }
    result.push(*chars.last().unwrap());
    result
}

/// Decode: expand "2AB3CD4E" → "AABCCCDEEEE"
pub fn decode(s: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            count = count * 10 + (c as u32 - '0' as u32) as usize;
        } else {
            let repeat = if count == 0 { 1 } else { count };
            for _ in 0..repeat {
                result.push(c);
            }
            count = 0;
        }
    }
    result
}

/// Recursive encode
pub fn encode_recursive(s: &str) -> String {
    fn go(chars: &[char], idx: usize, count: usize, result: &mut String) {
        if idx >= chars.len() {
            return;
        }
        if idx + 1 < chars.len() && chars[idx] == chars[idx + 1] {
            go(chars, idx + 1, count + 1, result);
        } else {
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push(chars[idx]);
            go(chars, idx + 1, 1, result);
        }
    }
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    go(&chars, 0, 1, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("AABCCCDEEEE"), "2AB3CD4E");
    }

    #[test]
    fn test_encode_no_repeats() {
        assert_eq!(encode("ABCDE"), "ABCDE");
    }

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(""), "");
    }

    #[test]
    fn test_encode_single() {
        assert_eq!(encode("A"), "A");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("2AB3CD4E"), "AABCCCDEEEE");
    }

    #[test]
    fn test_roundtrip() {
        let original = "AABCCCDEEEE";
        assert_eq!(decode(&encode(original)), original);
    }

    #[test]
    fn test_recursive() {
        assert_eq!(encode_recursive("AABCCCDEEEE"), "2AB3CD4E");
    }
}
