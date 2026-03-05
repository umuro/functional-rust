use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct BloomFilter {
    bits: Vec<u64>,
    m: usize, // number of bits
    k: usize, // number of hash functions
    count: usize,
}

impl BloomFilter {
    fn new(capacity: usize, fp_rate: f64) -> Self {
        // m = -n*ln(p) / (ln2)^2
        let m = (-(capacity as f64 * fp_rate.ln()) / (2f64.ln().powi(2))).ceil() as usize;
        // k = (m/n) * ln2
        let k = ((m as f64 / capacity as f64) * 2f64.ln()).ceil() as usize;
        let m = m.max(64);
        let k = k.max(1);
        Self {
            bits: vec![0u64; (m + 63) / 64],
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

    fn set_bit(&mut self, i: usize) { self.bits[i/64] |= 1u64 << (i%64); }
    fn get_bit(&self, i: usize) -> bool { (self.bits[i/64] >> (i%64)) & 1 == 1 }

    fn insert<T: Hash>(&mut self, item: &T) {
        for seed in 0..self.k as u64 {
            let idx = self.hash_val(item, seed);
            self.set_bit(idx);
        }
        self.count += 1;
    }

    fn contains<T: Hash>(&self, item: &T) -> bool {
        (0..self.k as u64).all(|seed| self.get_bit(self.hash_val(item, seed)))
    }

    fn approx_false_positive_rate(&self) -> f64 {
        let bits_set = self.bits.iter().map(|b| b.count_ones() as f64).sum::<f64>();
        (bits_set / self.m as f64).powi(self.k as i32)
    }
}

fn main() {
    let mut bf = BloomFilter::new(1000, 0.01); // 1000 items, 1% FP rate
    println!("m={} bits, k={} hash functions", bf.m, bf.k);

    let words = ["alice","bob","charlie","diana","eve"];
    for w in &words { bf.insert(w); }

    for w in &words {
        println!("{w}: {}", bf.contains(w)); // should all be true
    }

    let not_present = ["frank","grace","heidi","ivan","judy"];
    let fps: usize = not_present.iter().filter(|&&w| bf.contains(&w)).count();
    println!("False positives: {fps}/{} (expected ~0-1)", not_present.len());
    println!("FP rate approx: {:.4}", bf.approx_false_positive_rate());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn no_false_negatives() {
        let mut bf = BloomFilter::new(100, 0.01);
        for i in 0..50 { bf.insert(&i); }
        for i in 0..50 { assert!(bf.contains(&i), "false negative for {i}"); }
    }
    #[test] fn not_inserted_usually_absent() {
        let mut bf = BloomFilter::new(1000, 0.001);
        for i in 0..100i32 { bf.insert(&i); }
        // With very low FP rate, most non-members should be absent
        let fp: usize = (1000..1100i32).filter(|x| bf.contains(x)).count();
        assert!(fp < 10, "too many false positives: {fp}");
    }
}
