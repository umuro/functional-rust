#![allow(dead_code)]
//! Huffman coding tree construction and prefix-code generation.
//!
//! Two build strategies are provided:
//! * [`build_tree_heap`] — idiomatic Rust using a min-heap ([`BinaryHeap`]
//!   with [`Reverse`]).  O(n log n), no repeated scanning.
//! * [`build_tree_sorted`] — mirrors the OCaml original's sort/merge loop
//!   for easy side-by-side comparison.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A Huffman tree: either a leaf carrying a character + frequency, or an
/// internal node with cached total frequency and two children.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HTree {
    Leaf(char, u32),
    Node(Box<HTree>, Box<HTree>, u32),
}

impl HTree {
    /// Total frequency at this subtree.  O(1) because internal nodes cache it.
    pub fn freq(&self) -> u32 {
        match self {
            HTree::Leaf(_, f) | HTree::Node(_, _, f) => *f,
        }
    }
}

// Wrapper that makes a [`BinaryHeap`] behave as a min-heap keyed on
// `(freq, tiebreak)`.  `HTree` itself intentionally does not implement `Ord`
// because there is no meaningful total ordering on trees.
struct HeapEntry {
    freq: u32,
    tiebreak: usize,
    tree: HTree,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        (self.freq, self.tiebreak) == (other.freq, other.tiebreak)
    }
}
impl Eq for HeapEntry {}
impl Ord for HeapEntry {
    // Flipped comparison — BinaryHeap is a max-heap, so reversing here gives
    // a min-heap by frequency, with `tiebreak` breaking ties deterministically.
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .freq
            .cmp(&self.freq)
            .then_with(|| other.tiebreak.cmp(&self.tiebreak))
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// --- Approach 1: idiomatic Rust using a min-heap -----------------------------

/// Build a Huffman tree with a [`BinaryHeap`] acting as a min-priority queue.
/// O(n log n).  Returns `None` for empty input.
pub fn build_tree_heap(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut heap: BinaryHeap<HeapEntry> = freqs
        .iter()
        .enumerate()
        .map(|(i, &(c, f))| HeapEntry {
            freq: f,
            tiebreak: i,
            tree: HTree::Leaf(c, f),
        })
        .collect();
    let mut tiebreak = freqs.len();
    while heap.len() > 1 {
        let a = heap.pop()?;
        let b = heap.pop()?;
        let total = a.freq + b.freq;
        let merged = HTree::Node(Box::new(a.tree), Box::new(b.tree), total);
        heap.push(HeapEntry {
            freq: total,
            tiebreak,
            tree: merged,
        });
        tiebreak += 1;
    }
    heap.pop().map(|e| e.tree)
}

// --- Approach 2: sort/merge loop (OCaml-parallel) ----------------------------

/// Build a Huffman tree by re-sorting the working list on every merge.
/// A direct translation of the OCaml reference implementation; clearer for
/// teaching, O(n² log n) instead of O(n log n).
pub fn build_tree_sorted(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut trees: Vec<HTree> = freqs.iter().map(|&(c, f)| HTree::Leaf(c, f)).collect();
    trees.sort_by_key(HTree::freq);
    while trees.len() > 1 {
        let a = trees.remove(0);
        let b = trees.remove(0);
        let total = a.freq() + b.freq();
        let merged = HTree::Node(Box::new(a), Box::new(b), total);
        let pos = trees
            .iter()
            .position(|t| t.freq() > total)
            .unwrap_or(trees.len());
        trees.insert(pos, merged);
    }
    trees.into_iter().next()
}

// --- Code extraction ---------------------------------------------------------

/// Walk the tree and emit `(char, bitstring)` pairs, one per leaf.  Going left
/// appends `0`, going right appends `1`.  A single-leaf tree yields an empty
/// code — matching the OCaml behaviour.
pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    fn go(tree: &HTree, prefix: &mut String, out: &mut Vec<(char, String)>) {
        match tree {
            HTree::Leaf(c, _) => out.push((*c, prefix.clone())),
            HTree::Node(l, r, _) => {
                prefix.push('0');
                go(l, prefix, out);
                prefix.pop();
                prefix.push('1');
                go(r, prefix, out);
                prefix.pop();
            }
        }
    }
    let mut prefix = String::new();
    let mut out = Vec::new();
    go(tree, &mut prefix, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<(char, u32)> {
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
    fn empty_input_returns_none() {
        assert!(build_tree_heap(&[]).is_none());
        assert!(build_tree_sorted(&[]).is_none());
    }

    #[test]
    fn single_leaf_is_returned_as_is() {
        let t = build_tree_heap(&[('z', 7)]).expect("one leaf");
        assert_eq!(t, HTree::Leaf('z', 7));
        let pairs = codes(&t);
        assert_eq!(pairs, vec![('z', String::new())]);
    }

    #[test]
    fn root_frequency_is_total_of_inputs() {
        let total: u32 = sample().iter().map(|&(_, f)| f).sum();
        let t = build_tree_heap(&sample()).expect("non-empty");
        assert_eq!(t.freq(), total);
    }

    #[test]
    fn codes_are_prefix_free_and_complete() {
        let t = build_tree_heap(&sample()).expect("non-empty");
        let pairs = codes(&t);
        assert_eq!(pairs.len(), 6);
        // prefix-free: no code is a prefix of another
        for (i, (_, a)) in pairs.iter().enumerate() {
            for (j, (_, b)) in pairs.iter().enumerate() {
                if i != j {
                    assert!(!b.starts_with(a), "{a} is a prefix of {b}");
                }
            }
        }
    }

    #[test]
    fn most_frequent_symbol_gets_shortest_code() {
        let t = build_tree_heap(&sample()).expect("non-empty");
        let pairs = codes(&t);
        let shortest = pairs.iter().min_by_key(|(_, s)| s.len()).unwrap();
        assert_eq!(shortest.0, 'f');
        assert_eq!(shortest.1.len(), 1);
    }

    #[test]
    fn heap_and_sort_builds_produce_same_code_lengths() {
        // Exact tree shape can differ on ties, but the multiset of code
        // lengths is a Huffman invariant.
        let a = codes(&build_tree_heap(&sample()).unwrap());
        let b = codes(&build_tree_sorted(&sample()).unwrap());
        let mut la: Vec<usize> = a.iter().map(|(_, s)| s.len()).collect();
        let mut lb: Vec<usize> = b.iter().map(|(_, s)| s.len()).collect();
        la.sort_unstable();
        lb.sort_unstable();
        assert_eq!(la, lb);
    }
}
