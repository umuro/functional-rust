#![allow(dead_code)]

// Huffman Encoding — Greedy Tree Building
//
// Mirrors the OCaml `htree` algebraic type and the greedy algorithm:
// always merge the two lowest-frequency trees until one remains.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A Huffman tree node — a leaf holds a symbol and its frequency;
/// an internal node holds two child subtrees and the combined frequency.
#[derive(Debug, Clone)]
pub enum HTree {
    Leaf(char, u32),
    Node(Box<HTree>, Box<HTree>, u32),
}

impl HTree {
    /// Returns the frequency stored at this node.
    pub fn freq(&self) -> u32 {
        match self {
            HTree::Leaf(_, f) | HTree::Node(_, _, f) => *f,
        }
    }
}

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — BinaryHeap used as a min-heap
// ---------------------------------------------------------------------------

/// Newtype wrapper so `BinaryHeap<MinTree>` yields the *smallest* frequency first.
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
        // Reverse so the heap pops lowest-frequency first.
        other.0.freq().cmp(&self.0.freq())
    }
}

/// Build a Huffman tree using a priority queue for O(n log n) greedy merging.
/// Returns `None` if `freqs` is empty.
pub fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut heap: BinaryHeap<MinTree> = freqs
        .iter()
        .map(|&(c, f)| MinTree(HTree::Leaf(c, f)))
        .collect();

    while heap.len() > 1 {
        let MinTree(a) = heap.pop()?;
        let MinTree(b) = heap.pop()?;
        let combined = a.freq() + b.freq();
        heap.push(MinTree(HTree::Node(Box::new(a), Box::new(b), combined)));
    }

    heap.pop().map(|MinTree(t)| t)
}

// ---------------------------------------------------------------------------
// Solution 2: Functional/recursive — mirrors OCaml's List.sort + inner `go`
// ---------------------------------------------------------------------------

/// Build a Huffman tree by sorting and merging in a loop, closely following
/// the OCaml `go` inner function. Returns `None` if `freqs` is empty.
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut trees: Vec<HTree> = freqs.iter().map(|&(c, f)| HTree::Leaf(c, f)).collect();
    trees.sort_by_key(HTree::freq);
    Some(go(trees))
}

fn go(mut trees: Vec<HTree>) -> HTree {
    match trees.len() {
        0 => panic!("go: empty — unreachable via public API"),
        1 => trees.remove(0),
        _ => {
            // The two lowest-frequency trees are at the front of the sorted vec.
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

// ---------------------------------------------------------------------------
// Code extraction — traverse the tree, accumulating "0"/"1" prefix strings
// ---------------------------------------------------------------------------

/// Returns all `(symbol, binary-code-string)` pairs encoded in `tree`.
/// Mirrors OCaml's recursive `codes prefix tree` function.
pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    codes_acc(tree, String::new())
}

fn codes_acc(tree: &HTree, prefix: String) -> Vec<(char, String)> {
    match tree {
        HTree::Leaf(c, _) => vec![(*c, prefix)],
        HTree::Node(left, right, _) => {
            let mut left_codes = codes_acc(left, format!("{prefix}0"));
            left_codes.extend(codes_acc(right, format!("{prefix}1")));
            left_codes
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const FREQS: &[(char, u32)] = &[
        ('a', 5),
        ('b', 9),
        ('c', 12),
        ('d', 13),
        ('e', 16),
        ('f', 45),
    ];

    // --- HTree::freq ---

    #[test]
    fn test_freq_leaf() {
        assert_eq!(HTree::Leaf('x', 7).freq(), 7);
    }

    #[test]
    fn test_freq_node() {
        let node = HTree::Node(
            Box::new(HTree::Leaf('a', 3)),
            Box::new(HTree::Leaf('b', 4)),
            7,
        );
        assert_eq!(node.freq(), 7);
    }

    // --- build_tree_recursive ---

    #[test]
    fn test_recursive_empty() {
        assert!(build_tree_recursive(&[]).is_none());
    }

    #[test]
    fn test_recursive_single() {
        let tree = build_tree_recursive(&[('z', 1)]).unwrap();
        assert!(matches!(tree, HTree::Leaf('z', 1)));
    }

    #[test]
    fn test_recursive_two_symbols() {
        let tree = build_tree_recursive(&[('a', 3), ('b', 5)]).unwrap();
        assert_eq!(tree.freq(), 8);
        assert!(matches!(tree, HTree::Node(_, _, 8)));
    }

    #[test]
    fn test_recursive_root_freq_equals_total() {
        let total: u32 = FREQS.iter().map(|&(_, f)| f).sum();
        let tree = build_tree_recursive(FREQS).unwrap();
        assert_eq!(tree.freq(), total);
    }

    // --- build_tree_idiomatic ---

    #[test]
    fn test_idiomatic_empty() {
        assert!(build_tree_idiomatic(&[]).is_none());
    }

    #[test]
    fn test_idiomatic_single() {
        let tree = build_tree_idiomatic(&[('z', 1)]).unwrap();
        assert!(matches!(tree, HTree::Leaf('z', 1)));
    }

    #[test]
    fn test_idiomatic_root_freq_equals_total() {
        let total: u32 = FREQS.iter().map(|&(_, f)| f).sum();
        let tree = build_tree_idiomatic(FREQS).unwrap();
        assert_eq!(tree.freq(), total);
    }

    #[test]
    fn test_idiomatic_two_symbols() {
        let tree = build_tree_idiomatic(&[('a', 3), ('b', 5)]).unwrap();
        assert_eq!(tree.freq(), 8);
    }

    // --- codes ---

    #[test]
    fn test_codes_single_leaf() {
        let tree = HTree::Leaf('a', 1);
        let result = codes(&tree);
        assert_eq!(result, vec![('a', String::new())]);
    }

    #[test]
    fn test_codes_contains_all_symbols() {
        let tree = build_tree_recursive(FREQS).unwrap();
        let mut result = codes(&tree);
        result.sort_by_key(|&(c, _)| c);
        let symbols: Vec<char> = result.iter().map(|&(c, _)| c).collect();
        assert_eq!(symbols, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_codes_only_zero_one_bits() {
        let tree = build_tree_recursive(FREQS).unwrap();
        for (_, code) in codes(&tree) {
            assert!(code.chars().all(|b| b == '0' || b == '1'));
        }
    }

    #[test]
    fn test_codes_prefix_free() {
        // No code should be a prefix of another — a fundamental Huffman property.
        let tree = build_tree_recursive(FREQS).unwrap();
        let result = codes(&tree);
        for (i, (_, ci)) in result.iter().enumerate() {
            for (j, (_, cj)) in result.iter().enumerate() {
                if i != j {
                    assert!(
                        !cj.starts_with(ci.as_str()),
                        "code '{ci}' is a prefix of '{cj}'"
                    );
                }
            }
        }
    }

    #[test]
    fn test_codes_most_frequent_shortest() {
        // 'f' (freq 45) must receive the shortest code.
        let tree = build_tree_recursive(FREQS).unwrap();
        let result = codes(&tree);
        let f_len = result
            .iter()
            .find(|&&(c, _)| c == 'f')
            .map(|(_, code)| code.len())
            .unwrap();
        for (c, code) in &result {
            if *c != 'f' {
                assert!(
                    code.len() >= f_len,
                    "'{c}' code length {} < 'f' code length {f_len}",
                    code.len()
                );
            }
        }
    }

    #[test]
    fn test_codes_unique() {
        // Every symbol gets a distinct binary code.
        let tree = build_tree_recursive(FREQS).unwrap();
        let result = codes(&tree);
        let mut code_strings: Vec<&str> = result.iter().map(|(_, c)| c.as_str()).collect();
        let before = code_strings.len();
        code_strings.sort_unstable();
        code_strings.dedup();
        assert_eq!(before, code_strings.len());
    }
}
