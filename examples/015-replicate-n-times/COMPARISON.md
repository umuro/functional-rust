# Replicate Elements N Times: OCaml vs Rust

## The Core Insight
Replication is the general case of duplication — produce n copies of each element. It's a clean demonstration of repetition combinators: OCaml's `List.init` and Rust's `std::iter::repeat().take()`. The problem also highlights pre-allocation in Rust: since you know the output size exactly (`len * n`), you can avoid all reallocation.

## OCaml Approach
A recursive helper generates n copies, then the main function maps over the list:
```ocaml
let replicate lst n =
  let rec repeat x = function
    | 0 -> []
    | k -> x :: repeat x (k - 1)
  in List.concat_map (fun x -> repeat x n) lst
```
Or more concisely with `List.init`:
```ocaml
let replicate lst n = List.concat_map (fun x -> List.init n (fun _ -> x)) lst
```

## Rust Approach
`std::iter::repeat` combined with `take` and `flat_map` is elegant:
```rust
pub fn replicate<T: Clone>(list: &[T], n: usize) -> Vec<T> {
    list.iter()
        .flat_map(|x| std::iter::repeat(x.clone()).take(n))
        .collect()
}
```
The pre-allocated imperative version is optimal for performance:
```rust
let mut result = Vec::with_capacity(list.len() * n);
for item in list { for _ in 0..n { result.push(item.clone()); } }
```

## Key Differences
| Aspect | OCaml | Rust |
|--------|-------|------|
| Repeat combinator | `List.init n (fun _ -> x)` | `repeat(x).take(n)` |
| Expansion | `concat_map` | `flat_map` |
| Memory | Intermediate lists (GC) | Lazy iterator (zero-cost) |
| Pre-allocation | Not possible (linked list) | `Vec::with_capacity(len * n)` |
| n = 0 | Returns `[]` naturally | Returns empty Vec |

## What Rust Learners Should Notice
- **`std::iter::repeat` is lazy**: Unlike `vec![x; n]` which allocates immediately, `repeat(x).take(n)` yields elements one at a time. Inside `flat_map`, this means no intermediate allocation.
- **Pre-allocation matters**: `Vec::with_capacity(list.len() * n)` allocates once. Without it, the Vec may reallocate multiple times as it grows. OCaml lists can't pre-allocate.
- **Clone cost scales with n**: Each element is cloned n times. For expensive-to-clone types, consider `Rc<T>` to share instead of copy.
- **Edge case: n = 0**: All implementations naturally return empty for n=0 — `take(0)` yields nothing, and the recursive base case `0 -> []` handles it.

## Further Reading
- [Rust — std::iter::repeat](https://doc.rust-lang.org/std/iter/fn.repeat.html)
- [99 OCaml Problems #15](https://ocaml.org/problems)
- [Rust — Vec::with_capacity](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity)
