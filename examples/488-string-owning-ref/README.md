📖 **[View on hightechmind.io →](https://hightechmind.io/rust/488-string-owning-ref)**

---

# String Owning References

Self-referential data — a struct that owns a string and also holds slices into it — is the hardest string pattern in Rust. The safe solution stores byte-offset pairs rather than `&str` references, extracting slices on demand.

## Problem Statement

A common parser pattern is to own an input string and cache the positions of tokens within it: `{ source: String, tokens: Vec<&str> }`. This is a **self-referential struct** — `tokens` would borrow from `source` in the same struct, which Rust's borrow checker forbids. The safe workaround stores `(usize, usize)` byte offsets instead of `&str` references and reconstructs slices from `&self.source[start..end]` when needed. This is how `logos`, `nom`, and most Rust parsers work internally.

## Learning Outcomes

- Understand why Rust forbids self-referential structs with borrowed fields
- Store byte offset pairs `(usize, usize)` as a safe alternative to cached `&str`
- Reconstruct `&str` slices from stored offsets on-demand
- Understand `Cow`-based tri-variant string ownership (`Static`, `Owned`, `Borrowed`)
- Recognise when `Pin<Box<T>>` is needed for genuinely self-referential data

## Rust Application

`ParsedString` owns the source and stores word boundaries as byte offsets:

```rust
pub struct ParsedString {
    source: String,
    words: Vec<(usize, usize)>,  // (start, end) byte indices
}
```

`get_word(index)` reconstructs the slice on demand:

```rust
pub fn get_word(&self, index: usize) -> Option<&str> {
    self.words.get(index)
        .map(|(start, end)| &self.source[*start..*end])
}
```

`StringOrStatic` demonstrates a tri-variant string type covering the three ownership cases: `&'static str` (no allocation, infinite lifetime), `String` (owned), and `&'a str` (borrowed with lifetime `'a`).

## OCaml Approach

OCaml's GC makes self-referential structures straightforward — the GC follows all pointers, so a struct can hold both an owning reference and a derived slice:

```ocaml
type parsed = {
  source: string;
  words: (int * int) list;  (* or store string directly *)
}

let get_word p i =
  let (start, len) = List.nth p.words i in
  String.sub p.source start len  (* allocates — no slice type *)
```

OCaml's lack of a zero-copy slice type means `get_word` always allocates with `String.sub`; Rust's approach is zero-copy.

## Key Differences

1. **Self-referential structs**: Rust forbids them (borrow checker) without `unsafe`; OCaml allows them freely because the GC manages all lifetimes.
2. **Zero-copy slices**: Rust's offset-based approach returns `&str` with no allocation; OCaml's `String.sub` always copies.
3. **`Pin`**: Rust's `Pin<Box<T>>` prevents a self-referential struct from moving in memory (which would invalidate internal pointers); OCaml moves objects during GC compaction but updates all pointers automatically.
4. **`Cow` lifetime**: Rust's `Cow<'a, str>` carries a lifetime parameter tying the borrowed variant to its source; OCaml has no equivalent — all strings are GC-lifetime.

## Exercises

1. **Line-number tracker**: Build `LineIndex { source: String, line_starts: Vec<usize> }` that precomputes newline positions and provides `fn line(&self, n: usize) -> &str`.
2. **Token stream**: Build `Tokenizer` that stores the source `String` and a `Vec<(TokenKind, usize, usize)>` for token type, start, and end byte offsets. Implement an iterator that yields `(TokenKind, &str)` by slicing on demand.
3. **Genuine self-reference with `ouroboros`**: Use the `ouroboros` crate to create a `SelfRefParsed` struct that safely stores both the `String` and `Vec<&str>` references, and compare ergonomics against the offset approach.
