# Comparison: Church Numerals — OCaml vs Rust

## Core Insight

Church numerals reveal a fundamental difference: OCaml's type system embraces higher-rank polymorphism naturally (`let zero f x = x` works for any `f`), while Rust's ownership model and monomorphized generics make pure Church encoding painful. Rust closures are concrete types, not abstract functions — each closure has a unique unnameable type, requiring `Box<dyn Fn>` for dynamic dispatch.

## OCaml

```ocaml
let zero _f x = x
let one f x = f x
let succ n f x = f (n f x)
let add m n f x = m f (n f x)
let mul m n f = m (n f)
let to_int n = n (fun x -> x + 1) 0
```

## Rust — Practical encoding

```rust
#[derive(Clone, Copy)]
pub struct ChurchNum(pub usize);

impl ChurchNum {
    pub fn succ(self) -> Self { ChurchNum(self.0 + 1) }
    pub fn add(self, other: Self) -> Self { ChurchNum(self.0 + other.0) }

    pub fn apply<T>(&self, f: impl Fn(T) -> T, x: T) -> T {
        (0..self.0).fold(x, |acc, _| f(acc))
    }
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Zero | `let zero _f x = x` | `Box::new(\|_f\| Box::new(\|x\| x))` |
| Type | Inferred polymorphic | `Box<dyn Fn(Box<dyn Fn>) -> Box<dyn Fn>>` |
| Composition | Natural function nesting | Ownership tangles |
| Practical alt | Not needed | `struct ChurchNum(usize)` |
| Performance | GC-managed closures | Heap alloc per `Box<dyn Fn>` |
| Elegance | ⭐⭐⭐⭐⭐ | ⭐⭐ (pure) / ⭐⭐⭐⭐ (struct) |

## Learner Notes

- **OCaml shines here**: Lambda calculus is OCaml's home turf — the syntax is almost mathematical notation
- **Rust's closure problem**: Each closure is a unique type. Composing them requires trait objects (`dyn Fn`) and heap allocation
- **`impl Fn` vs `dyn Fn`**: `impl Fn` is zero-cost but monomorphic; `dyn Fn` is dynamic but allocates. Church numerals need the latter
- **Practical encoding wins**: In real Rust code, wrap the value in a struct and provide `apply` — same Church semantics, zero ceremony
- **This is a language design lesson**: Some abstractions are natural in some languages and awkward in others
