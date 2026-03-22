#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub enum HTree {
    Leaf { ch: char, freq: u32 },
    Node {
        left: Box<HTree>,
        right: Box<HTree>,
        freq: u32,
    },
}

impl HTree {
    pub fn freq(&self) -> u32 {
        match self {
            Self::Leaf { freq, .. } | Self::Node { freq, .. } => *freq,
        }
    }
}

struct MinFreq(HTree);

impl PartialEq for MinFreq {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq() == other.0.freq()
    }
}
impl Eq for MinFreq {}
impl PartialOrd for MinFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MinFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.freq().cmp(&self.0.freq())
    }
}

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut heap: BinaryHeap<MinFreq> = freqs
        .iter()
        .map(|&(ch, freq)| MinFreq(HTree::Leaf { ch, freq }))
        .collect();
    while heap.len() > 1 {
        let a = heap.pop()?.0;
        let b = heap.pop()?.0;
        let freq = a.freq() + b.freq();
        heap.push(MinFreq(HTree::Node {
            left: Box::new(a),
            right: Box::new(b),
            freq,
        }));
    }
    heap.pop().map(|entry| entry.0)
}

pub fn build_tree_functional(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut trees: Vec<HTree> = freqs
        .iter()
        .map(|&(ch, freq)| HTree::Leaf { ch, freq })
        .collect();
    trees.sort_by_key(HTree::freq);
    Some(go(trees))
}

fn go(mut trees: Vec<HTree>) -> HTree {
    match trees.len() {
        0 => panic!("empty frequency list"),
        1 => trees.remove(0),
        _ => {
            let a = trees.remove(0);
            let b = trees.remove(0);
            let freq = a.freq() + b.freq();
            let merged = HTree::Node {
                left: Box::new(a),
                right: Box::new(b),
                freq,
            };
            trees.push(merged);
            trees.sort_by_key(HTree::freq);
            go(trees)
        }
    }
}

pub fn codes(tree: &HTree, prefix: &str) -> Vec<(char, String)> {
    match tree {
        HTree::Leaf { ch, .. } => vec![(*ch, prefix.to_string())],
        HTree::Node { left, right, .. } => {
            let mut result = codes(left, &format!("{prefix}0"));
            result.extend(codes(right, &format!("{prefix}1")));
            result
        }
    }
}

fn main() {
    let freqs = vec![
        ('a', 5),
        ('b', 9),
        ('c', 12),
        ('d', 13),
        ('e', 16),
        ('f', 45),
    ];

    println!("=== BinaryHeap (idiomatic) ===");
    let tree = build_tree(&freqs).expect("non-empty input");
    let mut result = codes(&tree, "");
    result.sort_by_key(|(c, _)| *c);
    for (c, code) in &result {
        println!("{c}: {code}");
    }

    println!("\n=== Sorted Vec (functional/recursive) ===");
    let tree2 = build_tree_functional(&freqs).expect("non-empty input");
    let mut result2 = codes(&tree2, "");
    result2.sort_by_key(|(c, _)| *c);
    for (c, code) in &result2 {
        println!("{c}: {code}");
    }
}

/* Output:
   === BinaryHeap (idiomatic) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0

   === Sorted Vec (functional/recursive) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0
*/
