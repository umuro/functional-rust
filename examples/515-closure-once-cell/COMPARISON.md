# OCaml vs Rust: Lazy Evaluation

## OCaml
```ocaml
(* Built-in lazy keyword *)
let expensive = lazy (List.fold_left (+) 0 (List.init 1000 Fun.id))

let value = Lazy.force expensive  (* computed on first force *)
let value2 = Lazy.force expensive (* cached *)
```

## Rust
```rust
use std::sync::OnceLock;

static EXPENSIVE: OnceLock<i64> = OnceLock::new();

fn get_value() -> i64 {
    *EXPENSIVE.get_or_init(|| (1..=1000i64).sum())
}
```

## Key Differences

1. **OCaml**: Built-in `lazy` keyword and `Lazy.force`
2. **Rust**: Uses OnceLock (thread-safe) or OnceCell (single-thread)
3. **OCaml**: Lazy values are first-class with explicit forcing
4. **Rust**: Closures passed to get_or_init for deferred computation
5. Both ensure computation happens at most once
