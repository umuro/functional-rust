#![allow(dead_code)]

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

pub fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree> {
    use std::collections::BinaryHeap;

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
            other
                .freq
                .cmp(&self.freq)
                .then(other.counter.cmp(&self.counter))
        }
    }

    let mut counter = freqs.len();
    let mut heap: BinaryHeap<Item> = freqs
        .iter()
        .enumerate()
        .map(|(i, &(c, f))| Item {
            freq: f,
            counter: i,
            tree: HTree::Leaf(c, f),
        })
        .collect();

    loop {
        let a = heap.pop()?;
        let b = match heap.pop() {
            Some(item) => item,
            None => return Some(a.tree),
        };
        let merged_freq = a.freq + b.freq;
        let merged = HTree::Node(Box::new(a.tree), Box::new(b.tree), merged_freq);
        heap.push(Item {
            freq: merged_freq,
            counter,
            tree: merged,
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
    let freqs = vec![
        ('a', 5u32),
        ('b', 9),
        ('c', 12),
        ('d', 13),
        ('e', 16),
        ('f', 45),
    ];

    println!("=== Idiomatic (BinaryHeap) ===");
    if let Some(tree) = build_tree_idiomatic(&freqs) {
        let mut result = codes(&tree, "");
        result.sort_by_key(|(c, _)| *c);
        for (c, code) in &result {
            println!("{c}: {code}");
        }
    }

    println!("\n=== Recursive (sort-each-round, OCaml style) ===");
    if let Some(tree) = build_tree_recursive(&freqs) {
        let mut result = codes(&tree, "");
        result.sort_by_key(|(c, _)| *c);
        for (c, code) in &result {
            println!("{c}: {code}");
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

   === Recursive (sort-each-round, OCaml style) ===
   a: 1100
   b: 1101
   c: 100
   d: 101
   e: 111
   f: 0
*/
