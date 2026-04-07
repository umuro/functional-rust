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
use std::cmp::Ordering;
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

// Newtype wrapper that reverses Ord for a min-heap.
struct MinTree(HTree);
impl Ord for MinTree {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.freq().cmp(&self.0.freq()) // reversed
    }
}

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

### Rust (functional/recursive — mirrors OCaml's List.sort + go)

```rust
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() { return None; }
    let mut trees: Vec<HTree> = freqs.iter()
        .map(|&(c, f)| HTree::Leaf(c, f))
        .collect();
    trees.sort_by_key(HTree::freq);
    Some(go(trees))
}

fn go(mut trees: Vec<HTree>) -> HTree {
    match trees.len() {
        0 => panic!("go: empty list"),
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

pub fn codes(tree: &HTree) -> Vec<(char, String)> {
    codes_acc(tree, String::new())
}

fn codes_acc(tree: &HTree, prefix: String) -> Vec<(char, String)> {
    match tree {
        HTree::Leaf(c, _) => vec![(*c, prefix)],
        HTree::Node(left, right, _) => {
            let mut left_codes = codes_acc(left, format!("{prefix}0"));
            left_codes.extend(codes_acc(right, format!("{prefix}1")));
            left_codes
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type htree = Leaf of char * int \| Node of htree * htree * int` | `enum HTree { Leaf(char, u32), Node(Box<HTree>, Box<HTree>, u32) }` |
| Frequency accessor | `val freq : htree -> int` | `fn freq(&self) -> u32` |
| Build tree | `val build_tree : (char * int) list -> htree` | `fn build_tree(freqs: &[(char, u32)]) -> Option<HTree>` |
| Code extraction | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree) -> Vec<(char, String)>` |
| Priority queue | `List.sort` at each step | `BinaryHeap<MinTree>` |

## Key Insights

1. **Recursive types need explicit boxing.** In OCaml, `Node of htree * htree * int` compiles fine because OCaml heap-allocates all values. In Rust, `Node(HTree, HTree, u32)` would be an infinitely-sized type; `Box<HTree>` makes the size finite by indirecting through a pointer.

2. **Min-heap via reversed `Ord`.** Rust's `BinaryHeap` is a max-heap. To get min-heap behavior, the newtype `MinTree` flips the comparison: `other.freq().cmp(&self.freq())`. This is the standard Rust pattern — cleaner than OCaml's lambda `(fun a b -> compare (freq a) (freq b))`.

3. **`|` pattern merging works in both languages.** OCaml: `match t with Leaf(_,f) -> f | Node(_,_,f) -> f`. Rust: `match self { Leaf(_, f) | Node(_, _, f) => *f }`. The syntactic parallel is striking — Rust deliberately adopted this from ML languages.

4. **Ownership enables zero-copy merging.** `trees.remove(0)` transfers ownership of the `HTree` value out of the `Vec`. We can then move `a` and `b` directly into the new `Node` without any `clone()`. OCaml has no concept of ownership — it just GC-manages everything.

5. **`Option` vs `failwith`.** OCaml's `go` calls `failwith "empty"` for the impossible empty case. Rust's public API returns `Option<HTree>` and documents the invariant — `panic!` is reserved for the internal `go` function where the invariant is maintained by construction.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** building Huffman trees on large alphabets, or in performance-sensitive code — O(n log n) vs O(n²) for the sort-each-time approach.

**Use recursive Rust (sort-and-merge) when:** translating OCaml/Haskell directly for pedagogical purposes, or when the input is small and clarity of the algorithm matters more than performance.
