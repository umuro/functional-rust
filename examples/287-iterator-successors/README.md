📖 **[View on hightechmind.io →](https://hightechmind.io/rust/287-iterator-successors)**

---

# 287: Recursive Sequences with successors()

## Problem Statement

Many mathematical and algorithmic sequences are defined recursively: each element is a function of its predecessor. Powers of 2 (`1, 2, 4, 8, ...`), the Collatz sequence, convergence sequences in numerical methods, and tree/graph traversal via repeated `next_node(current)` calls all share this structure. `std::iter::successors()` formalizes this pattern: given a first value and a function `f(current) -> Option<next>`, it generates the entire infinite (or finite) sequence.

## Learning Outcomes

- Understand `successors(first, f)` as generating `first, f(first), f(f(first)), ...` until `f` returns `None`
- Use `successors` for power sequences, Collatz sequences, and convergence series
- Recognize `successors` as `unfold` specialized to single-value state
- Combine `successors` with `take()`, `take_while()`, and `sum()` to bound and aggregate sequences

## Rust Application

`std::iter::successors(Some(start), |current| next_from_current)` generates a sequence. When the closure returns `None`, the iterator terminates:

```rust
// Powers of 2 up to max
pub fn powers_of_2(max: u32) -> impl Iterator<Item = u32> {
    std::iter::successors(Some(1u32), move |&n| {
        if n < max { Some(n * 2) } else { None }
    })
}

// Collatz sequence from start to 1
pub fn collatz(start: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(start), |&n| {
        if n == 1 { None }
        else if n % 2 == 0 { Some(n / 2) }
        else { Some(3 * n + 1) }
    })
}
```

## OCaml Approach

OCaml's `Seq.unfold` is the equivalent — it threads a state through a generator function. For single-value state (where state equals current value), it directly mirrors `successors`:

```ocaml
let powers_of_2 max =
  Seq.unfold (fun n -> if n > max then None else Some (n, n * 2)) 1

let collatz start =
  Seq.unfold (fun n ->
    if n = 0 then None
    else Some (n, if n = 1 then 0  (* sentinel stop *)
                 else if n mod 2 = 0 then n/2 else 3*n+1)
  ) start
```

## Key Differences

1. **Naming**: Rust calls it `successors`; Haskell calls it `iterate` (infinite) or `unfoldr`; OCaml uses `Seq.unfold`.
2. **First value**: Rust's `successors` takes `Option<T>` as the first argument — `None` produces an empty iterator immediately.
3. **State vs output**: `successors` uses the same type for state and output (current element is the state); `from_fn` separates them.
4. **Convergence**: Used in numerical methods (Newton's method iterations), parsing (advancing through AST nodes), and graph traversal (BFS/DFS via successive frontier sets).

## Exercises

1. Use `successors` to implement Newton's method for square root: starting from an initial guess, iterate `x -> (x + n/x) / 2` until convergence.
2. Generate the binary representations of all powers of 2 up to 2^20 using `successors(Some(1u32), |n| n.checked_mul(2))`.
3. Use `successors` to traverse a linked structure: given a node type with an optional `next` pointer, traverse the entire list.
