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
    let mut heap: BinaryHeap<MinTree> = freqs
        .iter()
        .map(|&(c, f)| MinTree(HTree::Leaf(c, f)))
        .collect();

    while heap.len() > 1 {
        let MinTree(a) = heap.pop()?;
        let MinTree(b) = heap.pop()?;
        let combined = a.freq() + b.freq();
        heap.push(MinTree(HTree::Node(Box::new(a), Box::new(b), combined)));
    }

    heap.pop().map(|MinTree(t)| t)
}
```

### Rust (functional/recursive — mirrors OCaml `go`)
```rust
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() {
        return None;
    }
    let mut trees: Vec<HTree> = freqs.iter().map(|&(c, f)| HTree::Leaf(c, f)).collect();
    trees.sort_by_key(HTree::freq);
    Some(go(trees))
}

fn go(mut trees: Vec<HTree>) -> HTree {
    match trees.len() {
        0 => panic!("go: empty — unreachable via public API"),
        1 => trees.remove(0),
        _ => {
            let a = trees.remove(0);
            let b = trees.remove(0);
            let combined = a.freq() + b.freq();
            let merged = HTree::Node(Box::new(a), Box::new(b), combined);
            trees.push(merged);
            trees.sort_by_key(HTree::freq);
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
| Code extraction | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree) -> Vec<(char, String)>` |
| Input list | `(char * int) list` | `&[(char, u32)]` (borrowed slice) |

## Key Insights

1. **Recursive types need `Box`:** OCaml's garbage collector handles recursive heap allocation transparently. Rust requires `Box<HTree>` inside `Node` because the compiler must know the size of every type at compile time — without `Box`, the enum would be infinitely sized.

2. **Min-heap via `Ord` reversal:** OCaml has no heap in its standard library, so the original code re-sorts the entire list after every merge (O(n² log n)). Rust's `BinaryHeap` is natively a max-heap; wrapping `HTree` in a `MinTree` newtype that reverses the `Ord` comparison gives us a min-heap and true O(n log n) performance.

3. **`Option` instead of exceptions:** OCaml's `failwith "empty"` raises a runtime exception for empty input. Rust encodes this as `Option<HTree>` — the caller is forced by the type system to handle the empty case, eliminating a class of runtime failures.

4. **Borrowing vs. ownership in code extraction:** OCaml's `codes prefix tree` takes the tree by reference implicitly. Rust's `codes(tree: &HTree)` is explicit — we borrow the tree for traversal, avoiding a copy of the entire structure. The `prefix: String` is passed by value and extended with `format!` to produce new owned strings for each branch.

5. **`sort_by_key` vs. comparison lambda:** OCaml uses `List.sort (fun a b -> compare (freq a) (freq b))`. Rust's `Vec::sort_by_key` accepts a key extractor (`HTree::freq` as a function pointer), which is more readable and avoids the two-argument comparison closure.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** you care about performance and are processing large symbol sets — the heap avoids re-sorting after every merge, giving O(n log n) total vs. O(n² log n) for the sort-every-time approach.

**Use recursive Rust (sort-and-merge) when:** you want the code to mirror the OCaml original as closely as possible for educational purposes, or when n is small enough that the O(n² log n) cost is irrelevant.
