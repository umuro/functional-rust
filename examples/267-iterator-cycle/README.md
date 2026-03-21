📖 **[View on hightechmind.io →](https://hightechmind.io/rust/267-iterator-cycle)**

---

# 267: Infinite Cycling with cycle()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Repeating a pattern indefinitely is needed in round-robin scheduling, alternating colors in UI tables, repeating short sequences to fill longer ones, or generating periodic signals. The `cycle()` adapter creates an infinite iterator by cloning the source iterator each time it is exhausted. Combined with `take()` or `zip()`, it produces exactly as many repetitions as needed without manually computing modular indices.

## Learning Outcomes

- Understand that `cycle()` repeats a finite iterator infinitely by cloning it at exhaustion
- Combine `cycle()` with `take(n)` to consume exactly `n` elements from the cycled source
- Use `cycle()` with `zip()` to assign repeating labels or colors to a longer sequence
- Recognize that `cycle()` requires the source iterator to implement `Clone`

## Rust Application

`Iterator::cycle()` requires `Clone` on the iterator itself (not just the elements). It calls `clone()` each time the inner iterator is exhausted and restarts it:

```rust
// Repeat [1,2,3] indefinitely, take first 7
let result: Vec<i32> = [1, 2, 3].iter().copied().cycle().take(7).collect();
// [1, 2, 3, 1, 2, 3, 1]

// Round-robin labeling: assign alternating "a"/"b" labels to longer sequence
let items = [1i32, 2, 3, 4];
let labels = ["a", "b"];
let labeled: Vec<_> = items.iter().zip(labels.iter().cycle()).collect();
// [(1,"a"), (2,"b"), (3,"a"), (4,"b")]
```

## OCaml Approach

OCaml's `Seq` module can model cycling with a recursive lazy function:

```ocaml
let rec cycle xs () = match Seq.uncons xs with
  | None -> cycle xs ()  (* restart when exhausted *)
  | Some (x, rest) -> Seq.Cons (x, cycle rest)
```

For lists, cycling is typically handled with modular index arithmetic: `lst.(i mod List.length lst)`.

## Key Differences

1. **Clone requirement**: Rust's `cycle()` requires `Clone` on the iterator — slices' iterators are `Clone`, but consuming iterators typically are not.
2. **Infinite by design**: The result iterator never terminates; always pair with `take()`, `zip()`, or `take_while()` to bound it.
3. **Lazy evaluation**: Both Rust's `cycle()` and OCaml's lazy sequence approach avoid materializing the repeated sequence; only the original source is stored.
4. **Real-world uses**: CSS `:nth-child` patterns, round-robin load balancing, repeating background tiles, periodic waveform generation.

## Exercises

1. Implement a round-robin scheduler that distributes tasks among N workers using `cycle()` and `zip()`.
2. Generate the alternating sequence `[true, false, true, false, ...]` for 20 elements using `[true, false].iter().cycle()`.
3. Use `cycle()` to pad a shorter slice to match the length of a longer one by repeating the shorter slice's elements.
