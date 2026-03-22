#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

struct Entry {
    freq: u32,
    seq: usize,
    tree: HTree,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.seq == other.seq
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .freq
            .cmp(&self.freq)
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

pub fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut heap: BinaryHeap<Entry> = freqs
        .iter()
        .enumerate()
        .map(|(seq, &(c, f))| Entry {
            freq: f,
            seq,
            tree: HTree::Leaf(c, f),
        })
        .collect();

    let mut counter = freqs.len();

    loop {
        let a = heap.pop()?.tree;
        let b = match heap.pop() {
            Some(entry) => entry.tree,
            None => return Some(a),
        };
        let merged_freq = a.freq() + b.freq();
        heap.push(Entry {
            freq: merged_freq,
            seq: counter,
            tree: HTree::Node(Box::new(a), Box::new(b), merged_freq),
        });
        counter += 1;
    }
}

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
    let freqs = [('a', 5), ('b', 9), ('c', 12), ('d', 13), ('e', 16), ('f', 45)];

    println!("=== Idiomatic (BinaryHeap) ===");
    let tree = build_tree_idiomatic(&freqs).expect("non-empty");
    let mut result = codes(&tree, "");
    result.sort_by_key(|(c, _)| *c);
    for (c, code) in &result {
        println!("{c}: {code}");
    }

    println!("\n=== Recursive (sort each round) ===");
    let tree2 = build_tree_recursive(&freqs).expect("non-empty");
    let mut result2 = codes(&tree2, "");
    result2.sort_by_key(|(c, _)| *c);
    for (c, code) in &result2 {
        println!("{c}: {code}");
    }
}

/* Output:
   === Idiomatic (BinaryHeap) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0

   === Recursive (sort each round) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0
*/
