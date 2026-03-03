# Modified Run-Length Encoding: OCaml vs Rust

## The Core Insight
This problem naturally requires a sum type to represent two kinds of elements: singles and runs. Both OCaml and Rust make this elegant with algebraic data types. The interesting part is how each language handles the stateful traversal — accumulating runs of equal elements while distinguishing singletons.

## OCaml Approach
OCaml defines a parameterized variant type and uses pattern matching with accumulators:
```ocaml
type 'a rle = One of 'a | Many of int * 'a

let encode lst =
  pack lst |> List.map (fun group ->
    match group with
    | [x] -> One x
    | x :: _ -> Many (List.length group, x)
    | [] -> failwith "impossible")
```
The `pack` helper groups consecutive duplicates into sublists first, then `encode` classifies each group. The direct version tracks count in a recursive accumulator.

## Rust Approach
Rust uses a generic enum with the same structure:
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum RleItem<T> { One(T), Many(usize, T) }
```
The idiomatic approach uses index-based iteration with equality checks. The recursive version uses `split_first()` or position-finding to identify runs. Both require `Clone` because Rust can't share data implicitly.

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| Sum type | `type 'a rle = One of 'a \| Many of int * 'a` | `enum RleItem<T> { One(T), Many(usize, T) }` |
| Type parameter | `'a` (implicit) | `<T: PartialEq + Clone>` (explicit bounds) |
| Equality | Structural (built-in) | Requires `PartialEq` trait |
| Copying | Automatic (GC) | Requires `Clone` |
| Grouping | `pack` into sublists, then map | Direct counting (no intermediate lists) |
| Pattern matching | `[x]` matches singleton list | No slice literal patterns in stable |
| Performance | O(n) with intermediate allocations | O(n) single pass possible |

## What Rust Learners Should Notice
- **Trait bounds are explicit**: Where OCaml uses structural equality by default, Rust requires `PartialEq` to compare elements. This makes the contract visible.
- **Clone is the cost of ownership**: OCaml freely copies values (GC handles it). Rust requires `Clone` when you want to put a value into an enum variant while still traversing the original slice.
- **No list pattern matching on slices**: OCaml's `[x]` and `x :: rest` patterns don't exist for Rust slices (stable). You use `split_first()`, `.len()`, or index-based patterns instead.
- **Enums are the same concept**: The translation from OCaml variant to Rust enum is nearly mechanical — add `#[derive]` for common traits, add trait bounds for generics.

## Further Reading
- [The Rust Book — Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [99 OCaml Problems](https://ocaml.org/problems)
- [Rust — Deriving Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
