//! # String Encoding — UTF-8 and Beyond

pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

pub fn bytes_to_string(bytes: &[u8]) -> Option<String> {
    std::str::from_utf8(bytes).ok().map(String::from)
}

pub fn bytes_to_string_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

pub fn is_valid_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

pub fn encode_escape(s: &str) -> String {
    s.escape_default().to_string()
}

pub fn char_to_u32(c: char) -> u32 {
    c as u32
}

pub fn u32_to_char(n: u32) -> Option<char> {
    char::from_u32(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8() {
        let s = "hello 世界";
        let bytes = string_to_bytes(s);
        assert_eq!(bytes_to_string(&bytes), Some(s.to_string()));
    }

    #[test]
    fn test_invalid_utf8() {
        let invalid = vec![0xff, 0xfe];
        assert!(!is_valid_utf8(&invalid));
        assert!(bytes_to_string_lossy(&invalid).contains('\u{FFFD}'));
    }

    #[test]
    fn test_char_conversion() {
        assert_eq!(char_to_u32('A'), 65);
        assert_eq!(u32_to_char(65), Some('A'));
        assert_eq!(u32_to_char(0x1F600), Some('😀'));
    }
}
