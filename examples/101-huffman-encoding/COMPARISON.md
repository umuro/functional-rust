# OCaml vs Rust: Huffman Encoding

## Side-by-Side Code

### OCaml
```ocaml
type htree = Leaf of char * int | Node of htree * htree * int

let freq = function Leaf (_,f) -> f | Node (_,_,f) -> f

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

pub enum HTree {
    Leaf(char, u32),
    Node(Box<HTree>, Box<HTree>, u32),
}

// Newtype that orders by frequency only, enabling min-heap via Reverse<FreqOrd>
struct FreqOrd(HTree);
impl Ord for FreqOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.freq().cmp(&other.0.freq())
    }
}

pub fn build_tree(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut heap: BinaryHeap<Reverse<FreqOrd>> = freqs
        .iter()
        .map(|&(c, f)| Reverse(FreqOrd(HTree::Leaf(c, f))))
        .collect();
    while heap.len() > 1 {
        let Reverse(FreqOrd(a)) = heap.pop()?;
        let Reverse(FreqOrd(b)) = heap.pop()?;
        let freq = a.freq() + b.freq();
        heap.push(Reverse(FreqOrd(HTree::Node(Box::new(a), Box::new(b), freq))));
    }
    heap.pop().map(|Reverse(FreqOrd(t))| t)
}
```

### Rust (functional/recursive — mirrors OCaml list-sort)
```rust
pub fn build_tree_recursive(freqs: &[(char, u32)]) -> Option<HTree> {
    let mut trees: Vec<HTree> = freqs.iter()
        .map(|&(c, f)| HTree::Leaf(c, f))
        .collect();
    trees.sort_by_key(|t| t.freq());

    fn go(mut trees: Vec<HTree>) -> Option<HTree> {
        match trees.len() {
            0 => None,
            1 => trees.into_iter().next(),
            _ => {
                let a = trees.remove(0);
                let b = trees.remove(0);
                let freq = a.freq() + b.freq();
                let merged = HTree::Node(Box::new(a), Box::new(b), freq);
                let mut next: Vec<HTree> = std::iter::once(merged).chain(trees).collect();
                next.sort_by_key(|t| t.freq());
                go(next)
            }
        }
    }
    go(trees)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type htree = Leaf of char * int \| Node of htree * htree * int` | `enum HTree { Leaf(char, u32), Node(Box<HTree>, Box<HTree>, u32) }` |
| Frequency accessor | `let freq = function Leaf (_,f) -> f \| Node (_,_,f) -> f` | `fn freq(&self) -> u32 { match self { Leaf(_, f) \| Node(_, _, f) => *f } }` |
| Build function | `val build_tree : (char * int) list -> htree` | `fn build_tree(freqs: &[(char, u32)]) -> Option<HTree>` |
| Code generator | `val codes : string -> htree -> (char * string) list` | `fn codes(tree: &HTree) -> Vec<(char, String)>` |
| Priority queue | `List.sort` (re-sort each step) | `BinaryHeap<Reverse<FreqOrd>>` |

## Key Insights

1. **Box for recursive types:** OCaml's `htree` is natively recursive because
   the runtime uses heap-allocated values uniformly.  Rust's enum must know its
   size at compile time, so child nodes must be `Box<HTree>`.  This is not
   boilerplate — it is an explicit ownership declaration.

2. **Min-heap without a crate:** OCaml has no built-in priority queue so the
   idiomatic choice is `List.sort`.  Rust's `std::collections::BinaryHeap` is a
   max-heap; wrapping values in `Reverse<T>` flips the ordering to give an
   efficient O(log n) min-heap, replacing the O(n log n) re-sort in the
   recursive version.

3. **Trait-based ordering vs. inline comparator:** OCaml threads a comparison
   function `(fun a b -> compare (freq a) (freq b))` directly into `List.sort`.
   Rust encodes ordering in the type system via `impl Ord for FreqOrd`, keeping
   comparison logic declared once and reused implicitly wherever the type is
   ordered.  The `FreqOrd` newtype isolates this concern so `HTree` itself
   carries no ordering assumption.

4. **Owned strings vs. string slices:** OCaml's `^` operator copies both
   operands into a new string on every recursive call — O(depth) allocation per
   leaf.  The Rust version passes an owned `String` down the call stack with
   `format!("{prefix}0")`, which is the same cost but makes the allocation
   explicit and borrow-checker-safe without needing `&str` lifetime annotations.

5. **Pattern matching on slice vs. list:** The OCaml `go` function matches
   `| a :: b :: rest` on a list head.  Rust uses `trees.remove(0)` on a `Vec`,
   which is O(n) but equivalent.  A slice-pattern version `[a, b, rest @ ..]`
   is possible with `Box<[HTree]>` but adds complexity for no algorithmic gain.

## When to Use Each Style

**Use idiomatic Rust (BinaryHeap) when:** building or processing large priority
queues where O(log n) insert/remove matters — real compression pipelines,
scheduler implementations, or any greedy algorithm over many elements.

**Use recursive Rust (Vec + sort) when:** the data set is small (< 256 symbols
for standard Huffman), the directness of the OCaml translation aids
comprehension, or you are porting OCaml code and want a mechanical first-pass
before optimising.
