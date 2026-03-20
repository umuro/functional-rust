📖 **[View on hightechmind.io →](https://hightechmind.io/rust/528-closure-lifetime-capture)**

---

# Closures Capturing References

## Problem Statement

When a closure borrows data from its environment rather than moving it, the closure's lifetime is constrained by the validity of those borrows. This is the intersection of two of Rust's most distinctive features: closures and the borrow checker. The challenge is expressing in the type system that "this closure is valid as long as the data it borrows is valid." Getting this right enables zero-copy parsing, view-based APIs, and efficient filtering over borrowed slices without unnecessary cloning.

## Learning Outcomes

- How `'a` lifetime annotations constrain closures that capture references
- Why a closure returning `impl Fn(&str) -> bool + 'a` ties its lifetime to captured data
- How to build structs like `Filter<'a, T>` that hold both data and a closure over that data
- How `make_validator` captures two references with the same lifetime `'a`
- Where reference-capturing closures appear: parsers, search indices, view layers

## Rust Application

`make_prefix_checker<'a>(prefix: &'a str) -> impl Fn(&str) -> bool + 'a` captures `prefix` by reference — the returned closure can only live as long as `prefix`. `make_range_checker<'a>(data: &'a [i32])` similarly ties the closure to the slice's lifetime. `Filter<'a, T>` holds both `data: &'a [T]` and `predicate: Box<dyn Fn(&T) -> bool + 'a>` — the lifetime `'a` ensures the predicate cannot outlive the data it operates on. `make_validator<'a>(min, max)` captures two `&'a i32` references in one closure.

Key patterns:
- `impl Fn(&str) -> bool + 'a` — closure with bounded lifetime as return type
- `Box<dyn Fn(&T) -> bool + 'a>` — type-erased closure tied to data lifetime
- Lifetime elision for simple cases like `fn first_word(s: &str) -> &str`

## OCaml Approach

OCaml closures capture references to the GC heap — there are no lifetime annotations. The GC ensures captured values remain alive as long as the closure exists. The equivalent of `make_prefix_checker` is simply:

```ocaml
let make_prefix_checker prefix = fun s -> String.is_prefix s ~prefix
```

No lifetime annotation is needed because the GC prevents the prefix from being freed.

## Key Differences

1. **Lifetime annotations**: Rust requires explicit `'a` to express that a closure borrows from external data; OCaml relies on the GC to keep all captured values alive, requiring no annotations.
2. **Zero-copy semantics**: Rust closures over `&[T]` enable zero-copy filtering; OCaml's `list` slices are not zero-copy — sublist operations create new list nodes.
3. **Lifetime propagation**: Rust propagates `'a` through struct fields, function signatures, and trait objects; OCaml has no equivalent concept — all captured values are safe by construction.
4. **Error location**: When Rust closure lifetimes are wrong, the compiler reports the conflicting borrow with a specific variable and scope; OCaml never reports these errors because the GC prevents the issue.

## Exercises

1. **Multi-reference closure**: Write `make_between_checker<'a>(lo: &'a str, hi: &'a str) -> impl Fn(&str) -> bool + 'a` that returns true when the input is lexicographically between `lo` and `hi`.
2. **Lifetime-bounded filter struct**: Implement `struct Searcher<'a> { text: &'a str, pattern: &'a str }` with a method `matches_at(pos: usize) -> bool` that checks for the pattern at a given position.
3. **Two-lifetime struct**: Create `struct Join<'a, 'b> { left: &'a [i32], right: &'b [i32] }` with a method `merged_sorted(&self) -> Vec<i32>` that merges without requiring both lifetimes to be equal.
