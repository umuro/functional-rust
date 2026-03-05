# OCaml vs Rust: Closure Lifetimes

## OCaml
```ocaml
(* Closures capture freely — GC manages *)
let make_prefixer prefix =
  fun s -> prefix ^ s

let prefixer = make_prefixer "Hello, "
(* prefix kept alive by closure *)
```

## Rust
```rust
// Closure lifetime bounded by captured reference
pub fn make_prefixer<'a>(prefix: &'a str) -> impl Fn(&str) -> String + 'a {
    move |s| format!("{}{}", prefix, s)
}

// + 'a bounds closure lifetime to prefix lifetime
// Closure invalid after prefix dropped
```

## Key Differences

1. **OCaml**: GC keeps captured values alive
2. **Rust**: + 'a on impl bounds closure lifetime
3. **Rust**: Closure can't outlive its captures
4. **Rust**: Copy types avoid lifetime issues
5. Both: Closures capture environment
