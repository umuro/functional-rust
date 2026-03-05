# OCaml vs Rust: Enum Lifetimes

## OCaml
```ocaml
(* Variant can hold string without lifetime annotation *)
type token =
  | Word of string
  | Number of int
  | Punctuation of char
  | End

type 'a parse_result =
  | Ok of 'a * string
  | Err of string * string
```

## Rust
```rust
// Enum variant with reference needs lifetime
#[derive(Debug)]
pub enum Token<'a> {
    Word(&'a str),    // borrows from input
    Number(i64),
    Punctuation(char),
    End,
}

pub enum ParseResult<'a, T> {
    Ok(T, &'a str),           // remaining input
    Err(&'a str, String),     // failing position + message
}
```

## Key Differences

1. **OCaml**: string in variant is owned/GC-managed
2. **Rust**: &'a str in variant borrows from external source
3. **Rust**: Enum lifetime means "valid while source is valid"
4. Both: Enums can contain references or values
5. **Rust**: Zero-copy parsing possible with borrowed tokens
