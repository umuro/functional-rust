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

### Rust (idiomatic)

```rust
pub fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap: BinaryHeap<Reverse<(u32, usize, HTree)>> = freqs
        .iter()
        .enumerate()
        .map(|(i, &(c, f))| Reverse((f, i, HTree::Leaf(c, f))))
        .collect();

    let mut counter = freqs.len();
    loop {
        let Reverse((_, _, a)) = heap.pop()?;
        let b = match heap.pop() {
            Some(Reverse((_, _, node))) => node,
            None => return Some(a),
        };
        let merged_freq = a.freq() + b.freq();
        let merged = HTree::Node(Box::new(a), Box::new(b), merged_freq);
        heap.push(Reverse((merged_freq, counter, merged)));
        counter += 1;
    }
}
```

### Rust (functional/recursive)

```rust
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
            let merged = HTree::Node(Box::new(a), Box::new(b), a_freq + b_freq);
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
| tree type | `type htree = Leaf of char * int \| Node of htree * htree * int` | `enum HTree { Leaf(char, u32), Node(Box<HTree>, Box<HTree>, u32) }` |
| build tree | `val build_tree : (char * int) list -> htree` | `fn build_tree_idiomatic(freqs: &[(char, u32)]) -> Option<HTree>` |
| codes | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree, prefix: &str) -> Vec<(char, String)>` |
| priority queue | `list` re-sorted each step | `BinaryHeap<Reverse<(u32, usize, HTree)>>` |

## Key Insights

1. **Min-heap vs. sorted list:** OCaml re-sorts a list after each merge — O(n log n) per step, O(n² log n) total. Rust's `BinaryHeap` with `Reverse` gives O(log n) per push/pop, O(n log n) total. Both produce the same tree; the heap version is simply more efficient.
2. **Max-heap inversion:** Rust's `BinaryHeap` is a max-heap by design. Wrapping each entry in `std::cmp::Reverse` flips the comparison: the smallest element is now the "largest" in the inverted ordering, so it is popped first. This avoids writing a custom `Ord` implementation.
3. **Recursive type requires Box:** OCaml's `type htree = ... | Node of htree * htree * int` is valid because the runtime represents values as uniform pointers. Rust requires `Box<HTree>` inside the `Node` variant so the compiler can determine the enum's size at compile time — without `Box`, `HTree` would have infinite size.
4. **Empty-input safety:** OCaml's `go` raises `Failure "empty"` on an empty list — a runtime exception. The Rust version returns `Option<HTree>`, making the empty-input case a type-level distinction that callers must handle.
5. **Tie-breaking with a counter:** OCaml's structural comparison of tagged values provides implicit tie-breaking. The Rust tuple `(freq, counter, node)` adds an explicit insertion counter so that equal-frequency nodes are always popped in FIFO order, making the algorithm deterministic regardless of the `HTree`'s internal structure.

## When to Use Each Style

**Use idiomatic Rust (heap) when:** Building the Huffman tree in production — O(n log n) total is optimal, and the code is concise once the `Reverse` wrapper is understood.
**Use recursive Rust (sort each step) when:** Teaching the OCaml parallel or prioritizing code clarity over performance — the re-sort approach mirrors the OCaml source line by line and is easier to verify correct.
