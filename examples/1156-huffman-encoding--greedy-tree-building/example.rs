use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HTree {
    Leaf { ch: char, freq: u32 },
    Node { left: Box<HTree>, right: Box<HTree>, freq: u32 },
}

impl HTree {
    pub fn freq(&self) -> u32 {
        match self {
            HTree::Leaf { freq, .. } | HTree::Node { freq, .. } => *freq,
        }
    }
}

#[derive(Eq, PartialEq)]
struct MinFreq(Box<HTree>);

impl Ord for MinFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.freq().cmp(&self.0.freq())
    }
}

impl PartialOrd for MinFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }

    let mut heap: BinaryHeap<MinFreq> = freqs
        .iter()
        .map(|&(ch, freq)| MinFreq(Box::new(HTree::Leaf { ch, freq })))
        .collect();

    while heap.len() > 1 {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;
        let freq = a.freq() + b.freq();
        heap.push(MinFreq(Box::new(HTree::Node { left: a, right: b, freq })));
    }

    heap.pop().map(|entry| *entry.0)
}

pub fn build_tree_sorted(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }

    let mut trees: Vec<HTree> = {
        let mut v: Vec<HTree> = freqs
            .iter()
            .map(|&(ch, freq)| HTree::Leaf { ch, freq })
            .collect();
        v.sort_by_key(HTree::freq);
        v
    };

    while trees.len() > 1 {
        let a = trees.remove(0);
        let b = trees.remove(0);
        let freq = a.freq() + b.freq();
        let merged = HTree::Node { left: Box::new(a), right: Box::new(b), freq };
        let pos = trees.partition_point(|t| t.freq() <= freq);
        trees.insert(pos, merged);
    }

    trees.into_iter().next()
}

pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    fn go(tree: &HTree, prefix: String) -> Vec<(char, String)> {
        match tree {
            HTree::Leaf { ch, .. } => vec![(*ch, prefix)],
            HTree::Node { left, right, .. } => {
                let mut result = go(left, format!("{prefix}0"));
                result.extend(go(right, format!("{prefix}1")));
                result
            }
        }
    }
    go(tree, String::new())
}

pub fn encode(input: &str, table: &[(char, String)]) -> Option<String> {
    input
        .chars()
        .map(|c| {
            table
                .iter()
                .find(|(ch, _)| *ch == c)
                .map(|(_, code)| code.as_str())
        })
        .collect()
}

fn main() {
    let freqs = [('a', 5), ('b', 9), ('c', 12), ('d', 13), ('e', 16), ('f', 45)];

    println!("=== Huffman Encoding (BinaryHeap) ===");
    let tree = build_tree(&freqs).unwrap();
    let mut table = codes(&tree);
    table.sort_by_key(|(c, _)| *c);
    for (ch, code) in &table {
        println!("{ch}: {code}");
    }

    println!("\n=== Huffman Encoding (Sorted Vec / OCaml-style) ===");
    let tree2 = build_tree_sorted(&freqs).unwrap();
    let mut table2 = codes(&tree2);
    table2.sort_by_key(|(c, _)| *c);
    for (ch, code) in &table2 {
        println!("{ch}: {code}");
    }

    println!("\n=== Encoding 'face' ===");
    let encoded = encode("face", &table).unwrap();
    println!("face -> {encoded}");
    println!("bits: {}", encoded.len());
}

/* Output:
   === Huffman Encoding (BinaryHeap) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0

   === Huffman Encoding (Sorted Vec / OCaml-style) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0

   === Encoding 'face' ===
   face -> 010011100
   bits: 9
*/
