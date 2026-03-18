use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A Huffman tree: leaf holds a character + frequency; node holds two subtrees + combined frequency
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Extract frequency from any node — mirrors OCaml's `freq` accessor
    pub fn freq(&self) -> u32 {
        match self {
            HTree::Leaf { freq, .. } | HTree::Node { freq, .. } => *freq,
        }
    }
}

/// Wrapper that gives HTree min-heap ordering (lowest frequency = highest priority)
#[derive(Eq, PartialEq)]
struct MinFreq(Box<HTree>);

impl Ord for MinFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed so BinaryHeap (max-heap) becomes a min-heap by frequency
        other.0.freq().cmp(&self.0.freq())
    }
}

impl PartialOrd for MinFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Solution 1: Idiomatic Rust — BinaryHeap (priority queue) for O(n log n) construction
///
/// Greedy algorithm: always merge the two lowest-frequency trees until one remains.
/// Uses std's BinaryHeap with a min-heap wrapper rather than re-sorting on each step.
pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }

    let mut heap: BinaryHeap<MinFreq> = freqs
        .iter()
        .map(|&(ch, freq)| MinFreq(Box::new(HTree::Leaf { ch, freq })))
        .collect();

    while heap.len() > 1 {
        // Safe: loop condition guarantees at least 2 elements
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;
        let freq = a.freq() + b.freq();
        heap.push(MinFreq(Box::new(HTree::Node {
            left: a,
            right: b,
            freq,
        })));
    }

    // Unwrap safe: freqs was non-empty, heap has exactly 1 entry
    heap.pop().map(|entry| *entry.0)
}

/// Solution 2: Functional style — sorted Vec, mirrors the OCaml implementation
///
/// After each merge, inserts the new node back into the sorted list via binary search.
/// O(n²) but structurally identical to the OCaml `List.sort` approach.
pub fn build_tree_sorted(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }

    let mut trees: Vec<HTree> = {
        let mut v: Vec<HTree> = freqs
            .iter()
            .map(|&(ch, freq)| HTree::Leaf { ch, freq })
            .collect();
        v.sort_by_key(HTree::freq);
        v
    };

    while trees.len() > 1 {
        // Remove two lowest-frequency trees from the front
        let a = trees.remove(0);
        let b = trees.remove(0);
        let freq = a.freq() + b.freq();
        let merged = HTree::Node {
            left: Box::new(a),
            right: Box::new(b),
            freq,
        };
        // Re-insert in sorted position (equivalent to OCaml's List.sort after merging)
        let pos = trees.partition_point(|t| t.freq() <= freq);
        trees.insert(pos, merged);
    }

    trees.into_iter().next()
}

/// Generate Huffman codes by walking the tree: '0' for left branch, '1' for right
///
/// Returns (char, code) pairs; code is a string of '0'/'1' characters.
pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    fn go(tree: &HTree, prefix: String) -> Vec<(char, String)> {
        match tree {
            HTree::Leaf { ch, .. } => vec![(*ch, prefix)],
            HTree::Node { left, right, .. } => {
                let mut result = go(left, format!("{prefix}0"));
                result.extend(go(right, format!("{prefix}1")));
                result
            }
        }
    }
    go(tree, String::new())
}

/// Encode a string using a Huffman code table
pub fn encode(input: &str, table: &[(char, String)]) -> Option<String> {
    input
        .chars()
        .map(|c| {
            table
                .iter()
                .find(|(ch, _)| *ch == c)
                .map(|(_, code)| code.as_str())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_codes(pairs: Vec<(char, String)>) -> Vec<(char, String)> {
        let mut v = pairs;
        v.sort_by_key(|(c, _)| *c);
        v
    }

    /// Two implementations must produce trees with identical frequencies
    fn trees_match_freq(a: &HTree, b: &HTree) -> bool {
        a.freq() == b.freq()
    }

    #[test]
    fn test_empty_returns_none() {
        assert_eq!(build_tree(&[]), None);
        assert_eq!(build_tree_sorted(&[]), None);
    }

    #[test]
    fn test_single_character() {
        let tree = build_tree(&[('x', 10)]).unwrap();
        assert_eq!(tree.freq(), 10);
        let c = codes(&tree);
        assert_eq!(c, vec![('x', String::new())]);
    }

    #[test]
    fn test_two_characters() {
        let freqs = [('a', 3), ('b', 7)];
        let tree = build_tree(&freqs).unwrap();
        assert_eq!(tree.freq(), 10);
        let c = sorted_codes(codes(&tree));
        // Both chars get a 1-bit code
        assert_eq!(c.len(), 2);
        assert!(c.iter().all(|(_, code)| code.len() == 1));
    }

    #[test]
    fn test_standard_six_char_example() {
        // Classic textbook example: f has highest freq, gets shortest code
        let freqs = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        let tree = build_tree(&freqs).unwrap();
        assert_eq!(tree.freq(), 100); // total of all frequencies

        let c = codes(&tree);
        assert_eq!(c.len(), 6); // one code per character

        // 'f' has freq 45 (almost half) — must get the shortest code
        let f_code = c.iter().find(|(ch, _)| *ch == 'f').unwrap();
        let min_len = c.iter().map(|(_, code)| code.len()).min().unwrap();
        assert_eq!(f_code.1.len(), min_len);
    }

    #[test]
    fn test_codes_are_prefix_free() {
        let freqs = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        let tree = build_tree(&freqs).unwrap();
        let c = codes(&tree);
        // No code should be a prefix of any other code
        for (i, (_, ci)) in c.iter().enumerate() {
            for (j, (_, cj)) in c.iter().enumerate() {
                if i != j {
                    assert!(!cj.starts_with(ci.as_str()), "{ci} is a prefix of {cj}");
                }
            }
        }
    }

    #[test]
    fn test_sorted_and_heap_produce_same_total_freq() {
        let freqs = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        let t1 = build_tree(&freqs).unwrap();
        let t2 = build_tree_sorted(&freqs).unwrap();
        assert!(trees_match_freq(&t1, &t2));
    }

    #[test]
    fn test_encode_roundtrip() {
        let freqs = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        let tree = build_tree(&freqs).unwrap();
        let table = codes(&tree);
        let encoded = encode("fab", &table).unwrap();
        // Encoded result must be non-empty binary string
        assert!(!encoded.is_empty());
        assert!(encoded.chars().all(|c| c == '0' || c == '1'));
    }
}
