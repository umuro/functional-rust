# OCaml vs Rust: Lifetime Elision

## OCaml
```ocaml
(* No lifetime annotations ever needed *)
let strlen s = String.length s
let first_word s = List.hd (String.split_on_char ' ' s)
```

## Rust
```rust
// Elision Rule 1: Each input gets own lifetime
fn strlen(s: &str) -> usize  // &'a str implicitly

// Elision Rule 2: One input → output gets same lifetime
fn first_word(s: &str) -> &str  // &'a str → &'a str

// Elision Rule 3: &self → output gets self's lifetime
impl Parser { fn remaining(&self) -> &str }

// Multiple inputs: explicit required
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str
```

## Key Differences

1. **OCaml**: No concept of lifetimes
2. **Rust**: Three elision rules reduce annotation burden
3. **Rust**: Compiler infers common patterns
4. **Rust**: Explicit when ambiguous (multiple inputs)
5. Both: Clean API, different mechanisms
