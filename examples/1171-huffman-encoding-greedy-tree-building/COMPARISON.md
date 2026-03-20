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
pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() { return None; }
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
```

### Rust (functional/recursive — mirrors OCaml sort-per-step)
```rust
pub fn build_tree_sorted(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() { return None; }
    let mut trees: Vec<HTree> = freqs
        .iter()
        .map(|&(c, f)| HTree::Leaf(c, f))
        .collect();
    trees.sort_by_key(HTree::freq);

    fn go(mut trees: Vec<HTree>) -> HTree {
        if trees.len() == 1 { return trees.remove(0); }
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
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type htree = Leaf of char * int \| Node of htree * htree * int` | `enum HTree { Leaf(char, u32), Node(Box<HTree>, Box<HTree>, u32) }` |
| Frequency accessor | `val freq : htree -> int` | `fn freq(&self) -> u32` (method on `HTree`) |
| Tree builder | `val build_tree : (char * int) list -> htree` | `fn build_tree(freqs: &[(char, u32)]) -> Option<HTree>` |
| Code extractor | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree) -> Vec<(char, String)>` |
| Optional result | `failwith "empty"` (raises exception) | `None` (returns `Option`) |

## Key Insights

1. **Recursive type indirection:** OCaml's GC handles self-referential types naturally. Rust requires `Box<HTree>` to break the infinite-size cycle — without it, the compiler cannot determine the stack size of `HTree` at compile time. This single difference cascades through every tree construction site.

2. **Min-heap via newtype + `Reverse`:** `BinaryHeap` is a max-heap. Rust's standard approach is `BinaryHeap<Reverse<T>>` where `T: Ord`. Since ordering by frequency doesn't match structural equality of `HTree`, we introduce a `ByFreq` newtype that implements `Ord` based on `freq()` alone — then wrap it in `Reverse` for min-heap semantics. This is more explicit than OCaml's comparison function but makes the ordering visible and enforced by the type system.

3. **Ownership vs. list cons:** OCaml's `a :: b :: rest` destructs a list by borrowing its head elements. Rust cannot destructure a `Vec` this way — elements must be moved out explicitly via `remove(0)`. This is O(n) per removal but matches the algorithm's logical intent.

4. **Error handling strategy:** OCaml raises `failwith "empty"` for an empty input, propagating an exception. Rust returns `Option<HTree>` — the caller must handle the `None` case. This makes the partial nature of the function explicit in the type signature rather than documented only in prose or by convention.

5. **Code generation as method vs. argument:** The OCaml `codes` takes the `prefix` as an explicit parameter, threading it through recursion in the public signature. The Rust version hides this accumulator in a private inner function `go`, exposing a cleaner `codes(tree: &HTree) -> Vec<(char, String)>` API. Both approaches are idiomatic in their respective languages.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap):** When building Huffman trees in production code or with large alphabets (e.g., Unicode). The heap maintains the invariant incrementally at O(log n) per step vs O(n log n) for re-sorting — a significant win at scale.

**Use recursive/sorted Rust:** When the goal is pedagogical clarity or direct correspondence to a reference algorithm (textbook pseudocode, OCaml original). The sort-per-step approach is simpler to reason about correctness and matches most written descriptions of the greedy Huffman algorithm step-by-step.
