// 974: Skip List (Simplified)
// Probabilistic sorted structure: O(log n) average search/insert
// Uses raw indices instead of pointers (safer than raw pointers in Rust)

use std::collections::VecDeque;

const MAX_LEVEL: usize = 8;
const P: f64 = 0.5;

// Simple deterministic PRNG for reproducible tests (xorshift)
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self { Rng(seed) }
    fn next_f64(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 & 0xFFFF) as f64 / 0x10000 as f64
    }
    fn random_level(&mut self) -> usize {
        let mut level = 1;
        while level < MAX_LEVEL && self.next_f64() < P {
            level += 1;
        }
        level
    }
}

// Arena-based skip list: nodes stored in Vec, referenced by index
// 0 = header sentinel
struct SkipListNode {
    value: i64,
    forward: Vec<usize>, // indices into node arena; 0 = None (header is sentinel)
}

pub struct SkipList {
    nodes: Vec<SkipListNode>,
    level: usize,
    rng: Rng,
}

impl SkipList {
    pub fn new() -> Self {
        let header = SkipListNode {
            value: i64::MIN,
            forward: vec![0; MAX_LEVEL], // 0 = nil (self-loop = nil for header)
        };
        SkipList {
            nodes: vec![header],
            level: 0,
            rng: Rng::new(12345),
        }
    }

    fn is_nil(&self, idx: usize) -> bool {
        idx == 0 && self.nodes[0].forward[0] == 0
            || (idx == 0 && self.level == 0)
    }

    pub fn search(&self, target: i64) -> bool {
        let mut current = 0usize; // start at header
        for i in (0..self.level).rev() {
            loop {
                let next = self.nodes[current].forward[i];
                if next == 0 {
                    break;
                }
                if self.nodes[next].value < target {
                    current = next;
                } else {
                    break;
                }
            }
        }
        let next = self.nodes[current].forward[0];
        next != 0 && self.nodes[next].value == target
    }

    pub fn insert(&mut self, value: i64) {
        let mut update = vec![0usize; MAX_LEVEL];
        let mut current = 0usize;

        for i in (0..self.level).rev() {
            loop {
                let next = self.nodes[current].forward[i];
                if next == 0 || self.nodes[next].value >= value {
                    break;
                }
                current = next;
            }
            update[i] = current;
        }

        let new_level = self.rng.random_level();
        if new_level > self.level {
            for i in self.level..new_level {
                update[i] = 0; // header
            }
            self.level = new_level;
        }

        let new_idx = self.nodes.len();
        let mut new_node = SkipListNode {
            value,
            forward: vec![0; new_level],
        };

        for i in 0..new_level {
            new_node.forward[i] = self.nodes[update[i]].forward[i];
            self.nodes[update[i]].forward[i] = new_idx;
        }
        self.nodes.push(new_node);
    }

    pub fn to_vec(&self) -> Vec<i64> {
        let mut result = vec![];
        let mut current = self.nodes[0].forward[0];
        while current != 0 {
            result.push(self.nodes[current].value);
            current = self.nodes[current].forward[0];
        }
        result
    }
}

impl Default for SkipList {
    fn default() -> Self { Self::new() }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_order() {
        let mut sl = SkipList::new();
        for v in [5, 3, 7, 1, 9, 4, 6, 2, 8] {
            sl.insert(v);
        }
        assert_eq!(sl.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_search_found() {
        let mut sl = SkipList::new();
        for v in [5, 3, 7, 1, 9] {
            sl.insert(v);
        }
        assert!(sl.search(5));
        assert!(sl.search(1));
        assert!(sl.search(9));
    }

    #[test]
    fn test_search_not_found() {
        let mut sl = SkipList::new();
        for v in [5, 3, 7, 1, 9] {
            sl.insert(v);
        }
        assert!(!sl.search(0));
        assert!(!sl.search(10));
        assert!(!sl.search(4));
    }

    #[test]
    fn test_empty() {
        let sl = SkipList::new();
        assert_eq!(sl.to_vec(), Vec::<i64>::new());
        assert!(!sl.search(1));
    }

    #[test]
    fn test_single() {
        let mut sl = SkipList::new();
        sl.insert(42);
        assert_eq!(sl.to_vec(), vec![42]);
        assert!(sl.search(42));
        assert!(!sl.search(43));
    }
}
