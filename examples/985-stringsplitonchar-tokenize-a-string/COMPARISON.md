# OCaml vs Rust: String.split_on_char — Tokenize a String

## Side-by-Side Code

### OCaml

```ocaml
(* Split a CSV line *)
let fields = String.split_on_char ',' "Alice,30,Engineer,Amsterdam"
(* = ["Alice"; "30"; "Engineer"; "Amsterdam"] *)

(* Filter empty tokens from whitespace split *)
let words =
  List.filter (fun s -> s <> "") (String.split_on_char ' ' "  hello   world  ")
(* = ["hello"; "world"] *)
```

### Rust (idiomatic)

```rust
// Split a CSV line — lazy iterator, collect into Vec<&str>
let fields: Vec<&str> = "Alice,30,Engineer,Amsterdam".split(',').collect();
// = ["Alice", "30", "Engineer", "Amsterdam"]

// Filter empty tokens
let words: Vec<&str> = "  hello   world  "
    .split(' ')
    .filter(|s| !s.is_empty())
    .collect();
// = ["hello", "world"]

// Or use split_whitespace which collapses runs automatically
let words2: Vec<&str> = "  hello   world  ".split_whitespace().collect();
```

### Rust (functional / recursive)

```rust
pub fn split_recursive(s: &str, delimiter: char) -> Vec<&str> {
    match s.find(delimiter) {
        None => vec![s],
        Some(pos) => {
            let mut result = vec![&s[..pos]];
            result.extend(split_recursive(&s[pos + delimiter.len_utf8()..], delimiter));
            result
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Split function | `String.split_on_char : char -> string -> string list` | `str::split(char) -> Split<'_, char>` |
| Result type | `string list` | `Vec<&str>` (borrowed slices) |
| Filter step | `List.filter : ('a -> bool) -> 'a list -> 'a list` | `.filter(\|s\| !s.is_empty())` on the iterator |
| Whitespace shortcut | manual filter | `str::split_whitespace() -> SplitWhitespace<'_>` |
| Character type | `char` (Unicode scalar) | `char` (Unicode scalar) |

## Key Insights

1. **Zero-copy slices.** Rust's `split` returns `&str` slices that borrow from the original string.
   OCaml returns a list of newly allocated `string` values. Rust avoids allocation until `.collect()`.

2. **Lazy vs eager.** `str::split` produces a lazy iterator; tokens are computed on demand.
   OCaml's `String.split_on_char` builds the full list immediately.

3. **Empty-token behaviour is identical.** Both OCaml and Rust include empty strings when delimiters
   appear consecutively or at string boundaries. Both require an explicit filter pass to remove them.

4. **`split_whitespace` is a Rust extra.** OCaml has no direct equivalent — it requires splitting on
   `' '` then filtering. `split_whitespace` collapses any run of Unicode whitespace in one step.

5. **UTF-8 safety in recursion.** When slicing past a multi-byte delimiter (e.g. `'·'`, U+00B7),
   `char::len_utf8()` ensures the byte offset is correct. OCaml's `String.split_on_char` operates
   on bytes for ASCII chars but is designed for single-byte delimiters.

6. **Recursive style maps directly.** OCaml's natural recursion over a `char list` becomes recursion
   over string slices in Rust, with `find` playing the role of pattern-matching the head character.

## When to Use Each Style

**Use idiomatic Rust (`.split().filter().collect()`)** for production code: it is zero-copy, lazy,
and composes freely with other iterator adaptors (`.map`, `.take`, `.enumerate`, etc.).

**Use `split_whitespace()`** whenever your delimiter is whitespace — it handles tabs, newlines, and
consecutive spaces without extra filtering.

**Use the recursive style** when teaching the OCaml-to-Rust translation or when you need to process
tokens as they are found (e.g., early termination), since it makes the structural recursion explicit.
