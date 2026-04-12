#![allow(dead_code)]
//! Huffman Encoding: Greedy Tree Building
//! See example.ml for OCaml reference
//!
//! Builds a Huffman tree from character frequencies using a min-heap priority queue.
//! Characters with lower frequency get longer bit codes; higher frequency gets shorter codes.

use std::collections::HashMap;

/// A Huffman tree node — either a Leaf (character + frequency) or an internal Node.
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

/// Idiomatic Rust: build a Huffman tree from a (char, frequency) slice.
/// Uses a BinaryHeap with a struct wrapper (HTree is not Ord).
pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    use std::collections::BinaryHeap;

    // Wrapper so BinaryHeap can order by (freq, counter) without requiring HTree: Ord.
    struct Item {
        freq: u32,
        counter: usize,
        tree: HTree,
    }
    impl PartialEq for Item {
        fn eq(&self, other: &Self) -> bool {
            self.freq == other.freq && self.counter == other.counter
        }
    }
    impl Eq for Item {}
    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Item {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            // Reverse on freq: lower frequency → higher priority (min-heap).
            other
                .freq
                .cmp(&self.freq)
                .then(other.counter.cmp(&self.counter))
        }
    }

    let mut counter = freqs.len();
    let mut heap: BinaryHeap<Item> = freqs
        .iter()
        .enumerate()
        .map(|(i, &(c, f))| Item {
            freq: f,
            counter: i,
            tree: HTree::Leaf(c, f),
        })
        .collect();

    loop {
        let a = heap.pop()?;
        let b = match heap.pop() {
            Some(item) => item,
            None => return Some(a.tree),
        };
        let merged_freq = a.freq + b.freq;
        let merged = HTree::Node(Box::new(a.tree), Box::new(b.tree), merged_freq);
        heap.push(Item {
            freq: merged_freq,
            counter,
            tree: merged,
        });
        counter += 1;
    }
}

/// Functional/recursive: mirrors the OCaml sort-each-round approach.
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

/// Walk the Huffman tree to collect (char, code) pairs.
/// Left = "0", right = "1".
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

/// Build a Huffman tree from a text string (counts character frequencies automatically).
pub fn build_tree_from_text(text: &str) -> Option<HTree> {
    let mut freq: HashMap<char, u32> = HashMap::new();
    for c in text.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    let freqs: Vec<(char, u32)> = freq.into_iter().collect();
    build_tree(&freqs)
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
    fn test_build_tree_empty_returns_none() {
        assert!(build_tree(&[]).is_none());
        assert!(build_tree_recursive(&[]).is_none());
    }

    #[test]
    fn test_build_tree_single_char() {
        let freqs = vec![('x', 7)];
        let tree = build_tree(&freqs).expect("should build");
        assert_eq!(tree.freq(), 7);
        assert!(matches!(tree, HTree::Leaf('x', 7)));
    }

    #[test]
    fn test_root_frequency_is_sum_of_all() {
        let freqs = sample_freqs();
        let total: u32 = freqs.iter().map(|(_, f)| f).sum();
        let tree = build_tree(&freqs).expect("should build");
        assert_eq!(tree.freq(), total);
    }

    #[test]
    fn test_codes_covers_all_chars() {
        let freqs = sample_freqs();
        let tree = build_tree(&freqs).expect("should build");
        let mut result = codes(&tree, "");
        result.sort_by_key(|(c, _)| *c);
        let chars: Vec<char> = result.iter().map(|(c, _)| *c).collect();
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_codes_prefix_free() {
        let freqs = sample_freqs();
        let tree = build_tree(&freqs).expect("should build");
        let result = codes(&tree, "");
        // No code is a prefix of another in a valid Huffman tree.
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
        let tree = build_tree(&freqs).expect("should build");
        let result = codes(&tree, "");
        let f_len = result
            .iter()
            .find(|(c, _)| *c == 'f')
            .map(|(_, code)| code.len())
            .expect("f must have a code");
        assert_eq!(f_len, 1, "'f' should get a 1-bit code");
    }

    #[test]
    fn test_build_tree_from_text() {
        let tree = build_tree_from_text("aabbbcccc");
        assert!(tree.is_some());
        let tree = tree.unwrap();
        // Total frequency should equal string length.
        assert_eq!(tree.freq(), 9);
    }

    #[test]
    fn test_two_chars_get_one_bit_codes() {
        let freqs = vec![('a', 3), ('b', 5)];
        let tree = build_tree(&freqs).expect("should build");
        let result = codes(&tree, "");
        assert_eq!(result.len(), 2);
        for (_, code) in &result {
            assert_eq!(code.len(), 1);
        }
    }
}
