# OCaml vs Rust: Index-Value Pairs with enumerate()

## Side-by-Side Code

### OCaml
```ocaml
let fruits = ["apple"; "banana"; "cherry"] in

(* Loop with index *)
List.iteri (fun i fruit ->
  Printf.printf "%d: %s\n" i fruit
) fruits;

(* Filter by index *)
let evens_only = List.filteri (fun i _ -> i mod 2 = 0) fruits in

(* Map with index *)
let numbered = List.mapi (fun i name ->
  Printf.sprintf "#%d %s" (i + 1) name
) fruits
```

### Rust (idiomatic)
```rust
let fruits = ["apple", "banana", "cherry"];

// Loop with index
for (i, fruit) in fruits.iter().enumerate() {
    println!("{}: {}", i, fruit);
}

// Filter by index — stays in the lazy pipeline
let evens: Vec<_> = fruits.iter()
    .enumerate()
    .filter(|(i, _)| i % 2 == 0)
    .map(|(_, v)| *v)
    .collect();

// Map with index
let numbered: Vec<String> = fruits.iter()
    .enumerate()
    .map(|(i, f)| format!("{}. {}", i + 1, f))
    .collect();
```

### Rust (functional — find with index)
```rust
// Find the index of the first element matching a predicate
fn find_index<T, F: Fn(&T) -> bool>(items: &[T], pred: F) -> Option<usize> {
    items
        .iter()
        .enumerate()
        .find(|(_, v)| pred(v))
        .map(|(i, _)| i)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Indexed iteration | `List.iteri : (int -> 'a -> unit) -> 'a list -> unit` | `Iterator::enumerate() -> impl Iterator<Item=(usize, T)>` |
| Filter by index | `List.filteri : (int -> 'a -> bool) -> 'a list -> 'a list` | `.enumerate().filter(|(i, _)| ...).map(|(_, v)| v)` |
| Map with index | `List.mapi : (int -> 'a -> 'b) -> 'a list -> 'b list` | `.enumerate().map(|(i, v)| ...)` |
| Find by predicate | `List.find_opt : ('a -> bool) -> 'a list -> 'a option` | `.find(|(_, v)| pred(v))` → `Option<(usize, &T)>` |

## Key Insights

1. **Single adapter vs. separate functions:** OCaml provides `List.iteri`, `List.filteri`, and `List.mapi` as distinct functions for each indexed operation. Rust exposes a single `enumerate()` adapter that composes uniformly with the rest of the iterator chain — you don't need a separate API for every combination.

2. **Lazy vs. eager:** OCaml's `List.mapi` produces a new list immediately. Rust's `.enumerate().map(...)` is lazy; nothing is computed until `.collect()` or a consuming adapter is called. This lets you chain more transformations before paying allocation cost.

3. **Index type:** OCaml uses `int` (which is the platform-native signed integer) for indices. Rust uses `usize` — the unsigned, pointer-sized integer — which is the natural choice for collection indices and prevents negative-index bugs at the type level.

4. **Tuple destructuring in patterns:** Both languages destructure pairs naturally. OCaml uses `fun i x -> ...`; Rust uses `|(i, x)| ...` in closure patterns. Rust additionally supports `|(i, _)| ...` to ignore components, matching OCaml's `fun _ x -> ...`.

5. **No mutable counter needed:** The classic imperative pattern (`let mut i = 0; ... i += 1;`) is completely replaced. In Rust, `.enumerate()` works on *any* iterator — not just slices — so you get index-awareness even when iterating files, channels, or custom iterators.

## When to Use Each Style

**Use `enumerate()` in a chain when:** you need the index alongside filtering or mapping — staying lazy avoids an intermediate `collect()` and keeps the pipeline readable.

**Use `enumerate()` in a `for` loop when:** you only need to iterate with side effects (printing, mutating external state) and don't need to produce a new collection.
