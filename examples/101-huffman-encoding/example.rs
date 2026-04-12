#![allow(dead_code)]

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

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
                let a = trees.remove(0);
                let b = trees.remove(0);
                let freq = a.freq() + b.freq();
                let merged = HTree::Node(Box::new(a), Box::new(b), freq);
                let mut next: Vec<HTree> = std::iter::once(merged).chain(trees).collect();
                next.sort_by_key(|t| t.freq());
                go(next)
            }
        }
    }

    go(trees)
}

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

fn main() {
    let freqs = [
        ('a', 5),
        ('b', 9),
        ('c', 12),
        ('d', 13),
        ('e', 16),
        ('f', 45),
    ];

    println!("=== Idiomatic (BinaryHeap) ===");
    if let Some(tree) = build_tree(&freqs) {
        let mut c = codes(&tree);
        c.sort_by_key(|(ch, _)| *ch);
        for (ch, code) in &c {
            println!("{ch}: {code}");
        }
    }

    println!("\n=== Recursive (list-sort) ===");
    if let Some(tree) = build_tree_recursive(&freqs) {
        let mut c = codes(&tree);
        c.sort_by_key(|(ch, _)| *ch);
        for (ch, code) in &c {
            println!("{ch}: {code}");
        }
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

   === Recursive (list-sort) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0
*/
