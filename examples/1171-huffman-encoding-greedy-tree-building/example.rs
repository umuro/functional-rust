use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
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

pub fn build_tree_sorted(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut trees: Vec<HTree> = freqs
        .iter()
        .map(|&(c, f)| HTree::Leaf(c, f))
        .collect();
    trees.sort_by_key(HTree::freq);

    fn go(mut trees: Vec<HTree>) -> HTree {
        if trees.len() == 1 {
            return trees.remove(0);
        }
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
    let tree = build_tree(&freqs).unwrap();
    let mut cs = codes(&tree);
    cs.sort_by_key(|(c, _)| *c);
    for (c, code) in &cs {
        println!("{c}: {code}");
    }

    println!("\n=== Functional/sorted (OCaml style) ===");
    let tree2 = build_tree_sorted(&freqs).unwrap();
    let mut cs2 = codes(&tree2);
    cs2.sort_by_key(|(c, _)| *c);
    for (c, code) in &cs2 {
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

   === Functional/sorted (OCaml style) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0
*/
