use std::cmp::Ordering;
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

struct MinHeapNode(HTree);

impl PartialEq for MinHeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq() == other.0.freq()
    }
}

impl Eq for MinHeapNode {}

impl Ord for MinHeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.freq().cmp(&self.0.freq())
    }
}

impl PartialOrd for MinHeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

fn main() {
    let freqs = [
        ('a', 5),
        ('b', 9),
        ('c', 12),
        ('d', 13),
        ('e', 16),
        ('f', 45),
    ];
    let tree = build_tree(&freqs).unwrap();
    let mut cs = codes("", &tree);
    cs.sort_by_key(|&(c, _)| c);
    for (c, code) in &cs {
        println!("{}: {}", c, code);
    }
}

/* Output:
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0
*/
