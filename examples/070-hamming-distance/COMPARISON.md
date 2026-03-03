# Hamming Distance: OCaml vs Rust

## The Core Insight
Hamming distance is a perfect showcase for the zip-filter-count pipeline pattern. Both languages express it elegantly, but Rust's lazy iterators avoid the intermediate list allocations that OCaml's eager `List.combine` creates.

## OCaml Approach
OCaml's approach first converts strings to character lists with `chars_of_string`, then uses `List.combine` to pair corresponding characters, `List.filter` to keep mismatches, and `List.length` to count them. Each step creates a new list. The `fold_left2` variant avoids intermediate lists by folding over two lists simultaneously. Error handling uses exceptions (`raise Invalid_argument`).

## Rust Approach
Rust's `s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count()` is a single lazy pipeline — no intermediate collections are allocated. Each iterator adapter transforms the stream without materializing it. Error handling uses `Result<usize, String>`, forcing callers to handle the error case at compile time. The byte-level variant (`&[u8]`) avoids UTF-8 decoding overhead for ASCII strings.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| String to chars | `chars_of_string` (custom) | `.chars()` (built-in iterator) |
| Zip | `List.combine` (eager, allocates) | `.zip()` (lazy, zero allocation) |
| Filter | `List.filter` (eager) | `.filter()` (lazy) |
| Count | `List.length` | `.count()` |
| Errors | `raise (Invalid_argument ...)` | `Err("...".to_string())` |
| Fold variant | `List.fold_left2` | `.zip().fold(...)` |

## What Rust Learners Should Notice
- Rust iterators are lazy — `.zip().filter().count()` makes a single pass with zero allocation, compared to OCaml's multiple intermediate lists
- `Result<T, E>` in the return type forces callers to handle the unequal-length case — no hidden exceptions
- `.chars()` iterates over Unicode scalar values; for ASCII-only strings, working with `.bytes()` or `&[u8]` is faster
- The tuple destructuring `|(a, b)|` in closures mirrors OCaml's `(fun (a, b) -> ...)`
- Pattern: `iter().zip().filter().count()` is one of Rust's most common and powerful iterator patterns

## Further Reading
- [Rust Iterator::zip](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
- [Exercism OCaml Hamming](https://exercism.org/tracks/ocaml/exercises/hamming)
