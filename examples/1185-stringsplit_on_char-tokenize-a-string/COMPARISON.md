# OCaml vs Rust: String.split_on_char — Tokenize a String

## Side-by-Side Code

### OCaml
```ocaml
let csv_line = "Alice,30,Engineer,Amsterdam"
let fields = String.split_on_char ',' csv_line
let () = List.iteri (fun i f -> Printf.printf "Field %d: %s\n" i f) fields

let words = String.split_on_char ' ' "  hello   world  "
let nonempty = List.filter (fun s -> s <> "") words
```

### Rust (idiomatic)
```rust
let csv_line = "Alice,30,Engineer,Amsterdam";
let fields: Vec<&str> = csv_line.split(',').collect();
for (i, f) in fields.iter().enumerate() {
    println!("Field {}: {}", i, f);
}

let words: Vec<&str> = "  hello   world  "
    .split_whitespace()
    .collect();
```

### Rust (functional pipeline)
```rust
pub fn split_nonempty(s: &str, delim: char) -> Vec<&str> {
    s.split(delim).filter(|t| !t.is_empty()).collect()
}

pub fn parse_csv(line: &str) -> Vec<&str> {
    line.split(',').map(str::trim).collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| split | `String.split_on_char : char -> string -> string list` | `str::split(pattern) -> Split<'_, char>` (lazy iterator) |
| result type | `string list` (owned, allocated) | `Vec<&str>` (borrows from source) |
| filter empty | `List.filter (fun s -> s <> "") tokens` | `.filter(\|s\| !s.is_empty())` (iterator adapter) |
| index iteration | `List.iteri (fun i f -> ...) fields` | `fields.iter().enumerate()` |
| whitespace split | `split ' '` + filter | `str::split_whitespace()` (built-in) |

## Key Insights

1. **Eager vs. lazy**: OCaml's `String.split_on_char` immediately allocates and returns a `string list`; Rust's `str::split` returns a lazy `Split<'_, char>` iterator — no allocation until `.collect()` is called, and you can chain further adapters without intermediate collections.
2. **Argument order**: OCaml: `String.split_on_char delimiter string` (delimiter first — pipe-friendly); Rust: `string.split(delimiter)` (method on the string, delimiter as argument).
3. **Ownership**: OCaml returns owned `string` values; Rust's `split` returns `&str` slices that borrow from the original string — zero-copy, but the slices cannot outlive the source without cloning.
4. **Consecutive delimiters**: Both languages preserve empty strings between consecutive delimiters by default. Removing them requires `List.filter (fun s -> s <> "")` in OCaml or `.filter(|s| !s.is_empty())` in Rust.
5. **split_once**: Rust 1.52+ provides `str::split_once(delim)` which returns `Option<(&str, &str)>` for the first occurrence — a common pattern with no direct OCaml stdlib equivalent.

## When to Use Each Style

**Use `.split().collect()` when:** you need a `Vec<&str>` to index into or pass around.
**Use `.split().filter()...` as a lazy chain when:** you only need to iterate — avoid materializing a `Vec` if you process the tokens in a single pass.
**Use `split_whitespace()` when:** splitting on any whitespace and ignoring runs of spaces — it's shorter and clearer than `split(' ').filter(|s| !s.is_empty())`.
