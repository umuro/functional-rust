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
        other.0.freq().cmp(&self.0.freq())
    }
}

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
        0 => panic!("go: empty list"),
        1 => trees.remove(0),
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

fn main() {
    let freqs: &[(char, u32)] = &[
        ('a', 5),
        ('b', 9),
        ('c', 12),
        ('d', 13),
        ('e', 16),
        ('f', 45),
    ];

    println!("=== Idiomatic (BinaryHeap) ===");
    let tree = build_tree_idiomatic(freqs).unwrap();
    println!("Root frequency: {}", tree.freq());
    let mut result = codes(&tree);
    result.sort_by_key(|&(c, _)| c);
    for (c, code) in &result {
        println!("{c}: {code}");
    }

    println!("\n=== Recursive (sort-and-merge) ===");
    let tree2 = build_tree_recursive(freqs).unwrap();
    let mut result2 = codes(&tree2);
    result2.sort_by_key(|&(c, _)| c);
    for (c, code) in &result2 {
        println!("{c}: {code}");
    }
}

/* Output:
   === Idiomatic (BinaryHeap) ===
   Root frequency: 100
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0

   === Recursive (sort-and-merge) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0
*/
