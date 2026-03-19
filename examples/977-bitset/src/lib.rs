// 977: Bitset
// Set, clear, toggle, union, intersection via u64 word arrays
// OCaml uses 63-bit int (1 bit used for GC tag); Rust uses full u64

pub struct Bitset {
    bits: Vec<u64>,
    size: usize,
}

impl Bitset {
    pub fn new(size: usize) -> Self {
        let words = (size + 63) / 64;
        Bitset {
            bits: vec![0u64; words],
            size,
        }
    }

    fn word(&self, i: usize) -> usize {
        i / 64
    }
    fn bit(&self, i: usize) -> u64 {
        1u64 << (i % 64)
    }

    pub fn set(&mut self, i: usize) {
        assert!(
            i < self.size,
            "index {} out of range (size={})",
            i,
            self.size
        );
        let (w, b) = (self.word(i), self.bit(i));
        self.bits[w] |= b;
    }

    pub fn clear(&mut self, i: usize) {
        assert!(i < self.size, "index out of range");
        let (w, b) = (self.word(i), self.bit(i));
        self.bits[w] &= !b;
    }

    pub fn toggle(&mut self, i: usize) {
        assert!(i < self.size, "index out of range");
        let (w, b) = (self.word(i), self.bit(i));
        self.bits[w] ^= b;
    }

    pub fn test(&self, i: usize) -> bool {
        if i >= self.size {
            return false;
        }
        (self.bits[self.word(i)] >> (i % 64)) & 1 == 1
    }

    /// Count of set bits (popcount)
    pub fn count(&self) -> u32 {
        self.bits.iter().map(|w| w.count_ones()).sum()
    }

    /// Union: returns new bitset with bits set in either
    pub fn union(&self, other: &Bitset) -> Bitset {
        assert_eq!(self.size, other.size);
        let bits = self
            .bits
            .iter()
            .zip(&other.bits)
            .map(|(a, b)| a | b)
            .collect();
        Bitset {
            bits,
            size: self.size,
        }
    }

    /// Intersection: bits set in both
    pub fn intersect(&self, other: &Bitset) -> Bitset {
        assert_eq!(self.size, other.size);
        let bits = self
            .bits
            .iter()
            .zip(&other.bits)
            .map(|(a, b)| a & b)
            .collect();
        Bitset {
            bits,
            size: self.size,
        }
    }

    /// Difference: bits in self but not other
    pub fn difference(&self, other: &Bitset) -> Bitset {
        assert_eq!(self.size, other.size);
        let bits = self
            .bits
            .iter()
            .zip(&other.bits)
            .map(|(a, b)| a & !b)
            .collect();
        Bitset {
            bits,
            size: self.size,
        }
    }

    /// Symmetric difference: bits in one but not both
    pub fn symmetric_difference(&self, other: &Bitset) -> Bitset {
        assert_eq!(self.size, other.size);
        let bits = self
            .bits
            .iter()
            .zip(&other.bits)
            .map(|(a, b)| a ^ b)
            .collect();
        Bitset {
            bits,
            size: self.size,
        }
    }

    /// Iterate set bit indices
    pub fn iter_ones(&self) -> Vec<usize> {
        let mut result = vec![];
        for (w_idx, &word) in self.bits.iter().enumerate() {
            let mut word = word;
            while word != 0 {
                let bit = word.trailing_zeros() as usize;
                result.push(w_idx * 64 + bit);
                word &= word - 1; // clear lowest set bit
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_test_clear() {
        let mut bs = Bitset::new(128);
        bs.set(0);
        bs.set(5);
        bs.set(63);
        bs.set(64);
        bs.set(127);

        assert!(bs.test(0));
        assert!(bs.test(5));
        assert!(bs.test(63));
        assert!(bs.test(64));
        assert!(bs.test(127));
        assert!(!bs.test(1));
        assert_eq!(bs.count(), 5);

        bs.clear(5);
        assert!(!bs.test(5));
        assert_eq!(bs.count(), 4);
    }

    #[test]
    fn test_toggle() {
        let mut bs = Bitset::new(64);
        bs.toggle(10);
        assert!(bs.test(10));
        bs.toggle(10);
        assert!(!bs.test(10));
    }

    #[test]
    fn test_union() {
        let mut a = Bitset::new(64);
        let mut b = Bitset::new(64);
        for i in 0..4 {
            a.set(i);
        }
        for i in 2..6 {
            b.set(i);
        }
        let u = a.union(&b);
        assert_eq!(u.count(), 6);
        assert_eq!(u.iter_ones(), vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_intersect() {
        let mut a = Bitset::new(64);
        let mut b = Bitset::new(64);
        for i in 0..4 {
            a.set(i);
        }
        for i in 2..6 {
            b.set(i);
        }
        let i = a.intersect(&b);
        assert_eq!(i.count(), 2);
        assert!(i.test(2));
        assert!(i.test(3));
    }

    #[test]
    fn test_difference() {
        let mut a = Bitset::new(64);
        let mut b = Bitset::new(64);
        for i in 0..4 {
            a.set(i);
        }
        for i in 2..6 {
            b.set(i);
        }
        let d = a.difference(&b);
        assert_eq!(d.count(), 2);
        assert_eq!(d.iter_ones(), vec![0, 1]);
    }

    #[test]
    fn test_iter_ones() {
        let mut bs = Bitset::new(200);
        bs.set(0);
        bs.set(63);
        bs.set(64);
        bs.set(127);
        bs.set(128);
        bs.set(199);
        assert_eq!(bs.iter_ones(), vec![0, 63, 64, 127, 128, 199]);
    }
}
