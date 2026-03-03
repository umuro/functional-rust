# Duplicate Elements: OCaml vs Rust

## The Core Insight
Duplicating elements is a one-to-many transformation: each input produces exactly two outputs. It's a clean exercise in list construction — OCaml uses cons (`::`) to prepend two copies, while Rust uses `flat_map` or push-based iteration. The simplicity makes ownership differences especially visible.

## OCaml Approach
OCaml's recursive solution is elegantly minimal:
```ocaml
let rec duplicate = function
  | [] -> []
  | h :: t -> h :: h :: duplicate t
```
Two cons operations prepend two copies of the head. The tail-recursive version reverses at the end:
```ocaml
let duplicate_tr lst =
  let rec aux acc = function
    | [] -> List.rev acc
    | h :: t -> aux (h :: h :: acc) t
  in aux [] lst
```

## Rust Approach
Rust's iterator approach uses `flat_map` for the one-to-many expansion:
```rust
pub fn duplicate<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter().flat_map(|x| vec![x.clone(), x.clone()]).collect()
}
```
The imperative version with `push` is more efficient (avoids intermediate vec allocation):
```rust
for item in list {
    result.push(item.clone());
    result.push(item.clone());
}
```

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| Construction | `h :: h :: tail` (O(1) prepend) | `push` (O(1) amortized append) |
| Copying | Automatic (values shared) | Explicit `clone()` |
| Direction | Builds front-to-back (cons) | Builds back via push |
| Memory | New list nodes (GC) | Pre-allocatable Vec |
| One-to-many | `concat_map (fun x -> [x;x])` | `flat_map(\|x\| vec![x,x])` |

## What Rust Learners Should Notice
- **`with_capacity` for known sizes**: When you know the output will be exactly `2 * input.len()`, pre-allocate with `Vec::with_capacity`. OCaml can't do this with linked lists.
- **`flat_map` creates temporary Vecs**: `flat_map(|x| vec![x, x])` allocates a small Vec per element. The `push`-based version avoids this overhead entirely.
- **Clone is the explicit cost**: In OCaml, `h :: h :: t` shares the same value. In Rust, `Clone` creates independent copies. For `Copy` types (integers, etc.), this is zero-cost.
- **Simple problems reveal language philosophy**: OCaml optimizes for expressiveness (one line!). Rust lets you choose between expressiveness (`flat_map`) and performance (`push` with pre-allocation).

## Further Reading
- [99 OCaml Problems #14](https://ocaml.org/problems)
- [Rust — Iterator::flat_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)
- [Rust — Vec::with_capacity](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity)
