📖 **[View on hightechmind.io →](https://hightechmind.io/rust/107-lifetime-structs)**

---

# 107-lifetime-structs — Lifetimes in Structs
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

When a struct holds a reference, the struct's validity is tied to the data it borrows. A struct holding `&str` cannot outlive the `String` it was created from — if the `String` is dropped, the struct would hold a dangling pointer. Rust's lifetime annotations on structs make this relationship explicit, preventing the struct from being used after its data is freed.

This is the pattern behind zero-copy parsers (structs holding slices into the original input), iterator adapters (structs holding references to collections), and any API that borrows from caller data.

## Learning Outcomes

- Annotate a struct with a lifetime parameter `<'a>`
- Understand that the struct lives at most as long as the borrowed data
- Use lifetime annotations in `impl` blocks
- Handle multiple borrowed fields sharing or having independent lifetimes
- Connect to zero-copy parsing patterns in `nom`, `winnow`, and `serde`

## Rust Application

`src/lib.rs` defines `Excerpt<'a> { text: &'a str, page: u32 }` — the struct lives at most as long as the `str` it borrows. The `impl<'a>` block's `announce` method uses lifetime elision (rule 3) to omit the output lifetime. `Article<'a>` holds three `&'a str` fields — the same lifetime `'a` means the struct lives only as long as the shortest-lived of its borrowed strings.

Zero-copy parsing (JSON, HTTP headers, binary protocols) is the primary use case: parse a large buffer once into a struct of slices, all pointing into the original buffer — no string copies needed.

## OCaml Approach

OCaml structs holding string references have no equivalent constraint — the GC keeps everything alive:

```ocaml
type excerpt = { text: string; page: int }

let make_excerpt text page = { text; page }
(* text can be freed by OCaml's GC only when all references are dropped *)
```

An OCaml `excerpt` can outlive any particular binding to the original string because the GC tracks reference counts. There is no concept of "borrows from" in OCaml's type system.

## Key Differences

1. **Lifetime annotation burden**: Rust requires explicit `<'a>` on any struct holding a reference; OCaml requires no annotations.
2. **Zero-copy semantics**: Rust's lifetime annotation makes zero-copy safe — the compiler proves the slice is valid; OCaml's GC ensures validity at runtime.
3. **Multiple lifetimes**: Rust structs can have multiple independent lifetime parameters (`<'a, 'b>`); OCaml has no equivalent.
4. **Compile-time vs runtime**: Rust enforces borrowing at compile time with zero runtime overhead; OCaml's GC enforces memory safety at runtime.

## Exercises

1. Create a `ParseResult<'a> { input: &'a str, consumed: &'a str, remaining: &'a str }` struct for a simple parser and implement a `parse_word` function.
2. Write a `Config<'a>` struct that borrows from a `&'a str` configuration file content and provides methods to look up values.
3. Demonstrate the lifetime error when trying to store an `Excerpt<'a>` in a `Vec` that outlives the source string.
