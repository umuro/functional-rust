#![allow(dead_code)]
//! 101: Huffman Encoding — Greedy Tree Building
//! Builds an optimal prefix-free code tree by repeatedly merging the two
//! lowest-frequency nodes, mirroring the OCaml pattern-match / list-sort approach.

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

// ── Tree type ────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum HTree {
    Leaf(char, u32),
    Node(Box<HTree>, Box<HTree>, u32),
}

impl HTree {
    pub fn freq(&self) -> u32 {
        match self {
            HTree::Leaf(_, f) | HTree::Node(_, _, f) => *f,
        }
    }
}

// Wrapper so BinaryHeap<Reverse<FreqOrd>> acts as a min-heap by frequency.
// All four Ord traits must be consistent; we use frequency only.
struct FreqOrd(HTree);

impl PartialEq for FreqOrd {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq() == other.0.freq()
    }
}
impl Eq for FreqOrd {}

impl Ord for FreqOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.freq().cmp(&other.0.freq())
    }
}
impl PartialOrd for FreqOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ── Solution 1: Idiomatic Rust — BinaryHeap min-heap ────────────────────────
//
// Uses std's BinaryHeap (max-heap) wrapped with Reverse to get a min-heap.
// Each iteration pops the two cheapest nodes and pushes their merged parent.
// O(n log n) — same asymptotic cost as the OCaml sort-per-step version but
// with O(log n) insert/remove instead of O(n) re-sort.

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut heap: BinaryHeap<Reverse<FreqOrd>> = freqs
        .iter()
        .map(|&(c, f)| Reverse(FreqOrd(HTree::Leaf(c, f))))
        .collect();

    while heap.len() > 1 {
        let Reverse(FreqOrd(a)) = heap.pop()?;
        let Reverse(FreqOrd(b)) = heap.pop()?;
        let freq = a.freq() + b.freq();
        heap.push(Reverse(FreqOrd(HTree::Node(
            Box::new(a),
            Box::new(b),
            freq,
        ))));
    }

    heap.pop().map(|Reverse(FreqOrd(t))| t)
}

// ── Solution 2: Functional/recursive — mirrors OCaml list-sort style ─────────
//
// Keeps a sorted Vec and re-sorts after each merge, exactly as the OCaml
// `go` function does.  More transparent but O(n² log n).

pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut trees: Vec<HTree> = freqs.iter().map(|&(c, f)| HTree::Leaf(c, f)).collect();
    trees.sort_by_key(|t| t.freq());

    fn go(mut trees: Vec<HTree>) -> Option<HTree> {
        match trees.len() {
            0 => None,
            1 => trees.into_iter().next(),
            _ => {
                // Remove two cheapest (front of sorted list)
                let a = trees.remove(0);
                let b = trees.remove(0);
                let freq = a.freq() + b.freq();
                let merged = HTree::Node(Box::new(a), Box::new(b), freq);
                // Re-insert and re-sort, then recurse
                let mut next: Vec<HTree> = std::iter::once(merged).chain(trees).collect();
                next.sort_by_key(|t| t.freq());
                go(next)
            }
        }
    }

    go(trees)
}

// ── Code generation ───────────────────────────────────────────────────────────
//
// Traverse the tree, prepending "0" for left branches and "1" for right.
// Returns (char, code-string) pairs; leaves carry the final code.

pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    fn go(node: &HTree, prefix: String, out: &mut Vec<(char, String)>) {
        match node {
            HTree::Leaf(c, _) => out.push((*c, prefix)),
            HTree::Node(left, right, _) => {
                go(left, format!("{prefix}0"), out);
                go(right, format!("{prefix}1"), out);
            }
        }
    }
    let mut out = Vec::new();
    go(tree, String::new(), &mut out);
    out
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

    #[test]
    fn test_single_char_produces_one_empty_code() {
        let tree = build_tree(&[('x', 7)]).unwrap();
        let c = codes(&tree);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].0, 'x');
        // A single leaf has no branching; the convention gives it an empty prefix.
        assert_eq!(c[0].1, "");
    }

    #[test]
    fn test_two_chars_get_single_bit_codes() {
        let tree = build_tree(&[('a', 3), ('b', 7)]).unwrap();
        let c = codes(&tree);
        assert_eq!(c.len(), 2);
        // Both codes must be exactly 1 bit
        assert!(c.iter().all(|(_, code)| code.len() == 1));
        // Codes must differ
        assert_ne!(c[0].1, c[1].1);
    }

    #[test]
    fn test_highest_frequency_gets_shortest_code() {
        let tree = build_tree(&sample_freqs()).unwrap();
        let c = codes(&tree);
        let f_len = c.iter().find(|(ch, _)| *ch == 'f').unwrap().1.len();
        let a_len = c.iter().find(|(ch, _)| *ch == 'a').unwrap().1.len();
        // f (freq 45) must have a strictly shorter code than a (freq 5)
        assert!(
            f_len < a_len,
            "f code len {f_len} should be < a code len {a_len}"
        );
    }

    #[test]
    fn test_codes_are_unique_and_correct_count() {
        let tree = build_tree(&sample_freqs()).unwrap();
        let c = codes(&tree);
        assert_eq!(c.len(), 6);
        let unique: HashSet<&str> = c.iter().map(|(_, s)| s.as_str()).collect();
        assert_eq!(unique.len(), 6, "all codes must be distinct");
    }

    #[test]
    fn test_root_frequency_equals_total() {
        let freqs = sample_freqs();
        let tree = build_tree(&freqs).unwrap();
        let total: u32 = freqs.iter().map(|(_, f)| f).sum();
        assert_eq!(tree.freq(), total);
    }

    #[test]
    fn test_recursive_and_heap_produce_same_total_frequency() {
        let freqs = sample_freqs();
        let t1 = build_tree(&freqs).unwrap();
        let t2 = build_tree_recursive(&freqs).unwrap();
        assert_eq!(t1.freq(), t2.freq());
        assert_eq!(codes(&t1).len(), codes(&t2).len());
    }

    #[test]
    fn test_empty_input_returns_none() {
        assert!(build_tree(&[]).is_none());
        assert!(build_tree_recursive(&[]).is_none());
    }

    #[test]
    fn test_all_codes_are_binary_strings() {
        let tree = build_tree(&sample_freqs()).unwrap();
        for (_, code) in codes(&tree) {
            assert!(
                code.chars().all(|c| c == '0' || c == '1'),
                "code {code:?} contains non-binary characters"
            );
        }
    }
}
