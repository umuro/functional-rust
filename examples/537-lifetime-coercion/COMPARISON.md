# OCaml vs Rust: Lifetime Coercion

## OCaml
```ocaml
(* No lifetime coercion concept — GC manages all *)
let use_briefly s = String.length s

let demo () =
  let long_lived = "static string" in
  use_briefly long_lived
```

## Rust
```rust
// Longer lifetime coerces to shorter
pub fn use_briefly<'short>(s: &'short str) -> usize {
    s.len()
}

// 'static (longer) can be used where 'short expected
fn demo() {
    let static_str: &'static str = "forever";
    use_briefly(static_str);  // 'static -> 'short coercion
}

// Explicit bound: 'long outlives 'short
fn variance<'long: 'short, 'short>(r: &'long str) -> &'short str {
    r  // covariant: longer to shorter
}
```

## Key Differences

1. **OCaml**: No explicit lifetime relationships
2. **Rust**: 'long: 'short means 'long outlives 'short
3. **Rust**: Longer lifetimes coerce to shorter automatically
4. **Rust**: Enables 'static to be used anywhere
5. **Rust**: Variance rules determine valid coercions
