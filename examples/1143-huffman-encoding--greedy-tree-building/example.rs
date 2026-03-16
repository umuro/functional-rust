use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
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
        Reverse(self.0.freq()).cmp(&Reverse(other.0.freq()))
    }
}

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut heap: BinaryHeap<MinTree> = freqs
        .iter()
        .map(|&(c, f)| MinTree(HTree::Leaf(c, f)))
        .collect();

    while heap.len() > 1 {
        let MinTree(a) = heap.pop().unwrap();
        let MinTree(b) = heap.pop().unwrap();
        let combined = a.freq() + b.freq();
        heap.push(MinTree(HTree::Node(Box::new(a), Box::new(b), combined)));
    }

    heap.pop().map(|MinTree(t)| t)
}

pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut trees: Vec<HTree> = freqs
        .iter()
        .map(|&(c, f)| HTree::Leaf(c, f))
        .collect();
    trees.sort_by_key(HTree::freq);
    go(trees)
}

fn go(mut trees: Vec<HTree>) -> Option<HTree> {
    match trees.len() {
        0 => None,
        1 => trees.into_iter().next(),
        _ => {
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

pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    let mut result = Vec::new();
    collect_codes(tree, String::new(), &mut result);
    result
}

fn collect_codes(tree: &HTree, prefix: String, acc: &mut Vec<(char, String)>) {
    match tree {
        HTree::Leaf(c, _) => acc.push((*c, prefix)),
        HTree::Node(left, right, _) => {
            collect_codes(left, format!("{prefix}0"), acc);
            collect_codes(right, format!("{prefix}1"), acc);
        }
    }
}

fn main() {
    let freqs = [('a', 5), ('b', 9), ('c', 12), ('d', 13), ('e', 16), ('f', 45)];

    println!("=== BinaryHeap (idiomatic) ===");
    let tree = build_tree(&freqs).unwrap();
    let mut c = codes(&tree);
    c.sort_by_key(|(ch, _)| *ch);
    for (ch, code) in &c {
        println!("{ch}: {code}");
    }

    println!("\n=== Recursive (OCaml-style) ===");
    let tree2 = build_tree_recursive(&freqs).unwrap();
    let mut c2 = codes(&tree2);
    c2.sort_by_key(|(ch, _)| *ch);
    for (ch, code) in &c2 {
        println!("{ch}: {code}");
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

   === Recursive (OCaml-style) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0
*/
