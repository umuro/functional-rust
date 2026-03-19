//! # Fuzzing Concepts
//!
//! Demonstrates code that is fuzz-safe (never panics on any input).

/// A simple binary packet structure
#[derive(Debug, PartialEq, Clone)]
pub struct Packet {
    pub version: u8,
    pub payload_len: u8,
    pub payload: Vec<u8>,
}

/// Errors that can occur when parsing a packet
#[derive(Debug, PartialEq)]
pub enum ParseError {
    TooShort,
    InvalidVersion(u8),
    TruncatedPayload { expected: usize, got: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::TooShort => write!(f, "input too short (need ≥2 bytes)"),
            ParseError::InvalidVersion(v) => write!(f, "invalid version {}", v),
            ParseError::TruncatedPayload { expected, got } => {
                write!(f, "payload truncated: expected {} got {}", expected, got)
            }
        }
    }
}

/// Parse a simple binary packet format.
///
/// Format:
/// - Byte 0: version (must be 1-5)
/// - Byte 1: payload length
/// - Bytes 2..(2+payload_len): payload
///
/// **NEVER panics on any input** — returns Err for invalid data.
pub fn parse_packet(data: &[u8]) -> Result<Packet, ParseError> {
    if data.len() < 2 {
        return Err(ParseError::TooShort);
    }
    let version = data[0];
    if version == 0 || version > 5 {
        return Err(ParseError::InvalidVersion(version));
    }
    let payload_len = data[1] as usize;
    let available = data.len().saturating_sub(2);
    if available < payload_len {
        return Err(ParseError::TruncatedPayload {
            expected: payload_len,
            got: available,
        });
    }
    Ok(Packet {
        version,
        payload_len: payload_len as u8,
        payload: data[2..2 + payload_len].to_vec(),
    })
}

/// Encode a packet back to bytes
pub fn encode_packet(p: &Packet) -> Vec<u8> {
    let mut out = vec![p.version, p.payload_len];
    out.extend_from_slice(&p.payload);
    out
}

/// Parse a key=value string. Must not panic on any &str.
pub fn parse_kv(s: &str) -> Option<(&str, &str)> {
    s.split_once('=').and_then(|(k, v)| {
        if k.is_empty() || v.is_empty() {
            None
        } else {
            Some((k, v))
        }
    })
}

/// Validate that input is ASCII alphanumeric. Never panics.
pub fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_packet() {
        let data = &[1, 3, b'a', b'b', b'c'];
        let packet = parse_packet(data).unwrap();
        assert_eq!(packet.version, 1);
        assert_eq!(packet.payload, vec![b'a', b'b', b'c']);
    }

    #[test]
    fn test_parse_empty_input() {
        assert_eq!(parse_packet(&[]), Err(ParseError::TooShort));
    }

    #[test]
    fn test_parse_too_short() {
        assert_eq!(parse_packet(&[1]), Err(ParseError::TooShort));
    }

    #[test]
    fn test_parse_invalid_version() {
        assert_eq!(parse_packet(&[0, 0]), Err(ParseError::InvalidVersion(0)));
        assert_eq!(parse_packet(&[6, 0]), Err(ParseError::InvalidVersion(6)));
    }

    #[test]
    fn test_parse_truncated_payload() {
        let result = parse_packet(&[1, 10, b'x', b'y']);
        assert_eq!(
            result,
            Err(ParseError::TruncatedPayload {
                expected: 10,
                got: 2
            })
        );
    }

    #[test]
    fn test_roundtrip() {
        let original = Packet {
            version: 3,
            payload_len: 2,
            payload: vec![0xAB, 0xCD],
        };
        let encoded = encode_packet(&original);
        let decoded = parse_packet(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_parse_kv_valid() {
        assert_eq!(parse_kv("key=value"), Some(("key", "value")));
        assert_eq!(parse_kv("a=b"), Some(("a", "b")));
    }

    #[test]
    fn test_parse_kv_invalid() {
        assert_eq!(parse_kv("noequals"), None);
        assert_eq!(parse_kv("=value"), None);
        assert_eq!(parse_kv("key="), None);
        assert_eq!(parse_kv(""), None);
    }

    #[test]
    fn test_valid_identifier() {
        assert!(is_valid_identifier("foo"));
        assert!(is_valid_identifier("foo_bar"));
        assert!(is_valid_identifier("Foo123"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("foo-bar"));
        assert!(!is_valid_identifier("foo bar"));
    }

    // Fuzz-like exhaustive test
    #[test]
    fn test_parse_never_panics() {
        for v in 0..=255u8 {
            for len in 0..=10u8 {
                let data: Vec<u8> = std::iter::once(v)
                    .chain(std::iter::once(len))
                    .chain((0..len).map(|i| i))
                    .collect();
                let _ = parse_packet(&data); // Must not panic
            }
        }
    }
}
