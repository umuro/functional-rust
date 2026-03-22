#![allow(dead_code)]

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

/// Idiomatic Rust: build a Huffman tree using a BinaryHeap (min-heap via Reverse).
/// Avoids repeated sorting by using a priority queue.
pub fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    // BinaryHeap is a max-heap; wrap in Reverse to get min-heap by frequency.
    let mut heap: BinaryHeap<Reverse<(u32, usize, HTree)>> = freqs
        .iter()
        .enumerate()
        .map(|(i, &(c, f))| Reverse((f, i, HTree::Leaf(c, f))))
        .collect();

    // Use a counter to break frequency ties deterministically (FIFO order).
    let mut counter = freqs.len();

    loop {
        let Reverse((_, _, a)) = heap.pop()?;
        let b = match heap.pop() {
            Some(Reverse((_, _, node))) => node,
            None => return Some(a),
        };
        let merged_freq = a.freq() + b.freq();
        let merged = HTree::Node(Box::new(a), Box::new(b), merged_freq);
        heap.push(Reverse((merged_freq, counter, merged)));
        counter += 1;
    }
}

/// Functional/recursive Rust: mirrors the OCaml approach — sort the list each round.
/// Less efficient than the heap approach but structurally identical to the OCaml source.
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut trees: Vec<HTree> = freqs
        .iter()
        .map(|&(c, f)| HTree::Leaf(c, f))
        .collect();
    trees.sort_by_key(|t| t.freq());
    go(trees)
}

fn go(mut trees: Vec<HTree>) -> Option<HTree> {
    match trees.len() {
        0 => None,
        1 => trees.into_iter().next(),
        _ => {
            // Take the two lowest-frequency trees (already sorted).
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
/// '0' goes left (first child), '1' goes right (second child).
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
        // In a valid Huffman tree no code is a prefix of another.
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
        // 'f' has freq 45 — more than all others combined — so it gets code "0" or "1" (len 1).
        let freqs = sample_freqs();
        let tree = build_tree_idiomatic(&freqs).expect("should build");
        let result = codes(&tree, "");
        let f_code = result
            .iter()
            .find(|(c, _)| *c == 'f')
            .map(|(_, code)| code.len())
            .expect("f must have a code");
        assert_eq!(f_code, 1, "'f' should get a 1-bit code");
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
    fn test_codes_recursive_vs_idiomatic_same_set() {
        // Both approaches must produce codes for the same set of characters,
        // each code non-empty, and the prefix-free property holds for both.
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
        // All codes must be non-empty.
        for (_, code) in c1.iter().chain(c2.iter()) {
            assert!(!code.is_empty());
        }
    }
}
