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

struct MinHeapNode(HTree);

impl Ord for MinHeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.freq().cmp(&self.0.freq()) // reversed for min-heap
    }
}

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut heap: BinaryHeap<MinHeapNode> = freqs
        .iter()
        .map(|&(c, f)| MinHeapNode(HTree::Leaf(c, f)))
        .collect();
    while heap.len() > 1 {
        if let (Some(a_node), Some(b_node)) = (heap.pop(), heap.pop()) {
            let (a, b) = (a_node.0, b_node.0);
            let freq = a.freq() + b.freq();
            heap.push(MinHeapNode(HTree::Node(Box::new(a), Box::new(b), freq)));
        }
    }
    heap.pop().map(|n| n.0)
}

pub fn codes(prefix: &str, tree: &HTree) -> Vec<(char, String)> {
    match tree {
        HTree::Leaf(c, _) => vec![(*c, prefix.to_string())],
        HTree::Node(l, r, _) => {
            let left = codes(&(prefix.to_string() + "0"), l);
            let right = codes(&(prefix.to_string() + "1"), r);
            left.into_iter().chain(right).collect()
        }
    }
}
```

### Rust (functional/recursive — mirrors OCaml sort-then-recurse)

```rust
pub fn build_tree_functional(freqs: &[(char, u32)]) -> Option<HTree> {
    let trees = freqs.iter().map(|&(c, f)| HTree::Leaf(c, f)).collect();
    go_sorted(trees)
}

fn go_sorted(mut trees: Vec<HTree>) -> Option<HTree> {
    trees.sort_by_key(|t| t.freq());
    match trees.len() {
        0 => None,
        1 => Some(trees.remove(0)),
        _ => {
            let a = trees.remove(0);
            let b = trees.remove(0);
            let merged_freq = a.freq() + b.freq();
            let merged = HTree::Node(Box::new(a), Box::new(b), merged_freq);
            trees.push(merged);
            go_sorted(trees)
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type htree = Leaf of char * int \| Node of htree * htree * int` | `enum HTree { Leaf(char, u32), Node(Box<HTree>, Box<HTree>, u32) }` |
| Frequency accessor | `val freq : htree -> int` | `fn freq(&self) -> u32` (method on `HTree`) |
| Build function | `val build_tree : (char * int) list -> htree` | `fn build_tree(freqs: &[(char, u32)]) -> Option<HTree>` |
| Codes function | `val codes : string -> htree -> (char * string) list` | `fn codes(prefix: &str, tree: &HTree) -> Vec<(char, String)>` |
| Optional result | `failwith "empty"` (exception) | `Option<HTree>` (no exceptions) |

## Key Insights

1. **Recursive types need indirection in Rust.** OCaml's `htree` is self-referential by design — the runtime uses pointers transparently. Rust requires `Box<HTree>` explicitly so the compiler can compute `Node`'s stack size. The heap allocation is the same; only the annotation differs.

2. **`BinaryHeap` is a max-heap; reverse the `Ord` impl for a min-heap.** The `MinHeapNode` wrapper's `cmp` delegates to `other.freq().cmp(&self.freq())` (arguments swapped) — a standard Rust idiom for priority inversion. OCaml's `List.sort` with a custom comparator does the same thing conceptually but re-sorts the entire list each merge step, giving O(n log n) vs O(log n) per step.

3. **Or-patterns collapse repeated arms.** `Leaf(_, f) | Node(_, _, f) => *f` is a direct one-liner equivalent of OCaml's two-arm `match`. Both languages added or-patterns in similar timeframes; Rust stabilised them in 1.53.

4. **Error handling philosophy.** OCaml uses `failwith "empty"` (an exception) for the impossible-but-unchecked empty input case. Rust wraps the result in `Option<HTree>`, making the empty-input case visible in the type signature and forcing callers to handle it — no equivalent of an unchecked runtime exception.

5. **String ownership in code generation.** OCaml's `codes` uses the immutable string `prefix` directly with `^` for concatenation, copying lazily. Rust's recursive `codes` creates a new `String` per recursive call (`prefix.to_string() + "0"`), then chains iterators to avoid intermediate `Vec` allocations until the final `collect()`.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** building production Huffman codecs or processing large symbol alphabets — O(log n) per merge is a meaningful speedup over O(n log n) re-sorting for alphabets larger than a few dozen symbols.

**Use functional/recursive Rust (sort-then-recurse) when:** translating OCaml directly for learning purposes, prototyping, or when the alphabet is small (< 50 symbols) and clarity matters more than throughput.
