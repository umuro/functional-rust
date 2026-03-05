// 978: Count-Min Sketch
// Frequency estimation: O(1) update/query, O(depth × width) space
// Uses d hash functions × w counters; estimate = min over d rows

// Approach 1: Count-Min Sketch with multiple hash seeds
fn hash(seed: u64, s: &str) -> u64 {
    s.bytes().fold(seed, |h, b| h.wrapping_mul(seed).wrapping_add(b as u64) ^ b as u64)
}

pub struct CountMinSketch {
    table: Vec<Vec<u64>>,  // depth rows × width cols
    seeds: Vec<u64>,
    width: usize,
    depth: usize,
}

impl CountMinSketch {
    pub fn new(width: usize, depth: usize) -> Self {
        let seeds = vec![31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
        let depth_seeds: Vec<u64> = (0..depth).map(|i| seeds[i % seeds.len()]).collect();
        CountMinSketch {
            table: vec![vec![0u64; width]; depth],
            seeds: depth_seeds,
            width,
            depth,
        }
    }

    fn column(&self, row: usize, key: &str) -> usize {
        (hash(self.seeds[row], key) as usize) % self.width
    }

    /// Increment count for key by delta
    pub fn update(&mut self, key: &str, delta: u64) {
        for i in 0..self.depth {
            let col = self.column(i, key);
            self.table[i][col] += delta;
        }
    }

    /// Estimate frequency: minimum over all rows
    pub fn query(&self, key: &str) -> u64 {
        (0..self.depth)
            .map(|i| self.table[i][self.column(i, key)])
            .min()
            .unwrap_or(0)
    }

    /// Total events tracked (sum of row 0, approximate)
    pub fn total(&self) -> u64 {
        self.table[0].iter().sum()
    }
}

// Approach 2: Heavy Hitter tracking with Count-Min
pub struct FrequencyTracker {
    sketch: CountMinSketch,
    total_events: u64,
}

impl FrequencyTracker {
    pub fn new(width: usize, depth: usize) -> Self {
        FrequencyTracker {
            sketch: CountMinSketch::new(width, depth),
            total_events: 0,
        }
    }

    pub fn add(&mut self, key: &str) {
        self.sketch.update(key, 1);
        self.total_events += 1;
    }

    pub fn estimate_count(&self, key: &str) -> u64 {
        self.sketch.query(key)
    }

    pub fn estimate_frequency(&self, key: &str) -> f64 {
        if self.total_events == 0 {
            return 0.0;
        }
        self.estimate_count(key) as f64 / self.total_events as f64
    }

    pub fn total(&self) -> u64 {
        self.total_events
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_underestimate() {
        let mut sk = CountMinSketch::new(100, 5);
        for _ in 0..100 { sk.update("apple", 1); }
        for _ in 0..50  { sk.update("banana", 1); }
        for _ in 0..25  { sk.update("cherry", 1); }

        // Count-Min never underestimates
        assert!(sk.query("apple") >= 100);
        assert!(sk.query("banana") >= 50);
        assert!(sk.query("cherry") >= 25);
    }

    #[test]
    fn test_unseen_items_low() {
        let mut sk = CountMinSketch::new(1000, 5);
        for i in 0..100 { sk.update(&format!("item_{}", i), 1); }

        // Unseen items should have very low count
        let unseen = sk.query("completely_unseen_item_xyz");
        assert!(unseen < 5, "unseen item count too high: {}", unseen);
    }

    #[test]
    fn test_batch_update() {
        let mut sk = CountMinSketch::new(100, 4);
        sk.update("key", 100);
        assert!(sk.query("key") >= 100);
    }

    #[test]
    fn test_frequency_tracker() {
        let mut tracker = FrequencyTracker::new(200, 4);
        for _ in 0..900 { tracker.add("hot"); }
        for _ in 0..100 { tracker.add("cold"); }

        assert_eq!(tracker.total(), 1000);
        let hot_freq = tracker.estimate_frequency("hot");
        assert!(hot_freq >= 0.9, "hot frequency {:.3} should be >= 0.9", hot_freq);
        let cold_freq = tracker.estimate_frequency("cold");
        assert!(cold_freq >= 0.1, "cold frequency {:.3} should be >= 0.1", cold_freq);
    }

    #[test]
    fn test_multiple_deltas() {
        let mut sk = CountMinSketch::new(100, 4);
        sk.update("x", 50);
        sk.update("x", 50);
        assert!(sk.query("x") >= 100);
    }
}
