# OCaml vs Rust: String.sub and String.concat — Substring and Join

## Side-by-Side Code

### OCaml

```ocaml
let s = "Hello, World!"
let hello = String.sub s 0 5     (* "Hello" *)
let world = String.sub s 7 5     (* "World" *)

let parts = ["one"; "two"; "three"]
let joined = String.concat " | " parts   (* "one | two | three" *)
```

### Rust (idiomatic)

```rust
// Substring — safe, returns Option
pub fn substring(s: &str, pos: usize, len: usize) -> Option<&str> {
    s.get(pos..pos + len)
}

// Join — direct equivalent of String.concat
pub fn join(sep: &str, parts: &[&str]) -> String {
    parts.join(sep)
}
```

### Rust (functional / char-safe)

```rust
// Char-index substring — safe for Unicode, matches OCaml's character semantics
pub fn substring_chars(s: &str, pos: usize, len: usize) -> String {
    s.chars().skip(pos).take(len).collect()
}

// Fold-based join — explicit reduction matching OCaml's List.fold_left
pub fn join_iter(sep: &str, parts: &[&str]) -> String {
    let mut iter = parts.iter().copied();
    match iter.next() {
        None => String::new(),
        Some(first) => iter.fold(first.to_owned(), |mut acc, part| {
            acc.push_str(sep);
            acc.push_str(part);
            acc
        }),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Substring (byte) | `val sub : string -> int -> int -> string` | `fn substring(s: &str, pos: usize, len: usize) -> Option<&str>` |
| Substring (char) | `String.sub` (bytes = chars for ASCII) | `fn substring_chars(s: &str, pos: usize, len: usize) -> String` |
| Join | `val concat : string -> string list -> string` | `fn join(sep: &str, parts: &[&str]) -> String` |
| String list | `string list` | `&[&str]` (slice of string slices) |
| Optional result | `raises Invalid_argument` | `Option<&str>` |

## Key Insights

1. **Safety by type, not exception:** OCaml raises `Invalid_argument` when `pos` or `len` is out of range; Rust's `s.get(range)` returns `None`, pushing the caller to handle the absence explicitly — no exceptions can escape unnoticed.

2. **Zero-copy substring:** Rust `s.get(pos..pos+len)` returns `Option<&str>`, a borrowed slice into the original string — no heap allocation. OCaml's `String.sub` always allocates a fresh string on the OCaml heap.

3. **Byte vs Unicode char indexing:** OCaml strings are byte sequences (valid UTF-8 not required). `String.sub` indexes by byte, which equals character position only for ASCII. Rust `&str` is guaranteed UTF-8; byte-index slicing panics (or returns `None`) when it would split a multibyte character. The `.chars().skip().take()` idiom gives true char-level indexing safe for all Unicode inputs.

4. **Argument order reversal:** OCaml `String.concat sep list` takes the separator first, then the list. Rust `parts.join(sep)` is a method on the slice — the separator is the argument. Both produce identical output; the difference is purely syntactic convention.

5. **Fold as the underlying model:** `String.concat` is conceptually a `List.fold_left` with string append. The `join_iter` Rust implementation makes this explicit — it mirrors the OCaml mental model, which is useful when the separator logic needs to be customized beyond what `slice::join` supports.

## When to Use Each Style

**Use `s.get(pos..pos+len)` (byte slicing):** When you know your input is ASCII or you are already working with byte offsets (e.g., from a parser). Fastest path, zero allocation.

**Use `.chars().skip(n).take(m).collect()` (char iteration):** When processing user-visible text that may contain multibyte Unicode characters and you want positions that match human-readable character counts.

**Use `parts.join(sep)` (idiomatic join):** Always, unless you need to transform or filter elements during joining — it's the clearest, most efficient form and directly signals intent.

**Use fold-based `join_iter`:** When joining requires conditional separators, filtering, or accumulation logic beyond simple interleaving — the fold structure gives full control over each step.
