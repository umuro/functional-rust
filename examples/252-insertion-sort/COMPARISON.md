# OCaml vs Rust: Insertion Sort

## Side-by-Side Code

### OCaml

```ocaml
let rec insert x = function
  | [] -> [x]
  | h :: t as l ->
    if x <= h then x :: l
    else h :: insert x t

let insertion_sort lst =
  List.fold_left (fun acc x -> insert x acc) [] lst
```

### Rust (idiomatic — in-place)

```rust
pub fn insertion_sort_inplace<T: Ord>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);
            j -= 1;
        }
    }
}
```

### Rust (functional — fold + partition_point)

```rust
pub fn insertion_sort_functional<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    list.iter().cloned().fold(Vec::new(), |mut acc, x| {
        let pos = acc.partition_point(|h| h < &x);
        acc.insert(pos, x);
        acc
    })
}
```

### Rust (recursive — mirrors OCaml's `insert`)

```rust
pub fn insert_rec<T: Ord + Clone>(x: T, list: &[T]) -> Vec<T> {
    match list {
        [] => vec![x],
        [h, rest @ ..] => {
            if x <= *h {
                let mut result = vec![x];
                result.extend_from_slice(list);
                result
            } else {
                let mut result = vec![h.clone()];
                result.extend(insert_rec(x, rest));
                result
            }
        }
    }
}

pub fn insertion_sort_recursive<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    list.iter()
        .cloned()
        .fold(Vec::new(), |acc, x| insert_rec(x, &acc))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Insert function | `val insert : 'a -> 'a list -> 'a list` | `fn insert_rec<T: Ord + Clone>(x: T, list: &[T]) -> Vec<T>` |
| Sort (functional) | `val insertion_sort : 'a list -> 'a list` | `fn insertion_sort_functional<T: Ord + Clone>(list: &[T]) -> Vec<T>` |
| Sort (in-place) | *(not idiomatic in OCaml)* | `fn insertion_sort_inplace<T: Ord>(data: &mut [T])` |
| List/slice type | `'a list` | `&[T]` (borrow) / `Vec<T>` (owned) |
| Ordering constraint | `(compare : 'a -> 'a -> int)` implicit | `T: Ord` explicit trait bound |

## Key Insights

1. **`fold_left` is `Iterator::fold`:** OCaml's `List.fold_left f init lst` is exactly `.fold(init, f)` on a Rust iterator. The pattern transfers verbatim — only the argument order inside the closure differs slightly.

2. **Slice patterns replace list patterns:** OCaml's `| h :: t as l ->` becomes `[h, rest @ ..]` in Rust. Rust requires the full list to be a slice (`&[T]`), not a linked list, but the syntactic parallel is close enough to read as a direct translation.

3. **`partition_point` replaces linear scan:** OCaml's `insert` walks the list element by element. Rust's sorted `Vec` supports `partition_point` (binary search on a predicate), reducing comparisons from O(n) to O(log n). The overall sort is still O(n²) due to `Vec::insert` shifting, but search is faster.

4. **Stability from the comparison guard:** Both versions are stable because equal elements are inserted *before* existing equals. OCaml's `x <= h` keeps the new `x` to the left; Rust's `partition_point(|h| h < &x)` finds the position after all `h < x`, so ties also land before existing equal elements.

5. **Allocation model — functional vs. in-place:** OCaml lists are singly-linked and immutable, so every `x :: l` and `h :: insert x t` allocates a new cons cell — allocation is unavoidable. Rust's in-place version allocates a single `Vec` upfront and mutates it, making it O(1) space overhead. The functional Rust version mimics OCaml and allocates per call.

## When to Use Each Style

**Use idiomatic in-place (`insertion_sort_inplace`) when:** you own the data, care about performance, and want zero extra allocation. Rust's ownership model makes in-place mutation safe and idiomatic.

**Use functional fold (`insertion_sort_functional`) when:** you want a pure function that returns a new sorted collection without modifying the input — matching OCaml's immutable style or when the caller must retain the original slice.

**Use recursive (`insertion_sort_recursive`) when:** teaching or demonstrating the direct OCaml→Rust translation, or when the recursive structure itself is the subject of study. Avoid for production use on large inputs due to stack depth and repeated allocations.
