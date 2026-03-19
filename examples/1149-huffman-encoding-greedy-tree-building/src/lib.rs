#![allow(clippy::all)]
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Huffman tree: either a leaf (char + frequency) or an internal node
#[derive(Debug)]
pub enum HTree {
    Leaf(char, u32),
    Node(Box<HTree>, Box<HTree>, u32),
}

impl HTree {
    pub fn freq(&self) -> u32 {
        // Or-pattern collapses both arms into one binding
        match self {
            HTree::Leaf(_, f) | HTree::Node(_, _, f) => *f,
        }
    }
}

/// Wrapper that reverses BinaryHeap's max-order so we get a min-heap
struct MinHeapNode(HTree);

impl PartialEq for MinHeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq() == other.0.freq()
    }
}

impl Eq for MinHeapNode {}

impl Ord for MinHeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed: lower frequency = higher priority
        other.0.freq().cmp(&self.0.freq())
    }
}

impl PartialOrd for MinHeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Solution 1: Idiomatic Rust — BinaryHeap for O(log n) per merge
pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut heap: BinaryHeap<MinHeapNode> = freqs
        .iter()
        .map(|&(c, f)| MinHeapNode(HTree::Leaf(c, f)))
        .collect();

    while heap.len() > 1 {
        if let (Some(a_node), Some(b_node)) = (heap.pop(), heap.pop()) {
            let a = a_node.0;
            let b = b_node.0;
            let merged_freq = a.freq() + b.freq();
            heap.push(MinHeapNode(HTree::Node(
                Box::new(a),
                Box::new(b),
                merged_freq,
            )));
        }
    }
    heap.pop().map(|n| n.0)
}

/// Solution 2: Functional/recursive — mirrors the OCaml sort-then-merge style
pub fn build_tree_functional(freqs: &[(char, u32)]) -> Option<HTree> {
    let trees: Vec<HTree> = freqs.iter().map(|&(c, f)| HTree::Leaf(c, f)).collect();
    go_sorted(trees)
}

fn go_sorted(mut trees: Vec<HTree>) -> Option<HTree> {
    trees.sort_by_key(|t| t.freq());
    match trees.len() {
        0 => None,
        1 => Some(trees.remove(0)),
        _ => {
            let a = trees.remove(0);
            let b = trees.remove(0);
            let merged_freq = a.freq() + b.freq();
            let merged = HTree::Node(Box::new(a), Box::new(b), merged_freq);
            trees.push(merged);
            go_sorted(trees)
        }
    }
}

/// Recursively walk the tree, accumulating a binary prefix string per leaf
pub fn codes(prefix: &str, tree: &HTree) -> Vec<(char, String)> {
    match tree {
        HTree::Leaf(c, _) => vec![(*c, prefix.to_string())],
        HTree::Node(l, r, _) => {
            let left = codes(&(prefix.to_string() + "0"), l);
            let right = codes(&(prefix.to_string() + "1"), r);
            left.into_iter().chain(right).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const CLASSIC: [(char, u32); 6] = [
        ('a', 5),
        ('b', 9),
        ('c', 12),
        ('d', 13),
        ('e', 16),
        ('f', 45),
    ];

    fn weighted_len(freqs: &[(char, u32)], cs: &HashMap<char, String>) -> u32 {
        freqs.iter().map(|(c, f)| f * cs[c].len() as u32).sum()
    }

    #[test]
    fn test_empty_returns_none() {
        assert!(build_tree(&[]).is_none());
        assert!(build_tree_functional(&[]).is_none());
    }

    #[test]
    fn test_single_leaf() {
        let tree = build_tree(&[('x', 7)]).unwrap();
        assert_eq!(tree.freq(), 7);
        // Single-leaf tree: code is the empty prefix
        assert_eq!(codes("", &tree), vec![('x', String::new())]);
    }

    #[test]
    fn test_all_chars_present_in_codes() {
        let tree = build_tree(&CLASSIC).unwrap();
        let mut cs = codes("", &tree);
        cs.sort_by_key(|&(c, _)| c);
        let chars: Vec<char> = cs.iter().map(|(c, _)| *c).collect();
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_optimal_weighted_length_heap() {
        // Classic example: minimum weighted path length = 224
        let tree = build_tree(&CLASSIC).unwrap();
        let cs: HashMap<char, String> = codes("", &tree).into_iter().collect();
        assert_eq!(weighted_len(&CLASSIC, &cs), 224);
    }

    #[test]
    fn test_optimal_weighted_length_functional() {
        let tree = build_tree_functional(&CLASSIC).unwrap();
        let cs: HashMap<char, String> = codes("", &tree).into_iter().collect();
        assert_eq!(weighted_len(&CLASSIC, &cs), 224);
    }

    #[test]
    fn test_higher_freq_gets_shorter_code() {
        let tree = build_tree(&CLASSIC).unwrap();
        let cs: HashMap<char, String> = codes("", &tree).into_iter().collect();
        // 'f' (45) is the most frequent — must have the shortest code
        let f_len = cs[&'f'].len();
        for (c, _) in &CLASSIC {
            assert!(f_len <= cs[c].len(), "'f' code longer than '{}'", c);
        }
    }

    #[test]
    fn test_codes_are_prefix_free() {
        // Huffman property: no code is a prefix of another code
        let tree = build_tree(&CLASSIC).unwrap();
        let cs: Vec<String> = codes("", &tree).into_iter().map(|(_, c)| c).collect();
        for (i, a) in cs.iter().enumerate() {
            for (j, b) in cs.iter().enumerate() {
                if i != j {
                    assert!(
                        !b.starts_with(a.as_str()),
                        "prefix-free violation: '{}' is a prefix of '{}'",
                        a,
                        b
                    );
                }
            }
        }
    }

    #[test]
    fn test_two_symbols() {
        // Two symbols always get single-bit codes: "0" and "1"
        let tree = build_tree(&[('a', 3), ('b', 7)]).unwrap();
        let cs: HashMap<char, String> = codes("", &tree).into_iter().collect();
        assert_eq!(cs[&'a'].len(), 1);
        assert_eq!(cs[&'b'].len(), 1);
    }
}
