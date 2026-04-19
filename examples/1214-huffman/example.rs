use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
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

struct HeapEntry {
    freq: u32,
    tiebreak: usize,
    tree: HTree,
}
impl PartialEq for HeapEntry {
    fn eq(&self, o: &Self) -> bool {
        (self.freq, self.tiebreak) == (o.freq, o.tiebreak)
    }
}
impl Eq for HeapEntry {}
impl Ord for HeapEntry {
    fn cmp(&self, o: &Self) -> Ordering {
        o.freq
            .cmp(&self.freq)
            .then_with(|| o.tiebreak.cmp(&self.tiebreak))
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut heap: BinaryHeap<HeapEntry> = freqs
        .iter()
        .enumerate()
        .map(|(i, &(c, f))| HeapEntry {
            freq: f,
            tiebreak: i,
            tree: HTree::Leaf(c, f),
        })
        .collect();
    let mut tiebreak = freqs.len();
    while heap.len() > 1 {
        let a = heap.pop()?;
        let b = heap.pop()?;
        let total = a.freq + b.freq;
        heap.push(HeapEntry {
            freq: total,
            tiebreak,
            tree: HTree::Node(Box::new(a.tree), Box::new(b.tree), total),
        });
        tiebreak += 1;
    }
    heap.pop().map(|e| e.tree)
}

pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    fn go(tree: &HTree, prefix: &mut String, out: &mut Vec<(char, String)>) {
        match tree {
            HTree::Leaf(c, _) => out.push((*c, prefix.clone())),
            HTree::Node(l, r, _) => {
                prefix.push('0');
                go(l, prefix, out);
                prefix.pop();
                prefix.push('1');
                go(r, prefix, out);
                prefix.pop();
            }
        }
    }
    let mut prefix = String::new();
    let mut out = Vec::new();
    go(tree, &mut prefix, &mut out);
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
    let tree = build_tree(&freqs).expect("non-empty");
    let mut pairs = codes(&tree);
    pairs.sort_by_key(|p| p.0);
    for (c, code) in &pairs {
        println!("{c}: {code}");
    }
}

/* Output (one valid Huffman assignment — ties may shuffle):
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0
*/
