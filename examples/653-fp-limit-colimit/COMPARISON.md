# OCaml vs Rust: Limits and Colimits

## Product

### OCaml
```ocaml
type ('a, 'b) product = 'a * 'b
```

### Rust
```rust
struct Product<A, B>(A, B);
// Or just (A, B)
```

## Coproduct

### OCaml
```ocaml
type ('a, 'b) coproduct = Left of 'a | Right of 'b
```

### Rust
```rust
enum Coproduct<A, B> { Left(A), Right(B) }
// Or std::result::Result
```

## Initial Object

### OCaml
```ocaml
type initial = |  (* empty variant *)
```

### Rust
```rust
enum Initial {}  // Never type, also written as !
```

## Key Insight

Both languages naturally express categorical concepts:
- Tuples = Products
- Enums/Variants = Coproducts
- Unit = Terminal
- Never/Empty = Initial
