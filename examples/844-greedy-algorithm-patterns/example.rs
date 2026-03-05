/// Greedy Algorithms: Activity Selection and Huffman Coding.
///
/// Activity selection: sort by finish time, greedily pick non-overlapping.
/// Huffman: repeatedly merge two lowest-frequency nodes.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

// ─── Activity Selection ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Activity {
    id: usize,
    start: u64,
    finish: u64,
}

/// Return maximum set of non-overlapping activities (greedy by earliest finish).
fn activity_selection(activities: &[Activity]) -> Vec<&Activity> {
    let mut sorted: Vec<&Activity> = activities.iter().collect();
    sorted.sort_by_key(|a| a.finish);

    let mut selected = Vec::new();
    let mut last_finish = 0u64;

    for a in sorted {
        if a.start >= last_finish {
            selected.push(a);
            last_finish = a.finish;
        }
    }
    selected
}

// ─── Huffman Coding ──────────────────────────────────────────────────────────

#[derive(Debug)]
enum HuffmanNode {
    Leaf { symbol: char, freq: u64 },
    Internal { freq: u64, left: Box<HuffmanNode>, right: Box<HuffmanNode> },
}

impl HuffmanNode {
    fn freq(&self) -> u64 {
        match self {
            HuffmanNode::Leaf { freq, .. } => *freq,
            HuffmanNode::Internal { freq, .. } => *freq,
        }
    }
}

// Wrapper for BinaryHeap (min-heap via Reverse)
struct HeapItem(Box<HuffmanNode>);

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool { self.0.freq() == other.0.freq() }
}
impl Eq for HeapItem {}
impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min-heap: invert comparison
        other.0.freq().cmp(&self.0.freq())
    }
}

/// Build Huffman tree from (symbol, frequency) pairs.
fn huffman_tree(symbols: &[(char, u64)]) -> Option<Box<HuffmanNode>> {
    if symbols.is_empty() { return None; }

    let mut heap: BinaryHeap<HeapItem> = symbols
        .iter()
        .map(|&(symbol, freq)| HeapItem(Box::new(HuffmanNode::Leaf { symbol, freq })))
        .collect();

    while heap.len() > 1 {
        let HeapItem(left) = heap.pop().unwrap();
        let HeapItem(right) = heap.pop().unwrap();
        let freq = left.freq() + right.freq();
        let merged = Box::new(HuffmanNode::Internal { freq, left, right });
        heap.push(HeapItem(merged));
    }

    heap.pop().map(|HeapItem(node)| node)
}

/// Extract code table: symbol → binary string.
fn huffman_codes(node: &HuffmanNode) -> HashMap<char, String> {
    let mut codes = HashMap::new();
    traverse(node, String::new(), &mut codes);
    codes
}

fn traverse(node: &HuffmanNode, prefix: String, codes: &mut HashMap<char, String>) {
    match node {
        HuffmanNode::Leaf { symbol, .. } => {
            codes.insert(*symbol, if prefix.is_empty() { "0".to_string() } else { prefix });
        }
        HuffmanNode::Internal { left, right, .. } => {
            traverse(left, format!("{prefix}0"), codes);
            traverse(right, format!("{prefix}1"), codes);
        }
    }
}

/// Compute total encoded bits for a message.
fn encoded_bits(text: &str, codes: &HashMap<char, String>) -> usize {
    text.chars().map(|c| codes[&c].len()).sum()
}

fn main() {
    // Activity selection
    let activities = vec![
        Activity { id: 1, start: 1, finish: 4 },
        Activity { id: 2, start: 3, finish: 5 },
        Activity { id: 3, start: 0, finish: 6 },
        Activity { id: 4, start: 5, finish: 7 },
        Activity { id: 5, start: 3, finish: 9 },
        Activity { id: 6, start: 5, finish: 9 },
        Activity { id: 7, start: 6, finish: 10 },
        Activity { id: 8, start: 8, finish: 11 },
    ];
    let selected = activity_selection(&activities);
    let ids: Vec<usize> = selected.iter().map(|a| a.id).collect();
    println!("Activity selection: {} selected, ids={ids:?}", selected.len());

    // Huffman
    let freqs = [('a', 5u64), ('b', 2), ('c', 1), ('d', 3), ('e', 4), ('f', 7)];
    let tree = huffman_tree(&freqs).unwrap();
    let mut codes: Vec<(char, String)> = huffman_codes(&tree).into_iter().collect();
    codes.sort();
    println!("\nHuffman codes:");
    for (c, code) in &codes {
        let freq = freqs.iter().find(|&&(ch, _)| ch == *c).map(|&(_, f)| f).unwrap();
        println!("  '{c}' (freq={freq}): {code}");
    }
    let total_bits: u64 = freqs.iter().map(|&(c, f)| {
        let code = codes.iter().find(|(ch, _)| *ch == c).map(|(_, code)| code.len()).unwrap();
        f * code as u64
    }).sum();
    println!("Total weighted bits: {total_bits}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_selection_count() {
        let acts = vec![
            Activity { id: 1, start: 1, finish: 4 },
            Activity { id: 2, start: 3, finish: 5 },
            Activity { id: 3, start: 0, finish: 6 },
            Activity { id: 4, start: 5, finish: 7 },
            Activity { id: 5, start: 8, finish: 11 },
        ];
        let sel = activity_selection(&acts);
        assert_eq!(sel.len(), 3);
    }

    #[test]
    fn test_activity_selection_non_overlapping() {
        let acts = vec![
            Activity { id: 1, start: 0, finish: 2 },
            Activity { id: 2, start: 1, finish: 3 },
            Activity { id: 3, start: 2, finish: 4 },
        ];
        let sel = activity_selection(&acts);
        // Non-overlapping: check no two overlap
        for i in 0..sel.len() {
            for j in i + 1..sel.len() {
                assert!(sel[i].finish <= sel[j].start || sel[j].finish <= sel[i].start);
            }
        }
    }

    #[test]
    fn test_huffman_prefix_free() {
        let freqs = [('a', 5u64), ('b', 2), ('c', 1), ('d', 3)];
        let tree = huffman_tree(&freqs).unwrap();
        let codes = huffman_codes(&tree);
        // Prefix-free: no code is a prefix of another
        let code_list: Vec<String> = codes.values().cloned().collect();
        for (i, a) in code_list.iter().enumerate() {
            for (j, b) in code_list.iter().enumerate() {
                if i != j {
                    assert!(!b.starts_with(a.as_str()), "{b} starts with {a}");
                }
            }
        }
    }

    #[test]
    fn test_huffman_all_symbols_covered() {
        let freqs = [('a', 1u64), ('b', 2), ('c', 3), ('d', 4)];
        let tree = huffman_tree(&freqs).unwrap();
        let codes = huffman_codes(&tree);
        assert_eq!(codes.len(), 4);
        for (c, _) in &freqs {
            assert!(codes.contains_key(c), "'{c}' missing from codes");
        }
    }
}
