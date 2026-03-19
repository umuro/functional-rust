#![allow(clippy::all)]
//! Bloom Filter
//!
//! Probabilistic data structure for set membership with no false negatives.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A Bloom filter for probabilistic set membership testing
pub struct BloomFilter {
    bits: Vec<u64>,
    m: usize,
    k: usize,
    count: usize,
}

impl BloomFilter {
    /// Create a new Bloom filter for given capacity and false positive rate
    pub fn new(capacity: usize, fp_rate: f64) -> Self {
        let m = (-(capacity as f64 * fp_rate.ln()) / (2f64.ln().powi(2))).ceil() as usize;
        let k = ((m as f64 / capacity as f64) * 2f64.ln()).ceil() as usize;
        let m = m.max(64);
        let k = k.max(1);
        Self {
            bits: vec![0u64; m.div_ceil(64)],
            m,
            k,
            count: 0,
        }
    }

    fn hash_val<T: Hash>(&self, item: &T, seed: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        item.hash(&mut hasher);
        (hasher.finish() as usize) % self.m
    }

    fn set_bit(&mut self, i: usize) {
        self.bits[i / 64] |= 1u64 << (i % 64);
    }

    fn get_bit(&self, i: usize) -> bool {
        (self.bits[i / 64] >> (i % 64)) & 1 == 1
    }

    /// Insert an item
    pub fn insert<T: Hash>(&mut self, item: &T) {
        for seed in 0..self.k as u64 {
            let idx = self.hash_val(item, seed);
            self.set_bit(idx);
        }
        self.count += 1;
    }

    /// Check if item might be in the set (may have false positives)
    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        (0..self.k as u64).all(|seed| self.get_bit(self.hash_val(item, seed)))
    }

    /// Get number of items inserted
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Get number of bits
    pub fn bits(&self) -> usize {
        self.m
    }

    /// Get number of hash functions
    pub fn hash_count(&self) -> usize {
        self.k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_contains() {
        let mut bf = BloomFilter::new(100, 0.01);
        bf.insert(&"hello");
        bf.insert(&"world");
        assert!(bf.contains(&"hello"));
        assert!(bf.contains(&"world"));
    }

    #[test]
    fn test_not_contains() {
        let bf = BloomFilter::new(100, 0.01);
        // Empty filter should not contain anything
        assert!(!bf.contains(&"missing"));
    }

    #[test]
    fn test_false_positive_rate() {
        let mut bf = BloomFilter::new(1000, 0.01);
        for i in 0..500 {
            bf.insert(&i);
        }
        // Check false positives for items not inserted
        let mut fp_count = 0;
        for i in 1000..2000 {
            if bf.contains(&i) {
                fp_count += 1;
            }
        }
        // Should be roughly 1% = 10 false positives out of 1000
        assert!(fp_count < 50); // allow some variance
    }

    #[test]
    fn test_len() {
        let mut bf = BloomFilter::new(100, 0.01);
        assert_eq!(bf.len(), 0);
        bf.insert(&1);
        bf.insert(&2);
        assert_eq!(bf.len(), 2);
    }

    #[test]
    fn test_parameters() {
        let bf = BloomFilter::new(1000, 0.01);
        assert!(bf.bits() > 0);
        assert!(bf.hash_count() > 0);
    }
}
