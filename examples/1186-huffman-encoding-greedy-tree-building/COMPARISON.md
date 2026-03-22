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
pub fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree> {
    use std::collections::BinaryHeap;

    struct Item { freq: u32, counter: usize, tree: HTree }

    impl Ord for Item {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.freq.cmp(&self.freq).then(other.counter.cmp(&self.counter))
        }
    }
    // ... PartialOrd, PartialEq, Eq derived from Ord

    let mut counter = freqs.len();
    let mut heap: BinaryHeap<Item> = freqs.iter().enumerate()
        .map(|(i, &(c, f))| Item { freq: f, counter: i, tree: HTree::Leaf(c, f) })
        .collect();

    loop {
        let a = heap.pop()?;
        let b = match heap.pop() { Some(x) => x, None => return Some(a.tree) };
        let merged_freq = a.freq + b.freq;
        heap.push(Item {
            freq: merged_freq, counter,
            tree: HTree::Node(Box::new(a.tree), Box::new(b.tree), merged_freq),
        });
        counter += 1;
    }
}
```

### Rust (functional/recursive — mirrors OCaml sort-each-round)

```rust
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut trees: Vec<HTree> = freqs.iter()
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
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type htree = Leaf of char * int \| Node of htree * htree * int` | `enum HTree { Leaf(char, u32), Node(Box<HTree>, Box<HTree>, u32) }` |
| Build function | `val build_tree : (char * int) list -> htree` | `fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree>` |
| Codes function | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree, prefix: &str) -> Vec<(char, String)>` |
| Failure case | `failwith "empty"` (exception) | `None` (type-safe) |
| Recursive child | `htree` (GC-managed) | `Box<HTree>` (heap allocation) |

## Key Insights

1. **Recursive types need `Box`**: OCaml's GC handles recursive ADTs transparently. Rust enums must have a known size at compile time, so recursive variants wrap children in `Box<T>` to place them on the heap. This makes the allocation explicit but not burdensome.

2. **Priority queues and `Ord`**: Rust's `BinaryHeap` requires elements to implement `Ord`. Deriving `Ord` on `HTree` would be semantically incorrect (trees have no natural total order). The solution is a local `Item` wrapper struct that implements `Ord` by frequency only — a common Rust pattern to give domain-specific ordering to types without polluting their public interface.

3. **`Option` vs exceptions for empty input**: OCaml uses `failwith "empty"` — an unchecked exception. Rust's `Option<HTree>` forces the caller to handle the empty case at compile time. The `?` operator and `heap.pop()?` propagate `None` cleanly without explicit pattern matching.

4. **Algorithmic upgrade is natural**: Because Rust's standard library provides `BinaryHeap`, replacing repeated `Vec::sort` (O(n² log n)) with a heap (O(n log n)) is a natural step. OCaml's standard library lacks a priority queue, so the sort-each-round approach is the idiomatic baseline there.

5. **String concatenation**: OCaml uses `prefix ^ "0"` for string concatenation. Rust uses `format!("{prefix}0")`, which allocates a new `String` — identical semantics, explicit allocation. For performance-critical code one would pass a `&mut String` and push characters, but for clarity `format!` is idiomatic here.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** building real compression utilities where O(n log n) matters, or when the input alphabet is large (e.g., Unicode).

**Use recursive Rust (sort-each-round) when:** teaching the algorithm, translating OCaml code for comparison, or when the input is tiny and clarity outweighs performance.
