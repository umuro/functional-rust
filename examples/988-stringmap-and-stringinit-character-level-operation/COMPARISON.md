# OCaml vs Rust: String.map and String.init — Character-level Operations

## Side-by-Side Code

### OCaml

```ocaml
(* String.map: apply f to each character *)
let to_upper s = String.map Char.uppercase_ascii s

(* ROT-13 using String.map *)
let rot13 c =
  if c >= 'a' && c <= 'z' then Char.chr ((Char.code c - Char.code 'a' + 13) mod 26 + Char.code 'a')
  else if c >= 'A' && c <= 'Z' then Char.chr ((Char.code c - Char.code 'A' + 13) mod 26 + Char.code 'A')
  else c

let encoded = String.map rot13 "Hello World"

(* String.init: build a string from indices *)
let alphabet = String.init 26 (fun i -> Char.chr (i + Char.code 'a'))
```

### Rust (idiomatic)

```rust
// string_map: apply f to each char — mirrors String.map
pub fn string_map(s: &str, f: impl Fn(char) -> char) -> String {
    s.chars().map(f).collect()
}

// string_init: build a String from indices — mirrors String.init
pub fn string_init(n: usize, f: impl Fn(usize) -> char) -> String {
    (0..n).map(f).collect()
}
```

### Rust (ROT-13 with match pattern)

```rust
pub fn rot13(c: char) -> char {
    match c {
        'a'..='z' => (b'a' + (c as u8 - b'a' + 13) % 26) as char,
        'A'..='Z' => (b'A' + (c as u8 - b'A' + 13) % 26) as char,
        _ => c,
    }
}

pub fn rot13_string(s: &str) -> String {
    s.chars().map(rot13).collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Map over string | `String.map : (char -> char) -> string -> string` | `fn string_map(s: &str, f: impl Fn(char) -> char) -> String` |
| Init from indices | `String.init : int -> (int -> char) -> string` | `fn string_init(n: usize, f: impl Fn(usize) -> char) -> String` |
| Character type | `char` (byte in OCaml 4, Unicode scalar in OCaml 5) | `char` (always a Unicode scalar value, 4 bytes) |
| String type | `string` (mutable byte array) | `&str` (borrowed UTF-8 slice) / `String` (owned) |
| Integer index type | `int` | `usize` |

## Key Insights

1. **`String.map` → `.chars().map().collect()`**: OCaml provides this as a named stdlib function; Rust composes standard iterator adapters. The result is identical: a new string with each character transformed.

2. **`String.init` → `(0..n).map(f).collect()`**: OCaml's dedicated constructor is replaced by a range iterator in Rust. The range `0..n` acts as the index source — no special function required, just the universal iterator protocol.

3. **Encoding matters for char ops**: OCaml 4 strings are byte sequences (Latin-1 in practice); Rust strings are UTF-8. In Rust `.chars()` yields full Unicode scalar values (`char`), making the type actually safer for multi-byte characters — but arithmetic on `char` via `as u8` only makes sense for ASCII input, as done here.

4. **ROT-13 uses range pattern matching**: Rust's `'a'..='z'` range patterns in `match` make the cipher implementation concise and readable — far cleaner than chained `if/else` as in OCaml.

5. **ROT-13 is its own inverse**: Because ROT-13 shifts by half the alphabet (13 of 26), applying it twice returns the original string. This is a clean property to test: `rot13(rot13(s)) == s`.

6. **`impl Fn` vs function pointers**: In Rust, `impl Fn(char) -> char` accepts closures and named functions alike. This mirrors OCaml's ability to pass any `char -> char` function — including locally defined lambdas or module-level functions.

## When to Use Each Style

**Use idiomatic Rust (`chars().map().collect()`)** when transforming existing string content — it composes naturally with other iterator operations like `.filter()`, `.take()`, or `.enumerate()`.

**Use `(0..n).map(f).collect()`** when constructing a string from scratch based on position — for sequences, alphabets, repeated patterns, or index-derived content.
