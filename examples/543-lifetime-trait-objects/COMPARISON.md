# OCaml vs Rust: Trait Object Lifetimes

## OCaml
```ocaml
(* Objects don't have explicit lifetimes *)
class type renderer = object
  method render : string
end

let store (r : renderer) = r
```

## Rust
```rust
// Default: Box<dyn Trait> = Box<dyn Trait + 'static>
pub fn store(r: Box<dyn Renderer>) -> Box<dyn Renderer> { r }

// With borrowed data: explicit lifetime
pub fn use_borrowed<'a>(r: &'a dyn Renderer) -> String {
    r.render()
}

// Struct with borrowed field needs lifetime on dyn
struct Container<'a> {
    renderer: Box<dyn Renderer + 'a>,
}
```

## Key Differences

1. **OCaml**: Objects don't track borrowed references
2. **Rust**: dyn Trait has implicit or explicit lifetime
3. **Rust**: 'static default for owned trait objects
4. **Rust**: Explicit 'a needed for borrowed data
5. Both: Runtime polymorphism via indirection
