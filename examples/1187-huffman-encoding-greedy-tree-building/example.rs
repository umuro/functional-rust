#![allow(dead_code)]

use std::collections::BinaryHeap;

/// A Huffman tree node.
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

/// Build a Huffman tree using a BinaryHeap min-heap.
pub fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree> {
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
            other.freq.cmp(&self.freq).then(other.counter.cmp(&self.counter))
        }
    }
    let mut counter = freqs.len();
    let mut heap: BinaryHeap<Item> = freqs
        .iter()
        .enumerate()
        .map(|(i, &(c, f))| Item { freq: f, counter: i, tree: HTree::Leaf(c, f) })
        .collect();
    loop {
        let a = heap.pop()?;
        let b = match heap.pop() { Some(b) => b, None => return Some(a.tree) };
        let merged_freq = a.freq + b.freq;
        let merged = HTree::Node(Box::new(a.tree), Box::new(b.tree), merged_freq);
        heap.push(Item { freq: merged_freq, counter, tree: merged });
        counter += 1;
    }
}

/// Recursively collect (char, code) pairs from the tree.
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

fn main() {
    let freqs = vec![
        ('a', 5), ('b', 9), ('c', 12), ('d', 13), ('e', 16), ('f', 45),
    ];
    let tree = build_tree_idiomatic(&freqs).expect("non-empty input");
    println!("Root frequency: {} (sum of all = {})",
        tree.freq(),
        freqs.iter().map(|(_, f)| f).sum::<u32>()
    );
    let mut result = codes(&tree, "");
    result.sort_by_key(|(c, _)| *c);
    for (c, code) in &result {
        println!("  {}: {}", c, code);
    }
}

/* Output:
   Root frequency: 100 (sum of all = 100)
     a: 1100
     b: 1101
     c: 100
     d: 101
     e: 111
     f: 0
*/
