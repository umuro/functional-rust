📖 **[View on hightechmind.io →](https://hightechmind.io/rust/557-lifetime-output-lifetime)**

---

# Output Lifetime Patterns
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Every function returning a reference must express where that reference comes from — the output lifetime. In simple cases, elision handles this. In complex cases, understanding which input the output borrows from is essential for correctness. Three distinct patterns cover most use cases: (1) output tied to a single input, (2) output tied to the shortest common lifetime of multiple inputs, (3) output with a lifetime independent of inputs (`'static`). Getting the output lifetime wrong causes either compile errors or unnecessarily restrictive APIs.

## Learning Outcomes

- How `first_char(s: &str) -> Option<&str>` ties the output to the single input via elision
- How `common_prefix<'a>(a: &'a str, b: &'a str) -> &'a str` uses one lifetime for both inputs
- How `static_str(_s: &str) -> &'static str` returns data independent of the input
- How struct methods can return `&self`-lifetime vs stored-data-lifetime references
- When choosing the correct output lifetime affects API ergonomics

## Rust Application

`first_char(s: &str) -> Option<&str>` — elision ties the output to `s`. `common_prefix<'a>(a: &'a str, b: &'a str) -> &'a str` — output tied to the intersection of `a` and `b`'s lifetime (same `'a`). `static_str(_s: &str) -> &'static str` — ignores the input lifetime, returns program-duration data. `Container::get(&self, i: usize) -> Option<&str>` returns a reference to items stored in `self` — tied to `self`'s lifetime by Rule 3.

Key patterns:
- Single input → output tied to it (elision)
- `'a` covering multiple inputs → output is valid when both inputs are
- `&'static str` return — independent of any input

## OCaml Approach

OCaml functions return GC-managed values — there are no output lifetimes. The equivalent functions are:

```ocaml
let first_char s = if String.length s = 0 then None else Some (String.sub s 0 1)
let common_prefix a b = (* find and return common prefix as new string *)
let static_str _ = "static"
```

All returned values are GC-managed; no lifetime annotation describes where they came from.

## Key Differences

1. **Output source tracing**: Rust requires the programmer to specify which input a returned reference borrows from; OCaml returns owned or GC-managed values with no source annotation needed.
2. **Performance**: Rust `first_char` returns a `&str` slice — zero allocation; OCaml `String.sub` copies the character into a new string.
3. **Independent output**: Rust `-> &'static str` is a common optimization for returning compile-time constants; OCaml constants are also zero-allocation but through GC interning.
4. **Error source**: When output lifetime is wrong in Rust, the compiler reports "does not live long enough" at the use site; OCaml never reports lifetime errors.

## Exercises

1. **Multiple output lifetimes**: Write `fn split_first<'a>(s: &'a str, sep: char) -> (&'a str, &'a str)` returning the part before and after the first occurrence of `sep` as zero-copy slices.
2. **Independent static**: Implement `fn status_message(code: u16) -> &'static str` using a `match` expression returning string literal branches — verify no heap allocation occurs.
3. **Container iterator**: Add a method `fn iter(&self) -> impl Iterator<Item = &str>` to `Container` and verify the returned iterator's lifetime is tied to `&self`.
