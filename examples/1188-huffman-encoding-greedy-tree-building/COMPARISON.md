# OCaml vs Rust: Huffman Encoding — Greedy Tree Building

## Side-by-Side Code

### OCaml
```ocaml
type htree = Leaf of char * int | Node of htree * htree * int

let freq t = match t with Leaf (_,f) -> f | Node (_,_,f) -> f

let build_tree freqs =
  let trees = List.map (fun (c,f) -> Leaf (c,f)) freqs
    |> List.sort (fun a b -> compare (freq a) (freq b)) in
  let rec go = function
    | [t] -> t
    | a :: b :: rest ->
      let merged = Node (a, b, freq a + freq b) in
      let trees = List.sort (fun a b -> compare (freq a) (freq b)) (merged :: rest) in
      go trees
    | [] -> failwith "empty"
  in go trees

let rec codes prefix = function
  | Leaf (c, _) -> [(c, prefix)]
  | Node (l, r, _) -> codes (prefix ^ "0") l @ codes (prefix ^ "1") r
```

### Rust (idiomatic — BinaryHeap min-heap)
```rust
struct Entry { freq: u32, seq: usize, tree: HTree }

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)          // inverted → min-heap
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

pub fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut heap: BinaryHeap<Entry> = freqs.iter().enumerate()
        .map(|(seq, &(c, f))| Entry { freq: f, seq, tree: HTree::Leaf(c, f) })
        .collect();
    let mut counter = freqs.len();
    loop {
        let a = heap.pop()?.tree;
        let b = match heap.pop() { Some(e) => e.tree, None => return Some(a) };
        let merged_freq = a.freq() + b.freq();
        heap.push(Entry { freq: merged_freq, seq: counter,
            tree: HTree::Node(Box::new(a), Box::new(b), merged_freq) });
        counter += 1;
    }
}
```

### Rust (functional/recursive — mirrors OCaml)
```rust
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut trees: Vec<HTree> = freqs.iter()
        .map(|&(c, f)| HTree::Leaf(c, f)).collect();
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
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type htree = Leaf of char * int \| Node of htree * htree * int` | `enum HTree { Leaf(char, u32), Node(Box<HTree>, Box<HTree>, u32) }` |
| Frequency accessor | `val freq : htree -> int` | `fn freq(&self) -> u32` (method on `HTree`) |
| Build function | `val build_tree : (char * int) list -> htree` | `fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree>` |
| Code extraction | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree, prefix: &str) -> Vec<(char, String)>` |
| Optional result | `failwith "empty"` (exception) | `Option<HTree>` (None for empty input) |

## Key Insights

1. **Recursive enum needs `Box`:** OCaml's `htree` variant is implicitly heap-allocated — recursive constructors just work. Rust must break the infinite-size cycle explicitly: `Node(Box<HTree>, Box<HTree>, u32)`. Without `Box`, the compiler rejects the type.

2. **`BinaryHeap` is max-first — invert to get min-heap:** Rust's standard library only provides a max-heap. To get min-heap behavior, you implement `Ord` with an inverted comparison (`other.freq.cmp(&self.freq)` instead of `self.freq.cmp(&other.freq)`). The common alternative, `std::cmp::Reverse`, cannot be used here because it requires the wrapped type to implement `Ord`, and `HTree` contains `Box<HTree>` which has no natural total order.

3. **Tie-breaking with a sequence counter:** When two trees have equal frequency, a sequence counter ensures deterministic FIFO ordering. This mirrors OCaml's `List.sort` stability (OCaml's sort is stable; Rust's `sort_by_key` is also stable, but `BinaryHeap` is not a stable data structure).

4. **Error handling — exceptions vs `Option`:** OCaml's `go` on an empty list calls `failwith "empty"`, throwing an exception. Rust encodes this as `Option<HTree>` — `None` for empty input — forcing callers to handle the empty case explicitly without runtime panics.

5. **Algorithmic complexity difference:** The OCaml and Rust recursive versions both re-sort the list every iteration: O(n² log n) total. The idiomatic Rust version uses a `BinaryHeap` for O(n log n) total — each insert/pop is O(log n) and there are O(n) merges.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** building Huffman trees in production — the O(n log n) heap approach scales well and is the standard algorithm implementation.

**Use recursive Rust (sort each round) when:** translating OCaml code for pedagogical comparison, or when the input is small enough that algorithmic complexity doesn't matter and structural clarity is preferred.
