# Direct Run-Length Encoding: OCaml vs Rust

## The Core Insight
Direct RLE eliminates the intermediate step of grouping — it counts runs on the fly. This requires carrying mutable state (the current count) through traversal. OCaml does this with recursive accumulators; Rust offers both fold-based and imperative approaches, with the fold version being particularly interesting because it mutates the accumulator in place.

## OCaml Approach
The recursive version carries a `count` alongside the accumulator:
```ocaml
let rec aux count acc = function
  | [] -> List.rev acc
  | [x] -> List.rev ((if count = 0 then One x else Many (count+1, x)) :: acc)
  | x :: (y :: _ as rest) ->
    if x = y then aux (count+1) acc rest
    else let item = if count = 0 then One x else Many (count+1, x) in
         aux 0 (item :: acc) rest
```
The fold version builds the result in reverse, checking the head of the accumulator to extend the current run.

## Rust Approach
Rust's fold approach modifies the last element of the accumulator in place:
```rust
list.iter().fold(Vec::new(), |mut acc, x| {
    match acc.last_mut() {
        Some(RleItem::Many(n, ref y)) if y == x => { *n += 1; }
        Some(RleItem::One(ref y)) if y == x => {
            *acc.last_mut().unwrap() = RleItem::Many(2, y.clone());
        }
        _ => acc.push(RleItem::One(x.clone())),
    }
    acc
})
```
The `last_mut()` method gives a mutable reference to the last element — a pattern impossible in pure functional OCaml but natural in Rust.

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| State threading | Explicit parameters (`count`, `acc`) | `fold` with mutable accumulator |
| Mutation | Never (new list nodes) | In-place via `last_mut()` |
| Peeking ahead | `x :: (y :: _ as rest)` pattern | `tail.first()` or index comparison |
| Reverse at end | `List.rev acc` (common pattern) | Not needed (push appends) |
| Two-element look-ahead | Native with pattern matching | Requires explicit index logic |

## What Rust Learners Should Notice
- **`last_mut()` enables accumulator mutation**: Rust's ownership system lets you mutate the last element of a Vec through a mutable reference — something OCaml can't do with immutable lists.
- **OCaml's two-element pattern is cleaner**: `x :: (y :: _ as rest)` naturally looks at two elements. Rust requires comparing `list[i]` with `list[i-1]` or using `windows(2)`.
- **Both approaches are O(n)**: The algorithmic complexity is identical. The difference is stylistic — OCaml's recursive version is more declarative, Rust's fold version is more imperative.
- **Direct encoding avoids allocation**: Both languages benefit from not creating intermediate sublists, making this more memory-efficient than the two-step approach.

## Further Reading
- [99 OCaml Problems #13](https://ocaml.org/problems)
- [Rust — Iterator::fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
- [Rust — slice::last_mut](https://doc.rust-lang.org/std/primitive.slice.html#method.last_mut)
