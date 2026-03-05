// 963: Bloom Filter
// Probabilistic membership test: no false negatives, possible false positives
// Uses 3 independent hash functions + bit array (u64 words)

// Approach 1: Three hash functions (djb2, sdbm, fnv-like)
fn hash1(s: &str) -> usize {
    s.bytes().fold(5381usize, |h, b| h.wrapping_mul(31).wrapping_add(b as usize))
}

fn hash2(s: &str) -> usize {
    s.bytes().fold(0usize, |h, b| {
        (b as usize)
            .wrapping_add(h.wrapping_shl(6))
            .wrapping_add(h.wrapping_shl(16))
            .wrapping_sub(h)
    })
}

fn hash3(s: &str) -> usize {
    s.bytes().fold(0usize, |h, b| h.wrapping_mul(33) ^ (b as usize))
}

// Approach 2: Bloom filter with u64 bit array
pub struct BloomFilter {
    bits: Vec<u64>,
    num_bits: usize,
}

impl BloomFilter {
    pub fn new(num_bits: usize) -> Self {
        let words = (num_bits + 63) / 64;
        BloomFilter {
            bits: vec![0u64; words],
            num_bits,
        }
    }

    fn set_bit(&mut self, i: usize) {
        let idx = i % self.num_bits;
        let word = idx / 64;
        let bit = idx % 64;
        self.bits[word] |= 1u64 << bit;
    }

    fn get_bit(&self, i: usize) -> bool {
        let idx = i % self.num_bits;
        let word = idx / 64;
        let bit = idx % 64;
        (self.bits[word] >> bit) & 1 == 1
    }

    pub fn add(&mut self, s: &str) {
        self.set_bit(hash1(s));
        self.set_bit(hash2(s));
        self.set_bit(hash3(s));
    }

    pub fn might_contain(&self, s: &str) -> bool {
        self.get_bit(hash1(s)) && self.get_bit(hash2(s)) && self.get_bit(hash3(s))
    }

    pub fn count_set_bits(&self) -> u32 {
        self.bits.iter().map(|w| w.count_ones()).sum()
    }

    /// Estimated false positive rate given n items inserted
    pub fn false_positive_rate(&self, n: usize) -> f64 {
        let k = 3.0_f64; // number of hash functions
        let m = self.num_bits as f64;
        (1.0 - (-k * n as f64 / m).exp()).powf(k)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inserted_items_found() {
        let mut bf = BloomFilter::new(1024);
        let words = ["apple", "banana", "cherry", "dog", "elephant"];
        for w in &words {
            bf.add(w);
        }
        for w in &words {
            assert!(bf.might_contain(w), "must contain '{}'", w);
        }
    }

    #[test]
    fn test_false_positive_rate_reasonable() {
        let mut bf = BloomFilter::new(10_000);
        for i in 0..100 {
            bf.add(&format!("item_{}", i));
        }
        // Check 1000 non-inserted items, count false positives
        let fp: usize = (0..1000)
            .filter(|i| bf.might_contain(&format!("not_inserted_{}", i)))
            .count();
        // With 10000 bits, 3 hashes, 100 items → FP rate ~ 0.001
        // Allow up to 5% false positives in test
        assert!(fp < 50, "too many false positives: {}/1000", fp);
    }

    #[test]
    fn test_bit_operations() {
        let mut bf = BloomFilter::new(128);
        bf.set_bit(0);
        bf.set_bit(63);
        bf.set_bit(64);
        bf.set_bit(127);
        assert!(bf.get_bit(0));
        assert!(bf.get_bit(63));
        assert!(bf.get_bit(64));
        assert!(bf.get_bit(127));
        assert!(!bf.get_bit(1));
        assert!(!bf.get_bit(100));
    }

    #[test]
    fn test_count_set_bits() {
        let mut bf = BloomFilter::new(64);
        assert_eq!(bf.count_set_bits(), 0);
        bf.add("test");
        assert!(bf.count_set_bits() > 0);
        assert!(bf.count_set_bits() <= 3);
    }
}
