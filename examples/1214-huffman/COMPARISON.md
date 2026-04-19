# OCaml vs Rust: Huffman Coding

## Side-by-Side Code

### OCaml

```ocaml
type htree = Leaf of char * int | Node of htree * htree * int

let freq = function Leaf (_,f) -> f | Node (_,_,f) -> f

let build_tree freqs =
  let trees =
    List.map (fun (c,f) -> Leaf (c,f)) freqs
    |> List.sort (fun a b -> compare (freq a) (freq b))
  in
  let rec go = function
    | [t] -> t
    | a :: b :: rest ->
        let merged = Node (a, b, freq a + freq b) in
        go (List.sort (fun a b -> compare (freq a) (freq b)) (merged :: rest))
    | [] -> failwith "empty"
  in go trees

let rec codes prefix = function
  | Leaf (c, _) -> [(c, prefix)]
  | Node (l, r, _) -> codes (prefix ^ "0") l @ codes (prefix ^ "1") r
```

### Rust (idiomatic — min-heap)

```rust
pub fn build_tree_heap(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() { return None; }
    let mut heap: BinaryHeap<HeapEntry> = freqs.iter().enumerate()
        .map(|(i, &(c, f))| HeapEntry { freq: f, tiebreak: i, tree: HTree::Leaf(c, f) })
        .collect();
    let mut tiebreak = freqs.len();
    while heap.len() > 1 {
        let a = heap.pop()?;
        let b = heap.pop()?;
        let total = a.freq + b.freq;
        let merged = HTree::Node(Box::new(a.tree), Box::new(b.tree), total);
        heap.push(HeapEntry { freq: total, tiebreak, tree: merged });
        tiebreak += 1;
    }
    heap.pop().map(|e| e.tree)
}
```

### Rust (functional — sort/merge, OCaml-parallel)

```rust
pub fn build_tree_sorted(freqs: &[(char, u32)]) -> Option<HTree> {
    if freqs.is_empty() { return None; }
    let mut trees: Vec<HTree> =
        freqs.iter().map(|&(c,f)| HTree::Leaf(c,f)).collect();
    trees.sort_by_key(HTree::freq);
    while trees.len() > 1 {
        let a = trees.remove(0);
        let b = trees.remove(0);
        let total = a.freq() + b.freq();
        let merged = HTree::Node(Box::new(a), Box::new(b), total);
        let pos = trees.iter().position(|t| t.freq() > total).unwrap_or(trees.len());
        trees.insert(pos, merged);
    }
    trees.into_iter().next()
}
```

## Type Signatures

| Concept            | OCaml                              | Rust                                         |
|--------------------|------------------------------------|----------------------------------------------|
| Tree type          | `type htree = Leaf .. | Node ..`   | `pub enum HTree { Leaf(..), Node(..) }`      |
| Recursive child    | `htree` (direct)                   | `Box<HTree>` (sized requirement)             |
| Build signature    | `(char * int) list -> htree`       | `fn build_tree(&[(char,u32)]) -> Option<HTree>` |
| Codes signature    | `string -> htree -> (char*string) list` | `fn codes(&HTree) -> Vec<(char,String)>` |
| Failure on empty   | `failwith "empty"`                 | `Option::None`                               |

## Key Insights

1. **Boxing is mandatory for recursive enums in Rust.**  Without `Box`, the
   compiler can't compute a fixed size for `HTree` and rejects the type.
2. **`BinaryHeap` is max-heap; wrap to reverse.**  We implement `Ord` on a
   `HeapEntry` wrapper so flipping `cmp` order is local to the PQ use-site
   rather than polluting `HTree` with an orientation-specific `Ord` impl.
3. **No free structural ordering on trees.**  OCaml's `compare` sorts any two
   values; Rust refuses to derive `Ord` for `HTree` because `Box<HTree>`
   doesn't have a meaningful comparison beyond "compare the frequencies."
4. **String accumulation differs.**  OCaml's immutable `prefix ^ "0"` creates a
   fresh string per step; the Rust DFS pushes a single byte onto a shared
   `String` buffer and pops it on the way up — linear total work.
5. **Determinism needs a tiebreak.**  When two frequencies are equal, OCaml's
   stable sort preserves insertion order; Rust's `BinaryHeap` is unstable, so
   a monotonically-increasing `tiebreak` field is added to recover
   reproducible code assignments across runs.

## When to Use Each Style

**Use idiomatic Rust (`build_tree_heap`) when:** encoding real data and you care
about throughput — it's O(n log n) and streams input into the heap naturally.

**Use recursive Rust (`build_tree_sorted`) when:** teaching the algorithm or
comparing against the OCaml reference — the sort/merge loop reads the same
way and makes the invariant ("smallest two merge next") blindingly obvious.
