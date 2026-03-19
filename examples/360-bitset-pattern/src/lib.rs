//! # BitSet Pattern
//! Efficient set operations using bit manipulation.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitSet64(u64);

impl BitSet64 {
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn insert(&mut self, i: u32) {
        assert!(i < 64);
        self.0 |= 1u64 << i;
    }
    pub fn remove(&mut self, i: u32) {
        if i < 64 {
            self.0 &= !(1u64 << i);
        }
    }
    pub fn contains(&self, i: u32) -> bool {
        i < 64 && (self.0 >> i) & 1 == 1
    }
    pub fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }
    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }
    pub fn difference(&self, other: &Self) -> Self {
        Self(self.0 & !other.0)
    }
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }
    pub fn to_vec(&self) -> Vec<u32> {
        (0..64).filter(|&i| self.contains(i)).collect()
    }
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl Default for BitSet64 {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_contains() {
        let mut b = BitSet64::empty();
        b.insert(5);
        assert!(b.contains(5));
        assert!(!b.contains(4));
    }
    #[test]
    fn set_operations() {
        let mut a = BitSet64::empty();
        let mut b = BitSet64::empty();
        for i in [1, 2, 3] {
            a.insert(i);
        }
        for i in [2, 3, 4] {
            b.insert(i);
        }
        assert_eq!(a.intersection(&b).to_vec(), vec![2, 3]);
        assert_eq!(a.difference(&b).to_vec(), vec![1]);
    }
    #[test]
    fn count_ones() {
        let mut b = BitSet64::empty();
        for i in 0..10 {
            b.insert(i);
        }
        assert_eq!(b.count(), 10);
    }
}
