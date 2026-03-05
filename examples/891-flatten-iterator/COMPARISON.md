# OCaml vs Rust: Flatten Iterator

## Side-by-Side Code

### OCaml
```ocaml
(* Flatten a list of lists *)
let flatten_lists = List.flatten

(* flat_map = map + flatten *)
let flat_map f lst = List.flatten (List.map f lst)

let words_in_sentences sentences =
  flat_map (String.split_on_char ' ') sentences

(* Flatten options using filter_map *)
let flatten_options lst =
  List.filter_map Fun.id lst
```

### Rust (idiomatic — iterator chains)
```rust
fn flatten_vecs(nested: Vec<Vec<i32>>) -> Vec<i32> {
    nested.into_iter().flatten().collect()
}

fn words_in_sentences(sentences: &[&str]) -> Vec<String> {
    sentences.iter()
        .flat_map(|s| s.split_whitespace())
        .map(String::from)
        .collect()
}

fn flatten_options(opts: Vec<Option<i32>>) -> Vec<i32> {
    opts.into_iter().flatten().collect()
}
```

### Rust (functional/recursive — manual via fold)
```rust
fn flatten_vecs_fold(nested: Vec<Vec<i32>>) -> Vec<i32> {
    nested.into_iter().fold(Vec::new(), |mut acc, inner| {
        acc.extend(inner);
        acc
    })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Flatten nested lists | `val flatten : 'a list list -> 'a list` | `fn flatten_vecs(nested: Vec<Vec<i32>>) -> Vec<i32>` |
| flat_map | `List.flatten (List.map f lst)` | `.flat_map(f)` directly on iterator |
| Flatten options | `List.filter_map Fun.id lst` | `.into_iter().flatten().collect()` |
| Generic constraint | `'a list list` | `Iterator<Item: IntoIterator>` |

## Key Insights

1. **`Option` and `Result` implement `IntoIterator`** — In Rust, `Some(x)` yields one item and `None` yields zero, so `.flatten()` on `Vec<Option<T>>` is equivalent to OCaml's `List.filter_map Fun.id`. This is a beautiful unification: the same combinator works on nested `Vec`s, optional values, and fallible results.

2. **`.flat_map(f)` = `.map(f).flatten()`** — Both languages share this equivalence. OCaml expresses it as `List.flatten (List.map f lst)`; Rust has a dedicated `.flat_map()` adaptor that fuses the two steps without an intermediate allocation.

3. **Ownership-aware flattening** — `Vec<Vec<T>>.into_iter().flatten()` moves each inner `Vec` and its elements. Using `.iter().flatten()` instead would yield `&T` references, avoiding moves. OCaml's GC hides this distinction entirely; Rust makes ownership explicit at the type level.

4. **Depth is explicit** — `.flatten()` removes exactly one layer of nesting. To flatten two levels you chain `.flatten().flatten()`. OCaml's `List.flatten` is likewise single-depth; `deep_flatten` requires two calls. Neither language provides automatic arbitrary-depth flattening.

5. **Zero-cost laziness** — Rust's iterator adaptors are lazy; nothing is allocated until `.collect()`. OCaml's `List.flatten` builds an intermediate list eagerly. For large datasets, the Rust version is more memory-efficient because elements flow directly into the output collection.

## When to Use Each Style

**Use `.flatten()`** when you already have an `Iterator<Item: IntoIterator>` and want to collapse one nesting level — `Vec<Vec<T>>`, `Vec<Option<T>>`, or `Vec<Result<T, E>>`.

**Use `.flat_map(f)`** when the mapping step itself produces an iterable (e.g., splitting strings into words, expanding ranges). It is strictly more composable than `.map().flatten()` and communicates intent more clearly.

**Use the `fold`/recursive style** only when you need to accumulate state during flattening, or when teaching the underlying mechanics explicitly.
