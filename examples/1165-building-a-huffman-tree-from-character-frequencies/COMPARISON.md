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
    use std::collections::BinaryHeap;

    struct Item { freq: u32, counter: usize, tree: HTree }
    // Custom Ord reverses freq comparison → min-heap by frequency.

    let mut heap: BinaryHeap<Item> = freqs.iter().enumerate()
        .map(|(i, &(c, f))| Item { freq: f, counter: i, tree: HTree::Leaf(c, f) })
        .collect();
    loop {
        let a = heap.pop()?;
        let b = match heap.pop() { Some(b) => b, None => return Some(a.tree) };
        let merged_freq = a.freq + b.freq;
        let merged = HTree::Node(Box::new(a.tree), Box::new(b.tree), merged_freq);
        heap.push(Item { freq: merged_freq, counter, tree: merged });
    }
}
```

### Rust (functional/recursive — sort-each-round)
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
| Recursive type | automatic (GC allocates) | requires `Box<HTree>` to break the size cycle |
| Priority queue | `List.sort` each round (O(n² log n)) | `BinaryHeap` (O(n log n)) |
| Error handling | `failwith "empty"` (exception) | `Option<HTree>` (type-safe) |
| Code result | `(char * string) list` | `Vec<(char, String)>` |

## Key Insights

1. **Recursive type**: OCaml allows recursive ADTs directly; Rust requires `Box<HTree>` to break the infinite-size cycle because the compiler must know the size of every type at compile time.
2. **Priority queue**: OCaml's functional approach re-sorts the entire list each round (O(n² log n)); Rust's `BinaryHeap` maintains heap order, giving O(n log n) overall.
3. **Ord requirement**: Rust's `BinaryHeap` requires `Ord` on its elements. Since `HTree` has no natural ordering, a wrapper struct with a custom `Ord` (ordering only by frequency and a tie-breaking counter) is needed.
4. **Error handling**: OCaml raises `failwith "empty"` for invalid input; Rust returns `Option<HTree>`, forcing callers to handle the empty case at compile time rather than at runtime.
5. **Prefix-free codes**: Both implementations produce valid Huffman codes where no code is a prefix of another, because characters appear only in leaves of the tree.

## When to Use Each Style

**Use `BinaryHeap` when:** performance matters — O(n log n) vs O(n² log n) for large frequency tables.
**Use recursive sort-each-round when:** clarity and the OCaml parallel are the goal — the algorithm structure maps one-to-one with the OCaml source and is easier to verify by inspection.
