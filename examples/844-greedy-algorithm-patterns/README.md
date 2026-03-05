📖 **[View on hightechmind.io →](https://hightechmind.io/rust/844-greedy-algorithm-patterns)**

---

# 844: Greedy Algorithm Patterns — Activity Selection and Huffman Coding

**Difficulty:** 3  **Level:** Intermediate

Make the locally optimal choice at each step — and prove it gives the global optimum for the right problems.

## The Problem This Solves

A greedy algorithm makes the locally optimal decision at each step without revisiting previous choices. It's correct when the problem has two properties: the **greedy choice property** (a globally optimal solution can always be built by making the locally optimal choice) and **optimal substructure** (the optimal solution to the whole problem contains optimal solutions to subproblems). When these hold, greedy is far simpler and faster than dynamic programming.

This example implements two canonical greedy problems:
- **Activity selection**: given n intervals with start/end times, find the maximum number of non-overlapping intervals. Application: schedule the most meetings in a conference room.
- **Huffman coding**: given character frequencies, build the optimal prefix-free binary code that minimises total encoded length. Application: lossless compression in gzip/DEFLATE, JPEG/PNG header compression.

Both problems have O(n log n) greedy solutions — orders of magnitude faster than their DP counterparts would be if you tried them on large inputs.

## The Intuition

**Activity selection**: sort by finish time. Greedily pick each activity if it doesn't overlap the last selected one. Why does earliest-finish work? Any activity that finishes earlier leaves more room for future activities — swapping any optimal choice with an earlier-finishing one can only help.

**Huffman coding**: build a priority queue (min-heap) of characters by frequency. Repeatedly extract the two least-frequent nodes, create a new internal node with their combined frequency, and re-insert. The final tree assigns shorter codes to frequent characters and longer codes to rare ones — provably minimising total encoded length.

Activity selection: O(n log n) to sort, O(n) to scan. Huffman: O(n log n) with a binary heap. Both are elegant proofs by exchange argument: assume a better solution exists, swap its first differing choice with the greedy choice, and show the result is no worse.

## How It Works in Rust

```rust
// Activity Selection: maximum non-overlapping intervals
fn activity_selection(intervals: &[(usize, usize)]) -> Vec<usize> {
    let mut sorted: Vec<(usize, usize, usize)> = intervals.iter()
        .enumerate()
        .map(|(i, &(s, e))| (e, s, i)) // (end, start, id) for sort-by-end
        .collect();
    sorted.sort_unstable(); // sort by end time (tuple sort: first key wins)

    let mut selected = vec![];
    let mut last_end = 0;

    for (end, start, id) in sorted {
        if start >= last_end {
            selected.push(id);
            last_end = end;
        }
    }
    selected
}

// Huffman Coding: optimal prefix-free code from character frequencies
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
enum HuffTree {
    Leaf { ch: char, freq: usize },
    Node { freq: usize, left: Box<HuffTree>, right: Box<HuffTree> },
}

impl HuffTree {
    fn freq(&self) -> usize {
        match self { HuffTree::Leaf { freq, .. } | HuffTree::Node { freq, .. } => *freq }
    }
}

fn huffman(freqs: &[(char, usize)]) -> Option<HuffTree> {
    // Min-heap via Reverse wrapper: Reverse(freq) gives min at top
    let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    let mut trees: Vec<HuffTree> = freqs.iter()
        .map(|&(ch, freq)| HuffTree::Leaf { ch, freq })
        .collect();

    // Push indices into heap sorted by frequency
    for (i, t) in trees.iter().enumerate() {
        heap.push((Reverse(t.freq()), i));
    }

    // Repeatedly merge two minimum-frequency trees
    while heap.len() > 1 {
        let (_, i) = heap.pop().unwrap();
        let (_, j) = heap.pop().unwrap();
        // Move out of trees vec using swap with placeholder (avoids clone)
        let left  = std::mem::replace(&mut trees[i], HuffTree::Leaf { ch: '\0', freq: 0 });
        let right = std::mem::replace(&mut trees[j], HuffTree::Leaf { ch: '\0', freq: 0 });
        let combined_freq = left.freq() + right.freq();
        let new_idx = trees.len();
        trees.push(HuffTree::Node {
            freq: combined_freq,
            left: Box::new(left),
            right: Box::new(right),
        });
        heap.push((Reverse(combined_freq), new_idx));
    }

    heap.pop().map(|(_, i)| std::mem::replace(
        &mut trees[i], HuffTree::Leaf { ch: '\0', freq: 0 }
    ))
}

// Extract codes: left = '0', right = '1'
fn huffman_codes(tree: &HuffTree) -> Vec<(char, String)> {
    let mut codes = vec![];
    fn walk(node: &HuffTree, prefix: String, codes: &mut Vec<(char, String)>) {
        match node {
            HuffTree::Leaf { ch, .. } => codes.push((*ch, prefix)),
            HuffTree::Node { left, right, .. } => {
                walk(left,  prefix.clone() + "0", codes);
                walk(right, prefix        + "1", codes);
            }
        }
    }
    walk(tree, String::new(), &mut codes);
    codes
}
```

`Reverse<usize>` wraps a value to invert `BinaryHeap`'s default max-heap behavior — `Reverse(3) < Reverse(5)` is true, so the heap pops smallest frequency first. This is the idiomatic Rust min-heap pattern.

`std::mem::replace` moves values out of a Vec without cloning — essential when the type doesn't implement `Copy`. The placeholder `HuffTree::Leaf { ch: '\0', freq: 0 }` is never used; it's just dropped.

For activity selection, packing `(end, start, id)` as a tuple sorts by `end` first naturally — Rust's derived `Ord` on tuples compares lexicographically.

## What This Unlocks

- **Compression codecs**: Huffman coding is used in DEFLATE (gzip/zlib/PNG), JPEG coefficient encoding, and many custom binary formats — understanding it means understanding a piece of every compressed file on your system.
- **Scheduling systems**: activity selection underlies greedy schedulers (earliest-deadline-first, shortest-job-first) and interval graph colouring for resource allocation.
- **Algorithm design pattern**: recognising when a problem has greedy choice property saves you from implementing O(n²) or O(n³) DP where O(n log n) greedy suffices.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Min-heap | `priority_queue` library or manual | `BinaryHeap` + `Reverse<T>` wrapper — standard library |
| Recursive tree ADT | `type tree = Leaf of ... \| Node of ...` | `enum HuffTree { Leaf { .. }, Node { .. } }` |
| Move out of Vec | Pattern match (GC handles) | `std::mem::replace` — explicit move with placeholder |
| Sort by end time | `List.sort (fun (s1,e1,_) (s2,e2,_) -> compare e1 e2)` | Tuple sort `(end, start, id)` — lexicographic by default |
| Greedy termination | Pattern match on empty list | `while heap.len() > 1` loop |
