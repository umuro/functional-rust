#![allow(dead_code)]

use std::cmp::Ordering;

/// A Huffman tree node: either a Leaf (character + frequency) or an internal Node.
#[derive(Debug, Clone)]
pub enum HTree {
    Leaf(char, u32),
    Node(Box<HTree>, Box<HTree>, u32),
}

impl HTree {
    pub fn freq(&self) -> u32 {
        match self {
            HTree::Leaf(_, f) => *f,
            HTree::Node(_, _, f) => *f,
        }
    }
}

/// Heap entry: orders by frequency (min), breaking ties by insertion counter (FIFO).
struct Entry {
    freq: u32,
    seq: usize,
    tree: HTree,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.seq == other.seq
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// BinaryHeap is a max-heap; invert the comparison to get a min-heap by frequency.
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .freq
            .cmp(&self.freq)
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

/// Idiomatic Rust: build a Huffman tree using a BinaryHeap (min-heap via custom Ord).
/// O(n log n) — avoids re-sorting on every merge step.
pub fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree> {
    use std::collections::BinaryHeap;

    let mut heap: BinaryHeap<Entry> = freqs
        .iter()
        .enumerate()
        .map(|(seq, &(c, f))| Entry {
            freq: f,
            seq,
            tree: HTree::Leaf(c, f),
        })
        .collect();

    let mut counter = freqs.len();

    loop {
        let a = heap.pop()?.tree;
        let b = match heap.pop() {
            Some(entry) => entry.tree,
            None => return Some(a),
        };
        let merged_freq = a.freq() + b.freq();
        heap.push(Entry {
            freq: merged_freq,
            seq: counter,
            tree: HTree::Node(Box::new(a), Box::new(b), merged_freq),
        });
        counter += 1;
    }
}

/// Functional/recursive Rust: mirrors the OCaml approach — sort the list each round.
/// Structurally identical to the OCaml source; less efficient than the heap approach.
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut trees: Vec<HTree> = freqs.iter().map(|&(c, f)| HTree::Leaf(c, f)).collect();
    trees.sort_by_key(|t| t.freq());
    go(trees)
}

fn go(mut trees: Vec<HTree>) -> Option<HTree> {
    match trees.len() {
        0 => None,
        1 => trees.into_iter().next(),
        _ => {
            let a = trees.remove(0);
            let b = trees.remove(0);
            let merged_freq = a.freq() + b.freq();
            let merged = HTree::Node(Box::new(a), Box::new(b), merged_freq);
            trees.push(merged);
            trees.sort_by_key(|t| t.freq());
            go(trees)
        }
    }
}

/// Recursively collect (char, code-string) pairs by traversing the tree.
/// '0' descends left (first/lower-frequency child), '1' descends right.
pub fn codes(tree: &HTree, prefix: &str) -> Vec<(char, String)> {
    match tree {
        HTree::Leaf(c, _) => vec![(*c, prefix.to_owned())],
        HTree::Node(left, right, _) => {
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

    #[test]
    fn test_build_tree_idiomatic_single_char() {
        let freqs = vec![('x', 7)];
        let tree = build_tree_idiomatic(&freqs).expect("should build");
        assert_eq!(tree.freq(), 7);
        assert!(matches!(tree, HTree::Leaf('x', 7)));
    }

    #[test]
    fn test_build_tree_recursive_single_char() {
        let freqs = vec![('x', 7)];
        let tree = build_tree_recursive(&freqs).expect("should build");
        assert_eq!(tree.freq(), 7);
        assert!(matches!(tree, HTree::Leaf('x', 7)));
    }

    #[test]
    fn test_build_tree_empty_returns_none() {
        assert!(build_tree_idiomatic(&[]).is_none());
        assert!(build_tree_recursive(&[]).is_none());
    }

    #[test]
    fn test_root_frequency_is_sum_of_all() {
        let freqs = sample_freqs();
        let total: u32 = freqs.iter().map(|(_, f)| f).sum();
        let tree = build_tree_idiomatic(&freqs).expect("should build");
        assert_eq!(tree.freq(), total);
        let tree2 = build_tree_recursive(&freqs).expect("should build");
        assert_eq!(tree2.freq(), total);
    }

    #[test]
    fn test_codes_covers_all_chars() {
        let freqs = sample_freqs();
        let tree = build_tree_idiomatic(&freqs).expect("should build");
        let mut result = codes(&tree, "");
        result.sort_by_key(|(c, _)| *c);
        let chars: Vec<char> = result.iter().map(|(c, _)| *c).collect();
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_codes_prefix_free() {
        let freqs = sample_freqs();
        let tree = build_tree_idiomatic(&freqs).expect("should build");
        let result = codes(&tree, "");
        for i in 0..result.len() {
            for j in 0..result.len() {
                if i != j {
                    assert!(
                        !result[j].1.starts_with(&result[i].1),
                        "code {} is a prefix of {}",
                        result[i].1,
                        result[j].1
                    );
                }
            }
        }
    }

    #[test]
    fn test_highest_freq_char_gets_shortest_code() {
        // 'f' has freq 45 — more than all others combined — so it gets a 1-bit code.
        let freqs = sample_freqs();
        let tree = build_tree_idiomatic(&freqs).expect("should build");
        let result = codes(&tree, "");
        let f_len = result
            .iter()
            .find(|(c, _)| *c == 'f')
            .map(|(_, code)| code.len())
            .expect("f must have a code");
        assert_eq!(f_len, 1, "'f' should get a 1-bit code");
    }

    #[test]
    fn test_two_chars_each_get_one_bit_code() {
        let freqs = vec![('a', 3), ('b', 5)];
        let tree = build_tree_idiomatic(&freqs).expect("should build");
        let result = codes(&tree, "");
        assert_eq!(result.len(), 2);
        for (_, code) in &result {
            assert_eq!(code.len(), 1);
        }
    }

    #[test]
    fn test_codes_recursive_vs_idiomatic_same_char_set() {
        let freqs = sample_freqs();
        let t1 = build_tree_idiomatic(&freqs).expect("should build");
        let t2 = build_tree_recursive(&freqs).expect("should build");
        let mut c1 = codes(&t1, "");
        let mut c2 = codes(&t2, "");
        c1.sort_by_key(|(c, _)| *c);
        c2.sort_by_key(|(c, _)| *c);
        let chars1: Vec<char> = c1.iter().map(|(c, _)| *c).collect();
        let chars2: Vec<char> = c2.iter().map(|(c, _)| *c).collect();
        assert_eq!(chars1, chars2);
        for (_, code) in c1.iter().chain(c2.iter()) {
            assert!(!code.is_empty());
        }
    }
}
