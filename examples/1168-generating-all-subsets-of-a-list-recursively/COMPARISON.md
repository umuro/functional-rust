# OCaml vs Rust: Generating All Subsets of a List Recursively

## Side-by-Side Code

### OCaml
```ocaml
let rec powerset = function
  | [] -> [[]]
  | x :: rest ->
    let ps = powerset rest in
    ps @ List.map (fun s -> x :: s) ps
```

### Rust (idiomatic)
```rust
pub fn subsets<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    match items.split_first() {
        None => vec![vec![]],
        Some((first, rest)) => {
            let without = subsets(rest);
            let with_first: Vec<Vec<T>> = without.iter().map(|s| {
                let mut new_s = vec![first.clone()];
                new_s.extend_from_slice(s);
                new_s
            }).collect();
            let mut result = without;
            result.extend(with_first);
            result
        }
    }
}
```

### Rust (functional/recursive — mirror of OCaml structure)
```rust
pub fn subsets_recursive<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() { return vec![vec![]]; }
    let first = &items[0];
    let ps = subsets_recursive(&items[1..]);
    let with_first: Vec<Vec<T>> = ps.iter().map(|s| {
        let mut sub = vec![first.clone()];
        sub.extend_from_slice(s);
        sub
    }).collect();
    let mut result = ps;
    result.extend(with_first);
    result
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| powerset | `'a list -> 'a list list` | `fn subsets<T: Clone>(items: &[T]) -> Vec<Vec<T>>` |
| element copy | `x :: s` — O(1) cons (persistent list) | `first.clone()` — O(1) clone for `Copy` types, O(k) for others |
| append | `ps @ (List.map ...)` | `result.extend(with_first)` |
| empty case | `\| [] -> [[]]` | `None => vec![vec![]]` |
| decomposition | `\| x :: rest ->` | `Some((first, rest)) = items.split_first()` |

## Key Insights

1. **Clone cost**: OCaml's persistent lists share tails — `x :: s` is O(1) because it just prepends one node. Rust must `clone()` each element to build new `Vec`s, making the operation O(subset_size) per subset.
2. **Pattern matching on lists**: OCaml's `| [] -> ... | x :: rest ->` is the canonical idiom for list decomposition. Rust mirrors this with `split_first()` which returns `Option<(&T, &[T])>`.
3. **Output size**: Both produce 2ⁿ subsets for an n-element input. The exponential output size is inherent to the problem — both implementations are optimal.
4. **Argument borrowing**: Rust takes `&[T]` (a borrowed slice); items are not consumed. The `T: Clone` bound is required to copy elements into new subset Vecs. OCaml's polymorphic `'a list` handles this without an explicit bound.
5. **Accumulator order**: OCaml `ps @ (map ...)` appends "with-first" subsets after "without-first"; both Rust versions do the same. This means the empty set appears first and the full set appears last.

## When to Use Each Style

**Use `split_first()` when:** writing idiomatic Rust that handles slices — it expresses the head/tail decomposition cleanly.
**Use `items[0]` / `&items[1..]` when:** you prefer a more direct translation of the OCaml pattern that's immediately recognizable to OCaml readers.
