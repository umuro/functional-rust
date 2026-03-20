use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// A Huffman tree: a leaf holds a character and its frequency;
/// an internal node holds two subtrees and their combined frequency.
#[derive(Debug, PartialEq, Eq)]
pub enum HTree {
    Leaf(char, u32),
    Node(Box<HTree>, Box<HTree>, u32),
}

impl HTree {
    /// Frequency stored at this node (leaf or internal).
    pub fn freq(&self) -> u32 {
        match self {
            HTree::Leaf(_, f) | HTree::Node(_, _, f) => *f,
        }
    }
}

// Newtype for BinaryHeap: orders HTree by frequency only (no structural comparison).
struct ByFreq(HTree);

impl PartialEq for ByFreq {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq() == other.0.freq()
    }
}
impl Eq for ByFreq {}
impl PartialOrd for ByFreq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ByFreq {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.freq().cmp(&other.0.freq())
    }
}

/// Builds a Huffman tree using a min-heap (idiomatic Rust).
///
/// Each step pops the two lowest-frequency trees and merges them — O(n log n)
/// total. Uses `Reverse<ByFreq>` to turn `BinaryHeap` (max-heap) into a min-heap.
pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut heap: BinaryHeap<Reverse<ByFreq>> = freqs
        .iter()
        .map(|&(c, f)| Reverse(ByFreq(HTree::Leaf(c, f))))
        .collect();

    while heap.len() > 1 {
        let Reverse(ByFreq(a)) = heap.pop().unwrap();
        let Reverse(ByFreq(b)) = heap.pop().unwrap();
        let f = a.freq() + b.freq();
        heap.push(Reverse(ByFreq(HTree::Node(Box::new(a), Box::new(b), f))));
    }

    heap.pop().map(|Reverse(ByFreq(t))| t)
}

/// Builds a Huffman tree by sorting a Vec each iteration — mirrors the OCaml approach.
///
/// OCaml uses `List.sort` after every merge; Rust uses `sort_by_key`. Both are
/// O(n²log n) but identical in spirit. The recursive inner function `go` maps
/// directly to OCaml's `let rec go = function ...`.
pub fn build_tree_sorted(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut trees: Vec<HTree> = freqs.iter().map(|&(c, f)| HTree::Leaf(c, f)).collect();
    trees.sort_by_key(HTree::freq);

    // Recursive helper — mirrors OCaml's `let rec go = function`
    fn go(mut trees: Vec<HTree>) -> HTree {
        if trees.len() == 1 {
            return trees.remove(0);
        }
        // Take the two lowest-frequency trees (front of sorted Vec)
        let a = trees.remove(0);
        let b = trees.remove(0);
        let f = a.freq() + b.freq();
        let merged = HTree::Node(Box::new(a), Box::new(b), f);
        trees.push(merged);
        trees.sort_by_key(HTree::freq);
        go(trees)
    }

    Some(go(trees))
}

/// Traverses a Huffman tree and returns `(char, binary-code)` pairs.
///
/// Left branches append `"0"`, right branches append `"1"` — identical to the
/// OCaml `codes` function using string concatenation on the prefix.
pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    fn go(tree: &HTree, prefix: String) -> Vec<(char, String)> {
        match tree {
            HTree::Leaf(c, _) => vec![(*c, prefix)],
            HTree::Node(l, r, _) => {
                let mut result = go(l, format!("{prefix}0"));
                result.extend(go(r, format!("{prefix}1")));
                result
            }
        }
    }
    go(tree, String::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_single_char_builds_leaf_with_empty_code() {
        let tree = build_tree(&[('z', 42)]).unwrap();
        assert_eq!(tree, HTree::Leaf('z', 42));
        // A single-symbol alphabet: the only code is the empty string
        let cs = codes(&tree);
        assert_eq!(cs, vec![('z', String::new())]);
    }

    #[test]
    fn test_two_chars_get_single_bit_codes() {
        let tree = build_tree(&[('a', 3), ('b', 7)]).unwrap();
        let cs: HashMap<char, String> = codes(&tree).into_iter().collect();
        assert_eq!(cs.len(), 2);
        assert_eq!(cs[&'a'].len(), 1);
        assert_eq!(cs[&'b'].len(), 1);
        // The two codes must be distinct (one "0", one "1")
        assert_ne!(cs[&'a'], cs[&'b']);
    }

    #[test]
    fn test_classic_six_char_example_code_lengths() {
        // Standard CLRS Huffman example: optimal lengths are f=1, c/d/e=3, a/b=4
        let freqs = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        let tree = build_tree(&freqs).unwrap();
        let cs: HashMap<char, String> = codes(&tree).into_iter().collect();

        assert_eq!(cs.len(), 6);
        assert_eq!(cs[&'f'].len(), 1, "'f' must get 1-bit code (highest freq)");
        assert_eq!(cs[&'c'].len(), 3);
        assert_eq!(cs[&'d'].len(), 3);
        assert_eq!(cs[&'e'].len(), 3);
        assert_eq!(cs[&'a'].len(), 4, "'a' must get 4-bit code (lowest freq)");
        assert_eq!(cs[&'b'].len(), 4);
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
        let cs: Vec<String> = codes(&tree).into_iter().map(|(_, c)| c).collect();

        for (i, c1) in cs.iter().enumerate() {
            for (j, c2) in cs.iter().enumerate() {
                if i != j {
                    assert!(
                        !c2.starts_with(c1.as_str()),
                        "prefix-free violation: {c1} is a prefix of {c2}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_sorted_and_heap_produce_same_code_lengths() {
        // Both algorithms must produce optimally-coded lengths for all characters
        let freqs = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        let heap_lens: HashMap<char, usize> = codes(&build_tree(&freqs).unwrap())
            .into_iter()
            .map(|(c, s)| (c, s.len()))
            .collect();
        let sorted_lens: HashMap<char, usize> = codes(&build_tree_sorted(&freqs).unwrap())
            .into_iter()
            .map(|(c, s)| (c, s.len()))
            .collect();
        assert_eq!(heap_lens, sorted_lens);
    }

    #[test]
    fn test_empty_input_returns_none() {
        assert_eq!(build_tree(&[]), None);
        assert_eq!(build_tree_sorted(&[]), None);
    }
}
