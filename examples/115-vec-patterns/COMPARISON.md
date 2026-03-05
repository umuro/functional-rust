# OCaml vs Rust: Vec Operations Functionally

## Side-by-Side Code

### OCaml
```ocaml
let data = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
let sum =
  data
  |> List.filter (fun x -> x mod 2 = 0)
  |> List.map    (fun x -> x * 2)
  |> List.fold_left ( + ) 0
(* Each step produces a new intermediate list *)
```

### Rust (idiomatic — lazy iterator pipeline)
```rust
pub fn sum_of_doubled_evens(data: &[i32]) -> i32 {
    data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .sum()
}
// No intermediate Vec — elements flow through the pipeline one at a time
```

### Rust (functional/recursive — mirrors OCaml explicit recursion)
```rust
pub fn sum_of_doubled_evens_rec(data: &[i32]) -> i32 {
    match data {
        [] => 0,
        [x, rest @ ..] if x % 2 == 0 => x * 2 + sum_of_doubled_evens_rec(rest),
        [_, rest @ ..] => sum_of_doubled_evens_rec(rest),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| List type | `'a list` | `&[T]` (slice) |
| Map | `List.map : ('a -> 'b) -> 'a list -> 'b list` | `.map(f)` on `Iterator` |
| Filter | `List.filter : ('a -> bool) -> 'a list -> 'a list` | `.filter(pred)` on `Iterator` |
| Fold | `List.fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a` | `.fold(init, f)` on `Iterator` |
| Zip | `List.map2 : ('a -> 'b -> 'c) -> 'a list -> 'b list -> 'c list` | `.zip()` on `Iterator` |
| Partition | `List.partition : ('a -> bool) -> 'a list -> 'a list * 'a list` | `.partition(pred)` on `Iterator` |
| Flat-map | `List.concat_map : ('a -> 'b list) -> 'a list -> 'b list` | `.flat_map(f)` on `Iterator` |

## Key Insights

1. **Laziness**: OCaml's `List.map` is strict — it immediately allocates a new list for each step. Rust's `.map()` is lazy; elements are processed on demand, one at a time, with no intermediate `Vec` until `.collect()` is called.

2. **Allocation**: Chaining three OCaml list operations allocates three separate lists. The equivalent Rust iterator chain allocates exactly once (at `.collect()`) — or zero times if the terminal is `.sum()`, `.fold()`, or similar.

3. **Ownership and borrowing**: Rust's `iter()` yields `&T` references into the original slice, ensuring the source data isn't moved or copied. OCaml's garbage-collected lists share structure implicitly; Rust makes sharing explicit via lifetimes.

4. **Double-reference pattern**: `filter(|&&x| ...)` uses two `&` dereferences because `iter()` yields `&&i32` when the slice element is itself `i32` — a common Rust iterator idiom absent from OCaml.

5. **scan vs fold**: Rust's `.scan()` is an iterator adapter that yields every intermediate accumulator, making prefix sums trivial. OCaml requires a manual `fold_left` that threads a growing list through the accumulator.

## When to Use Each Style

**Use idiomatic Rust iterator chains when:** you want zero intermediate allocations, readable data-pipeline style code, and maximum performance — the common case for most transformations.

**Use recursive Rust when:** you need to mirror OCaml's structural recursion for pedagogical clarity, or when the problem decomposes naturally on the head/tail structure of a slice with slice patterns.
