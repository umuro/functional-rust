#![allow(clippy::all)]
//! Huffman encoding via greedy tree building.
//!
//! OCaml uses an algebraic type `htree` and repeatedly sorts a list to pick
//! the two lowest-frequency trees.  Rust mirrors the same structure with an
//! enum, but uses a `BinaryHeap` (min-heap via `Reverse`) for O(n log n)
//! merges instead of repeated O(n log n) sorts.

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// ── Tree definition ───────────────────────────────────────────────────────────

/// A Huffman tree: either a leaf holding a symbol + frequency,
/// or an internal node holding two sub-trees + their combined frequency.
#[derive(Debug, Clone)]
pub enum HTree {
    Leaf(char, u32),
    Node(Box<HTree>, Box<HTree>, u32),
}

impl HTree {
    /// Frequency stored at the root of this tree.
    pub fn freq(&self) -> u32 {
        match self {
            HTree::Leaf(_, f) | HTree::Node(_, _, f) => *f,
        }
    }
}

// ── Priority-queue wrapper ────────────────────────────────────────────────────

/// Newtype so `HTree` can live inside a `BinaryHeap` ordered by frequency.
/// `BinaryHeap` is a max-heap; wrapping with `Reverse` gives us a min-heap.
struct MinTree(HTree);

impl PartialEq for MinTree {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq() == other.0.freq()
    }
}
impl Eq for MinTree {}
impl PartialOrd for MinTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MinTree {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse so the *lowest* frequency is popped first.
        Reverse(self.0.freq()).cmp(&Reverse(other.0.freq()))
    }
}

// ── Solution 1: Idiomatic Rust — BinaryHeap min-heap ─────────────────────────

/// Build a Huffman tree from `(symbol, frequency)` pairs.
///
/// Uses a min-heap so each merge step is O(log n).
/// Returns `None` if `freqs` is empty.
pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut heap: BinaryHeap<MinTree> = freqs
        .iter()
        .map(|&(c, f)| MinTree(HTree::Leaf(c, f)))
        .collect();

    while heap.len() > 1 {
        // Safe: len > 1 guarantees two pops succeed.
        let MinTree(a) = heap.pop().unwrap();
        let MinTree(b) = heap.pop().unwrap();
        let combined = a.freq() + b.freq();
        heap.push(MinTree(HTree::Node(Box::new(a), Box::new(b), combined)));
    }

    heap.pop().map(|MinTree(t)| t)
}

// ── Solution 2: Functional/recursive — mirrors OCaml `go` ────────────────────

/// Build a Huffman tree using the recursive OCaml strategy:
/// sort the list, merge the two smallest, recurse.
///
/// Structurally identical to the OCaml `go` inner function.
/// O(n² log n) — not efficient, but shows the direct translation.
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut trees: Vec<HTree> = freqs.iter().map(|&(c, f)| HTree::Leaf(c, f)).collect();
    trees.sort_by_key(HTree::freq);
    go(trees)
}

fn go(mut trees: Vec<HTree>) -> Option<HTree> {
    match trees.len() {
        0 => None,
        1 => trees.into_iter().next(),
        _ => {
            // Remove the two lowest-frequency trees (front of sorted vec)
            let a = trees.remove(0);
            let b = trees.remove(0);
            let combined = a.freq() + b.freq();
            let merged = HTree::Node(Box::new(a), Box::new(b), combined);
            trees.push(merged);
            trees.sort_by_key(HTree::freq);
            go(trees)
        }
    }
}

// ── Code generation ───────────────────────────────────────────────────────────

/// Traverse the tree and collect `(symbol, binary_code_string)` pairs.
///
/// Left branch → append `'0'`, right branch → append `'1'`.
/// Mirrors OCaml's `codes prefix tree`.
pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    let mut result = Vec::new();
    collect_codes(tree, String::new(), &mut result);
    result
}

fn collect_codes(tree: &HTree, prefix: String, acc: &mut Vec<(char, String)>) {
    match tree {
        HTree::Leaf(c, _) => acc.push((*c, prefix)),
        HTree::Node(left, right, _) => {
            collect_codes(left, format!("{prefix}0"), acc);
            collect_codes(right, format!("{prefix}1"), acc);
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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

    // ── build_tree (BinaryHeap) ───────────────────────────────────────────────

    #[test]
    fn test_empty_returns_none() {
        assert!(build_tree(&[]).is_none());
    }

    #[test]
    fn test_single_symbol() {
        let tree = build_tree(&[('x', 7)]).unwrap();
        assert_eq!(tree.freq(), 7);
        // Single leaf: code is the empty string
        assert_eq!(codes(&tree), vec![('x', String::new())]);
    }

    #[test]
    fn test_two_symbols_root_freq() {
        let tree = build_tree(&[('a', 3), ('b', 5)]).unwrap();
        assert_eq!(tree.freq(), 8);
        let mut c = codes(&tree);
        c.sort_by_key(|(ch, _)| *ch);
        // Both must have exactly one-bit codes and be distinct
        assert!(c.iter().all(|(_, code)| code.len() == 1));
        let code_set: HashSet<&str> = c.iter().map(|(_, s)| s.as_str()).collect();
        assert!(code_set.contains("0") && code_set.contains("1"));
    }

    #[test]
    fn test_sample_root_frequency() {
        // 5+9+12+13+16+45 = 100
        let tree = build_tree(&sample_freqs()).unwrap();
        assert_eq!(tree.freq(), 100);
    }

    #[test]
    fn test_sample_all_symbols_present() {
        let tree = build_tree(&sample_freqs()).unwrap();
        let mut chars: Vec<char> = codes(&tree).into_iter().map(|(c, _)| c).collect();
        chars.sort();
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_codes_are_prefix_free() {
        let tree = build_tree(&sample_freqs()).unwrap();
        let c = codes(&tree);
        for (i, (_, ci)) in c.iter().enumerate() {
            for (j, (_, cj)) in c.iter().enumerate() {
                if i != j {
                    assert!(!cj.starts_with(ci.as_str()), "{ci} is a prefix of {cj}");
                }
            }
        }
    }

    #[test]
    fn test_highest_freq_gets_shortest_code() {
        // 'f' (freq 45) must get the shortest code
        let tree = build_tree(&sample_freqs()).unwrap();
        let c = codes(&tree);
        let f_len = c.iter().find(|(ch, _)| *ch == 'f').unwrap().1.len();
        let max_other = c
            .iter()
            .filter(|(ch, _)| *ch != 'f')
            .map(|(_, s)| s.len())
            .max()
            .unwrap();
        assert!(
            f_len < max_other,
            "f code len={f_len}, max other len={max_other}"
        );
    }

    // ── build_tree_recursive (OCaml-style sort-based) ─────────────────────────

    #[test]
    fn test_recursive_root_freq_matches_heap() {
        let freqs = sample_freqs();
        let t1 = build_tree(&freqs).unwrap();
        let t2 = build_tree_recursive(&freqs).unwrap();
        assert_eq!(t1.freq(), t2.freq());
    }

    #[test]
    fn test_recursive_empty_returns_none() {
        assert!(build_tree_recursive(&[]).is_none());
    }

    #[test]
    fn test_recursive_prefix_free() {
        let tree = build_tree_recursive(&sample_freqs()).unwrap();
        let c = codes(&tree);
        for (i, (_, ci)) in c.iter().enumerate() {
            for (j, (_, cj)) in c.iter().enumerate() {
                if i != j {
                    assert!(!cj.starts_with(ci.as_str()), "{ci} is a prefix of {cj}");
                }
            }
        }
    }
}
