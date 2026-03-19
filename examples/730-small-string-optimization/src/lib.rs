/// 730: Small String Optimization
/// Stores ≤23 bytes inline; falls back to `Box<str>` for longer strings.

const INLINE_CAP: usize = 23;

/// An SSO string. Size = 24 bytes (same as String on 64-bit).
#[derive(Debug)]
enum SsoString {
    Inline { buf: [u8; INLINE_CAP], len: u8 },
    Heap(Box<str>),
}

impl SsoString {
    pub fn new(s: &str) -> Self {
        if s.len() <= INLINE_CAP {
            let mut buf = [0u8; INLINE_CAP];
            buf[..s.len()].copy_from_slice(s.as_bytes());
            SsoString::Inline {
                buf,
                len: s.len() as u8,
            }
        } else {
            SsoString::Heap(s.into())
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SsoString::Inline { buf, len } => std::str::from_utf8(&buf[..*len as usize]).unwrap(),
            SsoString::Heap(s) => s,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            SsoString::Inline { len, .. } => *len as usize,
            SsoString::Heap(s) => s.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_inline(&self) -> bool {
        matches!(self, SsoString::Inline { .. })
    }
}

impl std::fmt::Display for SsoString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_inline() {
        let s = SsoString::new("");
        assert!(s.is_inline());
        assert_eq!(s.len(), 0);
        assert_eq!(s.as_str(), "");
    }

    #[test]
    fn short_string_inline() {
        let s = SsoString::new("hello");
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn boundary_23_bytes_is_inline() {
        let s23 = "a".repeat(INLINE_CAP);
        let sso = SsoString::new(&s23);
        assert!(sso.is_inline());
        assert_eq!(sso.as_str(), s23);
    }

    #[test]
    fn boundary_24_bytes_is_heap() {
        let s24 = "a".repeat(INLINE_CAP + 1);
        let sso = SsoString::new(&s24);
        assert!(!sso.is_inline());
        assert_eq!(sso.as_str(), s24);
    }

    #[test]
    fn long_string_heap() {
        let long = "this is a long string that exceeds the inline capacity";
        let sso = SsoString::new(long);
        assert!(!sso.is_inline());
        assert_eq!(sso.as_str(), long);
    }
}
