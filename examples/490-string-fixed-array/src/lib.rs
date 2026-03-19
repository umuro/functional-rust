//! # String Fixed Array — Stack-Allocated Strings
//!
//! Fixed-size strings without heap allocation.

/// Fixed-size string buffer
#[derive(Clone, Copy)]
pub struct FixedString<const N: usize> {
    buffer: [u8; N],
    len: usize,
}

impl<const N: usize> FixedString<N> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            len: 0,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() > N {
            return None;
        }
        let mut fs = Self::new();
        fs.buffer[..s.len()].copy_from_slice(s.as_bytes());
        fs.len = s.len();
        Some(fs)
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buffer[..self.len]).unwrap()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn push_str(&mut self, s: &str) -> bool {
        if self.len + s.len() > N {
            return false;
        }
        self.buffer[self.len..self.len + s.len()].copy_from_slice(s.as_bytes());
        self.len += s.len();
        true
    }

    pub fn push(&mut self, c: char) -> bool {
        let mut buf = [0u8; 4];
        let s = c.encode_utf8(&mut buf);
        self.push_str(s)
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<const N: usize> Default for FixedString<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> std::fmt::Display for FixedString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> std::fmt::Debug for FixedString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FixedString<{}>({:?})", N, self.as_str())
    }
}

/// Type aliases for common sizes
pub type String16 = FixedString<16>;
pub type String32 = FixedString<32>;
pub type String64 = FixedString<64>;
pub type String256 = FixedString<256>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let s = String32::from_str("hello").unwrap();
        assert_eq!(s.as_str(), "hello");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_too_long() {
        let result = String16::from_str("this string is way too long");
        assert!(result.is_none());
    }

    #[test]
    fn test_push_str() {
        let mut s = String32::new();
        assert!(s.push_str("hello"));
        assert!(s.push_str(" "));
        assert!(s.push_str("world"));
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_push_char() {
        let mut s = String16::new();
        s.push('H');
        s.push('i');
        assert_eq!(s.as_str(), "Hi");
    }

    #[test]
    fn test_clear() {
        let mut s = String32::from_str("hello").unwrap();
        s.clear();
        assert!(s.is_empty());
    }

    #[test]
    fn test_capacity() {
        let s = String64::new();
        assert_eq!(s.capacity(), 64);
    }

    #[test]
    fn test_stack_allocated() {
        // Verify it fits on stack
        let s = String256::from_str("stack allocated").unwrap();
        assert_eq!(
            std::mem::size_of_val(&s),
            256 + std::mem::size_of::<usize>()
        );
    }
}
