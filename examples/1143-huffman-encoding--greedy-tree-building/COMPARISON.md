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
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
pub enum HTree {
    Leaf(char, u32),
    Node(Box<HTree>, Box<HTree>, u32),
}

impl HTree {
    pub fn freq(&self) -> u32 {
        match self {
            HTree::Leaf(_, f) | HTree::Node(_, _, f) => *f,
        }
    }
}

struct MinTree(HTree);
// Ord impl wraps freq in Reverse so lowest frequency is popped first.

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut heap: BinaryHeap<MinTree> = freqs
        .iter()
        .map(|&(c, f)| MinTree(HTree::Leaf(c, f)))
        .collect();
    while heap.len() > 1 {
        let MinTree(a) = heap.pop().unwrap();
        let MinTree(b) = heap.pop().unwrap();
        let combined = a.freq() + b.freq();
        heap.push(MinTree(HTree::Node(Box::new(a), Box::new(b), combined)));
    }
    heap.pop().map(|MinTree(t)| t)
}
```

### Rust (functional/recursive — mirrors OCaml `go`)

```rust
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut trees: Vec<HTree> = freqs
        .iter()
        .map(|&(c, f)| HTree::Leaf(c, f))
        .collect();
    trees.sort_by_key(HTree::freq);
    go(trees)
}

fn go(mut trees: Vec<HTree>) -> Option<HTree> {
    match trees.len() {
        0 => None,
        1 => trees.into_iter().next(),
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
| Frequency accessor | `let freq t = match t with ...` | `fn freq(&self) -> u32` (method) |
| Build function | `val build_tree : (char * int) list -> htree` | `fn build_tree(freqs: &[(char, u32)]) -> Option<HTree>` |
| Code generation | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree) -> Vec<(char, String)>` |
| Empty input | `failwith "empty"` (exception) | `None` (Option) |

## Key Insights

1. **Recursive heap types need `Box`:** OCaml heap-allocates all values transparently. Rust requires explicit `Box<HTree>` in the `Node` variant to break the infinite-size cycle — without it, `HTree` would have no known size at compile time.

2. **`BinaryHeap` replaces repeated sorting:** OCaml's `go` calls `List.sort` on every iteration, which is O(n log n) per step for a total of O(n² log n). Rust's `BinaryHeap` with a `Reverse`-based `Ord` impl delivers O(log n) per merge for O(n log n) total — the standard efficient Huffman implementation.

3. **Custom ordering via newtype + `Ord`:** OCaml passes a comparison closure to `List.sort`. Rust's `BinaryHeap` requires the element type to implement `Ord`. A newtype wrapper `MinTree(HTree)` provides a clean `Ord` impl without polluting `HTree` itself with priority-queue semantics.

4. **Error handling style:** OCaml's `failwith "empty"` raises an exception on empty input. Rust returns `Option<HTree>`, propagating `None` for the empty case — explicit, composable, no runtime panics.

5. **String accumulation:** OCaml's `prefix ^ "0"` copies the string on each recursive step. Rust's `format!("{prefix}0")` does the same, allocating a new `String`. Both are O(depth) per leaf, proportional to the code length — no significant difference.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** building real Huffman compressors where performance matters; the heap approach is O(n log n) and the standard choice in production code.

**Use recursive Rust (sort-based) when:** demonstrating the algorithm's greedy structure pedagogically, or translating OCaml code directly for comparison purposes; the sort-per-step approach makes the "always pick the two smallest" invariant visually obvious at the cost of efficiency.
