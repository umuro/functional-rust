# Comparison: Unfold — OCaml vs Rust

## Core Insight

`unfold` is the dual of `fold`: fold reduces a collection to a value; unfold builds a collection from a seed. OCaml's version is naturally recursive and builds a list eagerly. Rust offers both eager (`Vec`) and lazy (`Iterator`) variants, with the lazy version being more idiomatic for potentially large sequences.

## OCaml

```ocaml
let rec unfold f seed = match f seed with
  | None -> []
  | Some (value, next_seed) -> value :: unfold f next_seed
```

## Rust — Eager

```rust
pub fn unfold<S, T>(seed: S, f: impl Fn(S) -> Option<(T, S)>) -> Vec<T> {
    let mut result = Vec::new();
    let mut state = seed;
    loop {
        match f(state) { None => break, Some((v, next)) => { result.push(v); state = next; } }
    }
    result
}
```

## Rust — Lazy

```rust
pub fn unfold_iter<S, T>(seed: S, f: impl Fn(&S) -> Option<(T, S)>) -> impl Iterator<Item = T> {
    let mut state = Some(seed);
    std::iter::from_fn(move || { let s = state.take()?; let (v, next) = f(&s)?; state = Some(next); Some(v) })
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Result type | `'a list` (eager) | `Vec<T>` or `impl Iterator` (lazy) |
| Recursion | Natural recursive cons | Loop or `from_fn` |
| Termination | `None` stops recursion | `None` breaks loop / ends iterator |
| Infinite seqs | Stack overflow risk | Lazy iterator handles infinite |
| Ownership | GC manages seed | `Fn(S) -> ...` takes ownership |

## Learner Notes

- **Dual of fold**: If fold is `[a,b,c] -> x`, unfold is `x -> [a,b,c]` — same function, opposite direction
- **`std::iter::from_fn`**: Rust's most flexible iterator builder — a closure that returns `Option<T>`
- **`successors`**: Simpler than `from_fn` when each value depends only on the previous one
- **Eager vs lazy**: OCaml's unfold builds the whole list; Rust's lazy version computes on demand
- **Ownership of seed**: Rust's `Fn(S) -> Option<(T, S)>` consumes the seed each step — ensures single owner
