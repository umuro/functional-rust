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
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct MinFreq(HTree);

impl Ord for MinFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.freq().cmp(&self.0.freq()) // reversed = min-heap
    }
}

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() { return None; }
    let mut heap: BinaryHeap<MinFreq> = freqs
        .iter()
        .map(|&(ch, freq)| MinFreq(HTree::Leaf { ch, freq }))
        .collect();
    while heap.len() > 1 {
        let a = heap.pop()?.0;
        let b = heap.pop()?.0;
        let freq = a.freq() + b.freq();
        heap.push(MinFreq(HTree::Node { left: Box::new(a), right: Box::new(b), freq }));
    }
    heap.pop().map(|e| e.0)
}
```

### Rust (functional/recursive — mirrors OCaml directly)

```rust
pub fn build_tree_functional(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() { return None; }
    let mut trees: Vec<HTree> = freqs
        .iter()
        .map(|&(ch, freq)| HTree::Leaf { ch, freq })
        .collect();
    trees.sort_by_key(HTree::freq);
    Some(go(trees))
}

fn go(mut trees: Vec<HTree>) -> HTree {
    match trees.len() {
        0 => panic!("empty"),
        1 => trees.remove(0),
        _ => {
            let a = trees.remove(0);
            let b = trees.remove(0);
            let freq = a.freq() + b.freq();
            let merged = HTree::Node { left: Box::new(a), right: Box::new(b), freq };
            trees.push(merged);
            trees.sort_by_key(HTree::freq);
            go(trees)
        }
    }
}

pub fn codes(tree: &HTree, prefix: &str) -> Vec<(char, String)> {
    match tree {
        HTree::Leaf { ch, .. } => vec![(*ch, prefix.to_string())],
        HTree::Node { left, right, .. } => {
            let mut result = codes(left, &format!("{prefix}0"));
            result.extend(codes(right, &format!("{prefix}1")));
            result
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type htree = Leaf of char * int \| Node of htree * htree * int` | `enum HTree { Leaf { ch: char, freq: u32 }, Node { left: Box<HTree>, right: Box<HTree>, freq: u32 } }` |
| Frequency extractor | `val freq : htree -> int` | `fn freq(&self) -> u32` |
| Build function | `val build_tree : (char * int) list -> htree` | `fn build_tree(freqs: &[(char, u32)]) -> Option<HTree>` |
| Code generator | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree, prefix: &str) -> Vec<(char, String)>` |
| Recursion | Tail-recursive via list pattern match | Ownership-consuming Vec; `Box<HTree>` for tree recursion |

## Key Insights

1. **Algebraic data types translate directly.** OCaml's `type htree = Leaf ... | Node ...` becomes Rust's `enum HTree`. Both use structural pattern matching. The only syntactic difference is struct variants (`{ ch, freq }`) vs tuple variants (`char * int`) — Rust struct variants make field names explicit and avoid positional mistakes.

2. **`Box<T>` makes recursive enums possible.** OCaml allocates heap memory transparently; Rust requires explicit `Box<HTree>` in `Node` to give the compiler a known size for the enum. This is a mechanical translation step, not a semantic change.

3. **`BinaryHeap` vs sorted list.** OCaml's `List.sort` after each merge is O(n log n) per step, making the overall algorithm O(n² log n). Rust's `BinaryHeap` with reversed `Ord` achieves O(log n) per insertion/removal, giving O(n log n) total. The newtype wrapper `MinFreq` with reversed comparison is the idiomatic Rust pattern for min-heap semantics.

4. **`Option<HTree>` vs `failwith`.** OCaml raises an exception on empty input (`failwith "empty"`). Rust returns `None` from the public API, making the error case explicit in the type system. Internal helpers (`go`) can still panic since they're only called after a non-empty guard.

5. **String prefix accumulation.** OCaml concatenates with `prefix ^ "0"` creating a new string per step. Rust's `format!("{prefix}0")` does the same. Both approaches are O(depth) per code, which is fine for Huffman trees since depth is O(log n) to O(n).

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** performance matters — for large alphabets (e.g., Unicode, byte-level compression) the O(n log n) heap approach significantly outperforms re-sorting a Vec after each merge.

**Use recursive Rust (Vec + sort) when:** clarity and direct OCaml correspondence are the priority, the alphabet is small, or you are porting OCaml code and want to verify correctness before optimizing.
