/// 749: Fuzzing Concepts — cargo fuzz approach
///
/// In a real project, create a fuzz target:
///
/// ```
/// // fuzz/fuzz_targets/parse_packet.rs
/// #![no_main]
/// use libfuzzer_sys::fuzz_target;
///
/// fuzz_target!(|data: &[u8]| {
///     let _ = my_crate::parse_packet(data);  // must NEVER panic
/// });
/// ```
///
/// Run with: `cargo fuzz run parse_packet`

// ── The code to fuzz ──────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub struct Packet {
    pub version:     u8,
    pub payload_len: u8,
    pub payload:     Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    TooShort,
    InvalidVersion(u8),
    TruncatedPayload { expected: usize, got: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::TooShort              => write!(f, "input too short (need ≥3 bytes)"),
            ParseError::InvalidVersion(v)     => write!(f, "invalid version {}", v),
            ParseError::TruncatedPayload { expected, got }
                => write!(f, "payload truncated: expected {} got {}", expected, got),
        }
    }
}

/// Parse a simple binary packet format:
/// - Byte 0: version (must be 1-5)
/// - Byte 1: payload length
/// - Bytes 2..(2+payload_len): payload
///
/// NEVER panics on any input — returns Err for invalid data.
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
        return Err(ParseError::TruncatedPayload { expected: payload_len, got: available });
    }
    Ok(Packet {
        version,
        payload_len: payload_len as u8,
        payload: data[2..2 + payload_len].to_vec(),
    })
}

/// A second function to fuzz: parsing a key=value string.
/// Must not panic on any &str.
pub fn parse_kv(s: &str) -> Option<(&str, &str)> {
    s.split_once('=').and_then(|(k, v)| {
        if k.is_empty() || v.is_empty() { None }
        else { Some((k, v)) }
    })
}

/// Roundtrip invariant: encode then decode = identity
pub fn encode_packet(p: &Packet) -> Vec<u8> {
    let mut v = vec![p.version, p.payload_len];
    v.extend_from_slice(&p.payload);
    v
}

/// Fuzz-target equivalent for tests (simulate arbitrary input)
fn fuzz_target(data: &[u8]) {
    // Must not panic
    let result = parse_packet(data);
    if let Ok(ref p) = result {
        // Invariant: encoded form roundtrips
        let encoded = encode_packet(p);
        let decoded = parse_packet(&encoded).unwrap();
        assert_eq!(decoded.version, p.version);
        assert_eq!(decoded.payload, p.payload);
    }
}

fn main() {
    let valid = &[0x01u8, 0x05, b'h', b'e', b'l', b'l', b'o'];
    println!("Parse valid:   {:?}", parse_packet(valid));
    println!("Parse empty:   {:?}", parse_packet(&[]));
    println!("Parse bad ver: {:?}", parse_packet(&[0x09, 0x00]));
    println!("Parse trunc:   {:?}", parse_packet(&[0x01, 0xFF]));

    // Simulate fuzzer corpus
    for data in [
        vec![],
        vec![1],
        vec![1, 3, b'a', b'b', b'c'],
        vec![9, 0],
        vec![1, 5, 0, 0],
        vec![255, 255, 255],
    ] {
        fuzz_target(&data);
    }
    println!("Fuzz simulation: no panics!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_packet() {
        let data = [0x01u8, 0x03, b'a', b'b', b'c'];
        let p = parse_packet(&data).unwrap();
        assert_eq!(p.version, 1);
        assert_eq!(p.payload, b"abc");
    }

    #[test]
    fn parse_empty_is_error() {
        assert_eq!(parse_packet(&[]), Err(ParseError::TooShort));
    }

    #[test]
    fn parse_invalid_version() {
        assert_eq!(parse_packet(&[0, 0]), Err(ParseError::InvalidVersion(0)));
        assert_eq!(parse_packet(&[9, 0]), Err(ParseError::InvalidVersion(9)));
    }

    #[test]
    fn parse_truncated_payload() {
        let data = [1u8, 10, b'h', b'i'];  // says 10 bytes, only 2 provided
        assert!(matches!(parse_packet(&data),
            Err(ParseError::TruncatedPayload { expected: 10, got: 2 })));
    }

    #[test]
    fn roundtrip_invariant() {
        let original = Packet { version: 3, payload_len: 4, payload: vec![1,2,3,4] };
        let encoded = encode_packet(&original);
        let decoded  = parse_packet(&encoded).unwrap();
        assert_eq!(decoded.version, original.version);
        assert_eq!(decoded.payload, original.payload);
    }

    #[test]
    fn fuzz_never_panics_on_random_ish_inputs() {
        // Simulate a small fuzzer corpus
        let corpus: Vec<Vec<u8>> = vec![
            vec![],
            vec![1, 0],
            vec![5, 5, 1, 2, 3, 4, 5],
            vec![255; 100],
            (0u8..=50).collect(),
        ];
        for input in &corpus {
            fuzz_target(input);   // must not panic
        }
    }

    #[test]
    fn parse_kv_valid() {
        assert_eq!(parse_kv("key=value"), Some(("key", "value")));
    }

    #[test]
    fn parse_kv_empty_key_none() {
        assert_eq!(parse_kv("=value"), None);
    }

    #[test]
    fn parse_kv_no_equals_none() {
        assert_eq!(parse_kv("noequals"), None);
    }
}
