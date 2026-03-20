📖 **[View on hightechmind.io →](https://hightechmind.io/rust/278-sierpinski-triangle)**

---

# Example 278: Sierpinski Triangle — Recursive ASCII Art

**Difficulty:** ⭐⭐
**Category:** Math/Recursion
**OCaml Source:** https://rosettacode.org/wiki/Sierpinski_triangle#OCaml

## Problem Statement

Generate a Sierpinski triangle of order N as ASCII art. Each order doubles the number of lines: order 0 is a single `*`, order 1 has 2 lines, order N has 2^N lines.

## Learning Outcomes

- Recursive string generation with collect and concat
- Translating OCaml's `List.map` to Rust's `.iter().map().collect()`
- Using `format!` for string padding and duplication
- Fold-based iterative alternative to recursion

## OCaml Approach

OCaml builds the triangle recursively: base case is `["*"]`, then each level maps `pad` over previous lines for the top half and duplicates lines (`s ^ " " ^ s`) for the bottom half. `List.map` and list concatenation (`@`) make this concise.

## Rust Approach

Rust mirrors the recursive structure exactly, using `Vec<String>` instead of `string list`. `.iter().map().collect()` replaces `List.map`, and `[top, bottom].concat()` replaces `@`. An iterative version uses `fold` to build up from order 0.

## Key Differences

1. **String operations:** OCaml's `String.make n ' ' ^ s` becomes `format!("{}{}", " ".repeat(pad), s)` — both readable, different idioms
2. **List concatenation:** OCaml's `top @ bottom` is O(n); Rust's `[top, bottom].concat()` allocates a new Vec
3. **Bit shift:** Both use `1 lsl n` / `1 << n` for powers of 2 — identical semantics
4. **Mutability:** OCaml's recursion is naturally immutable; Rust's fold version uses an accumulator that's moved (not mutated) each iteration

## Exercises

1. Implement Sierpinski carpet (a 2D square fractal) using the same recursive subdivision approach, parameterized by depth.
2. Generalize the rendering function to output SVG or HTML canvas instructions instead of ASCII, so the fractal can be displayed at arbitrary resolution.
3. Implement the Koch snowflake using a string rewriting system (L-system): define production rules, apply them `n` times, then interpret the resulting string as drawing commands.
