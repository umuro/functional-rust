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
#[derive(Eq, PartialEq)]
struct MinFreq(Box<HTree>);

impl Ord for MinFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed: BinaryHeap is max-heap; reversing makes it min-heap
        other.0.freq().cmp(&self.0.freq())
    }
}

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() { return None; }
    let mut heap: BinaryHeap<MinFreq> = freqs
        .iter()
        .map(|&(ch, freq)| MinFreq(Box::new(HTree::Leaf { ch, freq })))
        .collect();
    while heap.len() > 1 {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;
        let freq = a.freq() + b.freq();
        heap.push(MinFreq(Box::new(HTree::Node { left: a, right: b, freq })));
    }
    heap.pop().map(|entry| *entry.0)
}
```

### Rust (functional/sorted-Vec — mirrors OCaml)
```rust
pub fn build_tree_sorted(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() { return None; }
    let mut trees: Vec<HTree> = {
        let mut v: Vec<HTree> = freqs.iter()
            .map(|&(ch, freq)| HTree::Leaf { ch, freq })
            .collect();
        v.sort_by_key(HTree::freq);
        v
    };
    while trees.len() > 1 {
        let a = trees.remove(0);
        let b = trees.remove(0);
        let freq = a.freq() + b.freq();
        let merged = HTree::Node { left: Box::new(a), right: Box::new(b), freq };
        let pos = trees.partition_point(|t| t.freq() <= freq);
        trees.insert(pos, merged);
    }
    trees.into_iter().next()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type htree = Leaf of char * int \| Node of htree * htree * int` | `enum HTree { Leaf { ch, freq }, Node { left: Box<HTree>, right: Box<HTree>, freq } }` |
| Frequency accessor | `val freq : htree -> int` | `fn freq(&self) -> u32` |
| Build function | `val build_tree : (char * int) list -> htree` | `fn build_tree(freqs: &[(char, u32)]) -> Option<HTree>` |
| Code generation | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree) -> Vec<(char, String)>` |
| Optional result | implicit `failwith` on empty | `Option<HTree>` — explicit in the type |

## Key Insights

1. **Recursive types need indirection in Rust.** OCaml's `htree` is self-referential by default because the runtime uses uniform heap representation. Rust requires `Box<HTree>` to give the enum a known stack size — the compiler would reject a bare `HTree` field inside `HTree`.

2. **Reversed `Ord` is the standard min-heap pattern.** `std::collections::BinaryHeap` is a max-heap. Wrapping in a newtype and reversing the comparison in `Ord::cmp` is idiomatic Rust for priority queues ordered by minimum value, far cleaner than negating integers.

3. **`partition_point` replaces full re-sort.** The sorted-Vec version uses `slice::partition_point` (binary search) to find the insertion position in O(log n), then `Vec::insert` in O(n). This is more explicit about the invariant than OCaml's `List.sort`, which re-sorts the full list on every merge step.

4. **Error handling is in the type.** OCaml's `go []` raises `failwith "empty"` — a runtime exception. Rust's `build_tree` returns `Option<HTree>`, making the empty-input case explicit at the call site. No surprises.

5. **Ownership makes the algorithm explicit.** Each `heap.pop()` gives sole ownership of the subtree; passing it into `HTree::Node { left: a, right: b, freq }` transfers that ownership permanently. The borrow checker guarantees no aliasing — each node has exactly one owner in the tree, without reference counting or GC.

## When to Use Each Style

**Use idiomatic Rust (`BinaryHeap`) when:** building production-grade Huffman coding for real data; the O(n log n) complexity matters for large alphabets or streaming compression.

**Use functional/sorted-Vec when:** teaching the algorithm or porting OCaml code; the structure matches the mathematical description step-for-step, making it easier to verify correctness against a reference implementation.
