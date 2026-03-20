📖 **[View on hightechmind.io →](https://hightechmind.io/rust/533-lifetime-struct)**

---

# Lifetimes in Structs
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Most structs own their data outright. But some structs are intentionally views or windows into existing data — text highlights, tokenizer state, zero-copy parsers, iterator adapters. These structs hold references rather than owned values, which means their validity is tied to the lifetime of the data they reference. Rust's lifetime parameters on structs make this relationship explicit in the type, preventing a view struct from outliving its source data. This pattern is essential for zero-copy parsing (nom, winnow), text processing, and embedded data structures.

## Learning Outcomes

- Why structs with reference fields require lifetime parameters: `struct Highlight<'a>`
- How `Highlight<'a>` borrows from a source string and cannot outlive it
- How iterators holding a reference (`struct Words<'a>`) work with lifetime-annotated `Iterator` impls
- How multiple reference fields in a struct can share or have independent lifetimes
- Where lifetime-annotated structs appear: parsers (nom), zero-copy deserializers (serde)

## Rust Application

`Highlight<'a>` stores `text: &'a str` and position info — its `new` constructor takes `source: &'a str` and creates a slice into it. The caller cannot drop `source` while a `Highlight` exists. `Words<'a>` is an iterator holding `source: &'a str` and a `position: usize` — its `Iterator` impl yields `&'a str` slices without copying. Both structs use the same `'a` for all reference fields since they borrow from a single source, but structs borrowing from multiple sources would need independent lifetime parameters.

Key patterns:
- `struct Foo<'a> { field: &'a str }` — struct tied to external data's lifetime
- `impl<'a> Iterator for Words<'a>` with `type Item = &'a str` — yielding borrowed slices
- `Option<Self>` constructor: `new` returns `None` for invalid ranges

## OCaml Approach

OCaml string views use `string * int * int` tuples or a dedicated `Bigarray` slice. Since OCaml strings are immutable and GC-managed, holding a slice is safe with no annotations:

```ocaml
type highlight = { text: string; start: int; end_: int }
let make_highlight source start end_ =
  if end_ <= String.length source then Some { text = String.sub source start (end_ - start); start; end_ }
  else None
```

Note: `String.sub` copies — zero-copy substring views require `Bytes` or `Bigarray`.

## Key Differences

1. **Zero-copy slices**: Rust `&'a str` in a struct is a true zero-copy view into the source; OCaml `String.sub` copies the substring — zero-copy requires lower-level types.
2. **Lifetime annotation**: Rust requires `<'a>` on struct definitions holding references; OCaml records hold GC-managed values with no lifetime annotation needed.
3. **Iterator lifetimes**: Rust `impl Iterator` on a struct with `'a` yields references tied to the source; OCaml iterators return owned values by default.
4. **Compile-time vs GC**: Rust prevents use-after-free at compile time by tracking lifetimes; OCaml prevents it at runtime via garbage collection.

## Exercises

1. **Line iterator**: Implement `struct Lines<'a> { source: &'a str, pos: usize }` with `Iterator<Item = &'a str>` that yields each line (split on `'\n'`) as a zero-copy slice.
2. **Token view**: Write `struct Token<'a> { kind: TokenKind, text: &'a str }` where `TokenKind` is an enum and `text` borrows from the input source string.
3. **Multi-source struct**: Implement `struct Merge<'a, 'b> { left: &'a [i32], right: &'b [i32], pos_left: usize, pos_right: usize }` as a merge-sort iterator yielding `i32` values in sorted order.
