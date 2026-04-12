#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A Huffman tree: either a leaf (character + frequency)
/// or an internal node with left/right subtrees and combined frequency.
#[derive(Debug, Clone)]
pub enum HTree {
    Leaf {
        ch: char,
        freq: u32,
    },
    Node {
        left: Box<HTree>,
        right: Box<HTree>,
        freq: u32,
    },
}

impl HTree {
    /// Extracts the frequency from any tree node.
    pub fn freq(&self) -> u32 {
        match self {
            Self::Leaf { freq, .. } | Self::Node { freq, .. } => *freq,
        }
    }
}

/// Wrapper for `HTree` that provides min-heap ordering by frequency.
/// BinaryHeap is a max-heap; reversing the comparison makes it behave as a min-heap.
struct MinFreq(HTree);

impl PartialEq for MinFreq {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq() == other.0.freq()
    }
}
impl Eq for MinFreq {}

impl PartialOrd for MinFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse: lower frequency = higher priority
        other.0.freq().cmp(&self.0.freq())
    }
}

/// Build a Huffman tree from (char, frequency) pairs.
///
/// Solution 1: Idiomatic Rust — uses `BinaryHeap` as a min-heap (O(n log n)).
/// Each step pops the two lowest-frequency trees and merges them.
pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }

    let mut heap: BinaryHeap<MinFreq> = freqs
        .iter()
        .map(|&(ch, freq)| MinFreq(HTree::Leaf { ch, freq }))
        .collect();

    while heap.len() > 1 {
        // Safe: checked heap.len() > 1 above
        let a = heap.pop()?.0;
        let b = heap.pop()?.0;
        let freq = a.freq() + b.freq();
        heap.push(MinFreq(HTree::Node {
            left: Box::new(a),
            right: Box::new(b),
            freq,
        }));
    }

    heap.pop().map(|entry| entry.0)
}

/// Build a Huffman tree using a sorted Vec — mirrors the OCaml implementation.
///
/// Solution 2: Functional/recursive — sorts the list after each merge,
/// exactly as the OCaml `go` function does.
pub fn build_tree_functional(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }

    let mut trees: Vec<HTree> = freqs
        .iter()
        .map(|&(ch, freq)| HTree::Leaf { ch, freq })
        .collect();
    trees.sort_by_key(HTree::freq);

    Some(go(trees))
}

fn go(mut trees: Vec<HTree>) -> HTree {
    match trees.len() {
        0 => panic!("empty frequency list"),
        1 => trees.remove(0),
        _ => {
            // Take the two lowest-frequency trees (front of sorted vec)
            let a = trees.remove(0);
            let b = trees.remove(0);
            let freq = a.freq() + b.freq();
            let merged = HTree::Node {
                left: Box::new(a),
                right: Box::new(b),
                freq,
            };
            trees.push(merged);
            trees.sort_by_key(HTree::freq);
            go(trees)
        }
    }
}

/// Recursively generate prefix codes for each character in the tree.
/// Left branches append "0", right branches append "1".
/// Call with an empty `prefix` to get full codes from the root.
pub fn codes(tree: &HTree, prefix: &str) -> Vec<(char, String)> {
    match tree {
        HTree::Leaf { ch, .. } => vec![(*ch, prefix.to_string())],
        HTree::Node { left, right, .. } => {
            let mut result = codes(left, &format!("{prefix}0"));
            result.extend(codes(right, &format!("{prefix}1")));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_freqs() -> Vec<(char, u32)> {
        vec![
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ]
    }

    fn codes_map(tree: &HTree) -> std::collections::HashMap<char, String> {
        codes(tree, "").into_iter().collect()
    }

    #[test]
    fn test_empty_returns_none() {
        assert_eq!(build_tree(&[]).is_none(), true);
        assert_eq!(build_tree_functional(&[]).is_none(), true);
    }

    #[test]
    fn test_single_leaf_gets_empty_code() {
        let tree = build_tree(&[('x', 10)]).unwrap();
        let result = codes(&tree, "");
        assert_eq!(result, vec![('x', String::new())]);
    }

    #[test]
    fn test_two_symbols_get_single_bit_codes() {
        let tree = build_tree(&[('a', 3), ('b', 7)]).unwrap();
        let map = codes_map(&tree);
        assert_eq!(map.len(), 2);
        // Both codes must be exactly 1 bit
        assert_eq!(map[&'a'].len(), 1);
        assert_eq!(map[&'b'].len(), 1);
        // And they must differ
        assert_ne!(map[&'a'], map[&'b']);
    }

    #[test]
    fn test_classic_example_all_chars_encoded() {
        let tree = build_tree(&sample_freqs()).unwrap();
        let result = codes(&tree, "");
        assert_eq!(result.len(), 6);
        // Every character from the input must appear exactly once
        let chars: std::collections::HashSet<char> = result.iter().map(|(c, _)| *c).collect();
        assert!(chars.contains(&'a'));
        assert!(chars.contains(&'f'));
    }

    #[test]
    fn test_most_frequent_gets_shortest_code() {
        // 'f' has freq 45 — in this classic example it gets a 1-bit code
        let tree = build_tree(&sample_freqs()).unwrap();
        let map = codes_map(&tree);
        let f_len = map[&'f'].len();
        // 'f' must have the shortest code (or tied for shortest)
        let min_len = map.values().map(|c| c.len()).min().unwrap();
        assert_eq!(f_len, min_len);
    }

    #[test]
    fn test_less_frequent_gets_longer_code() {
        // 'a' (freq 5) must have a longer code than 'f' (freq 45)
        let tree = build_tree(&sample_freqs()).unwrap();
        let map = codes_map(&tree);
        assert!(map[&'a'].len() > map[&'f'].len());
    }

    #[test]
    fn test_functional_and_heap_produce_same_code_lengths() {
        let freqs = sample_freqs();
        let tree_heap = build_tree(&freqs).unwrap();
        let tree_func = build_tree_functional(&freqs).unwrap();

        let mut heap_codes: Vec<_> = codes(&tree_heap, "")
            .into_iter()
            .map(|(c, s)| (c, s.len()))
            .collect();
        let mut func_codes: Vec<_> = codes(&tree_func, "")
            .into_iter()
            .map(|(c, s)| (c, s.len()))
            .collect();

        heap_codes.sort_by_key(|(c, _)| *c);
        func_codes.sort_by_key(|(c, _)| *c);
        assert_eq!(heap_codes, func_codes);
    }

    #[test]
    fn test_freq_extraction() {
        let leaf = HTree::Leaf { ch: 'x', freq: 42 };
        assert_eq!(leaf.freq(), 42);

        let node = HTree::Node {
            left: Box::new(HTree::Leaf { ch: 'a', freq: 10 }),
            right: Box::new(HTree::Leaf { ch: 'b', freq: 20 }),
            freq: 30,
        };
        assert_eq!(node.freq(), 30);
    }

    #[test]
    fn test_codes_are_prefix_free() {
        // No code should be a prefix of another (fundamental Huffman property)
        let tree = build_tree(&sample_freqs()).unwrap();
        let result = codes(&tree, "");
        for (i, (_, code_a)) in result.iter().enumerate() {
            for (j, (_, code_b)) in result.iter().enumerate() {
                if i != j {
                    assert!(
                        !code_b.starts_with(code_a.as_str()),
                        "{code_a} is a prefix of {code_b}"
                    );
                }
            }
        }
    }
}
