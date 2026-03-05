# OCaml vs Rust: Church Encoding

Church encoding represents data purely with functions.

## Church Booleans

### OCaml
```ocaml
let church_true  a b = a
let church_false a b = b
let to_bool b = b true false
```

### Rust
```rust
fn church_true<T>() -> impl Fn(T, T) -> T { |a, _| a }
fn church_false<T>() -> impl Fn(T, T) -> T { |_, b| b }
fn to_bool(b: impl Fn(bool, bool) -> bool) -> bool { b(true, false) }
```

## Church Numerals

A number N is represented as "apply f N times":
- 0 = λf.λx.x
- 1 = λf.λx.f x
- 2 = λf.λx.f (f x)

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Closures** | Lightweight | Require explicit types |
| **Higher-rank** | Easy | Requires `impl Fn` or `dyn` |
| **Boxing** | GC handles | Manual `Box<dyn Fn>` |
