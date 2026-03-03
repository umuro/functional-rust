# Decode Run-Length Encoding: OCaml vs Rust

## The Core Insight
Decoding RLE is a one-to-many expansion: each encoded item produces one or more output elements. This is the natural domain of `flat_map` (Rust) / `concat_map` (OCaml). The problem also shows how algebraic types guide the transformation — each variant has a clear expansion rule.

## OCaml Approach
OCaml's `List.concat_map` (4.10+) handles the expansion cleanly:
```ocaml
let decode lst =
  List.concat_map (function
    | One x -> [x]
    | Many (n, x) -> List.init n (fun _ -> x))
  lst
```
The recursive version pattern-matches to reduce `Many(n, x)` step by step, prepending x and decrementing n until it reaches zero.

## Rust Approach
Rust's `flat_map` combined with `std::iter::repeat` is equally concise:
```rust
pub fn decode<T: Clone>(encoded: &[RleItem<T>]) -> Vec<T> {
    encoded.iter().flat_map(|item| match item {
        RleItem::One(x) => vec![x.clone()],
        RleItem::Many(n, x) => vec![x.clone(); *n],
    }).collect()
}
```
The `vec![val; count]` macro creates n copies, and `flat_map` concatenates all expansions.

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| Expansion | `List.concat_map` | `.flat_map()` |
| Repeat n times | `List.init n (fun _ -> x)` | `vec![x; n]` or `repeat(x).take(n)` |
| Pattern matching | `function \| One x -> ... \| Many (n,x) -> ...` | `match item { One(x) => ..., Many(n,x) => ... }` |
| Cloning | Automatic (GC) | Explicit `.clone()` required |
| Lazy vs eager | Eager (creates lists) | Lazy until `.collect()` |

## What Rust Learners Should Notice
- **`flat_map` is your friend**: Whenever you need to produce zero, one, or many items per input, `flat_map` (OCaml: `concat_map`) is the right combinator. It's more general than `map`.
- **`vec![x; n]` clones**: The macro `vec![x.clone(); n]` calls clone n times. For large n with expensive clones, consider `std::iter::repeat_with`.
- **Iterators compose lazily**: Rust's `flat_map` doesn't create intermediate vectors — it yields elements one at a time. The `vec!` inside is allocated, but `repeat().take()` avoids even that.
- **The recursive approach shows ownership cost**: Each recursive step in Rust needs `clone()` calls that OCaml avoids due to GC. This is the explicit trade-off.

## Further Reading
- [Rust — Iterator::flat_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)
- [OCaml — List.concat_map](https://v2.ocaml.org/api/List.html)
- [99 OCaml Problems #12](https://ocaml.org/problems)
