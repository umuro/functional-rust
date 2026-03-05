//! # Const Generic Struct
//!
//! Structures parameterized by compile-time constants.

/// Fixed-capacity ring buffer
#[derive(Debug)]
pub struct RingBuffer<T, const CAP: usize> {
    data: [Option<T>; CAP],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T: Copy, const CAP: usize> RingBuffer<T, CAP> {
    pub fn new() -> Self {
        RingBuffer {
            data: [None; CAP],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub const fn capacity(&self) -> usize {
        CAP
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn is_full(&self) -> bool {
        self.len == CAP
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            return Err(item);
        }
        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % CAP;
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.data[self.head].take();
        self.head = (self.head + 1) % CAP;
        self.len -= 1;
        item
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.head].as_ref()
        }
    }
}

impl<T: Copy, const CAP: usize> Default for RingBuffer<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

/// Bit set with compile-time size (stored as array of u64 words)
/// The number of u64 words is passed as a const generic WORDS parameter.
#[derive(Debug, Clone, Copy)]
pub struct BitSet<const BITS: usize, const WORDS: usize> {
    data: [u64; WORDS],
}

impl<const BITS: usize, const WORDS: usize> BitSet<BITS, WORDS> {
    pub fn new() -> Self {
        BitSet { data: [0; WORDS] }
    }

    pub const fn bits(&self) -> usize {
        BITS
    }

    pub fn set(&mut self, idx: usize) {
        if idx < BITS {
            self.data[idx / 64] |= 1 << (idx % 64);
        }
    }

    pub fn clear(&mut self, idx: usize) {
        if idx < BITS {
            self.data[idx / 64] &= !(1 << (idx % 64));
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        if idx < BITS {
            (self.data[idx / 64] >> (idx % 64)) & 1 != 0
        } else {
            false
        }
    }

    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|&x| x.count_ones() as usize).sum()
    }
}

impl<const BITS: usize, const WORDS: usize> Default for BitSet<BITS, WORDS> {
    fn default() -> Self {
        Self::new()
    }
}

/// Fixed-size string
#[derive(Debug, Clone, Copy)]
pub struct FixedString<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> FixedString<N> {
    pub const fn new() -> Self {
        FixedString { data: [0; N], len: 0 }
    }

    pub const fn capacity(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.data[..self.len]).unwrap_or("")
    }

    pub fn push_str(&mut self, s: &str) -> bool {
        let bytes = s.as_bytes();
        if self.len + bytes.len() <= N {
            self.data[self.len..self.len + bytes.len()].copy_from_slice(bytes);
            self.len += bytes.len();
            true
        } else {
            false
        }
    }
}

impl<const N: usize> Default for FixedString<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer() {
        let mut rb: RingBuffer<i32, 4> = RingBuffer::new();
        assert_eq!(rb.capacity(), 4);
        
        rb.push(1).unwrap();
        rb.push(2).unwrap();
        assert_eq!(rb.pop(), Some(1));
        assert_eq!(rb.pop(), Some(2));
    }

    #[test]
    fn test_ring_buffer_wrap() {
        let mut rb: RingBuffer<i32, 3> = RingBuffer::new();
        rb.push(1).unwrap();
        rb.push(2).unwrap();
        rb.push(3).unwrap();
        assert!(rb.push(4).is_err()); // Full
        
        assert_eq!(rb.pop(), Some(1));
        rb.push(4).unwrap(); // Now there's room
        assert_eq!(rb.pop(), Some(2));
    }

    #[test]
    fn test_bitset() {
        // BitSet<100, 2> means 100 bits stored in 2 u64 words (128 bits capacity)
        let mut bs: BitSet<100, 2> = BitSet::new();
        assert_eq!(bs.bits(), 100);
        
        bs.set(0);
        bs.set(50);
        bs.set(99);
        
        assert!(bs.get(0));
        assert!(bs.get(50));
        assert!(bs.get(99));
        assert!(!bs.get(1));
        assert_eq!(bs.count_ones(), 3);
    }

    #[test]
    fn test_fixed_string() {
        let mut s: FixedString<16> = FixedString::new();
        assert!(s.push_str("hello"));
        assert_eq!(s.as_str(), "hello");
        assert!(s.push_str(" world"));
        assert_eq!(s.as_str(), "hello world");
    }
}
