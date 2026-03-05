# OCaml vs Rust: Struct Lifetimes

## OCaml
```ocaml
(* GC handles string ownership — no lifetime annotation *)
type highlight = {
  text: string;
  start: int;
  end_pos: int;
}

let make_highlight source start end_pos =
  { text = String.sub source start (end_pos - start);
    start; end_pos }
```

## Rust
```rust
// Struct borrowing from external string needs 'a
#[derive(Debug)]
pub struct Highlight<'a> {
    pub text: &'a str,  // borrows from source
    pub start: usize,
    pub end: usize,
}

impl<'a> Highlight<'a> {
    pub fn new(source: &'a str, start: usize, end: usize) -> Option<Self> {
        Some(Highlight { text: &source[start..end], start, end })
    }
}
```

## Key Differences

1. **OCaml**: Strings are values, copied or shared via GC
2. **Rust**: &str borrows, struct must track borrow lifetime
3. **Rust**: 'a parameter says "valid as long as source"
4. **Rust**: Struct cannot outlive borrowed data
5. Both: Enable zero-copy views into larger strings
