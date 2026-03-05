# OCaml vs Rust: Closure Lifetime Capture

## OCaml
```ocaml
(* GC handles memory — no explicit lifetimes *)
let make_prefix_checker prefix =
  fun s -> String.starts_with ~prefix s

let checker = make_prefix_checker "hello"
(* prefix kept alive by GC as long as checker exists *)
```

## Rust
```rust
// Explicit lifetime ties closure to borrowed data
pub fn make_prefix_checker<'a>(prefix: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |s| s.starts_with(prefix)
}

// Closure invalid if prefix goes out of scope
```

## Key Differences

1. **OCaml**: GC keeps captured values alive automatically
2. **Rust**: Lifetime annotations express borrow duration
3. **Rust**: Compiler enforces closure doesn't outlive borrows
4. **Rust**: + 'a on return type bounds closure lifetime
5. Both: Closures capture environment, different memory models
