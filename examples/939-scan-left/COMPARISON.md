# Scan Left — OCaml vs Rust Comparison

## Core Insight

Scan is fold's cousin that keeps all intermediate states. OCaml builds it on top of `fold_left` with accumulator tuple `(state, results)`. Rust has `Iterator::scan()` built-in but with a different API: it uses mutable state (`FnMut`) for performance rather than returning new state.

## OCaml Approach

Implements `scan_left` using `List.fold_left` with a pair accumulator `(acc, results_list)`. Each step produces a new pair — pure functional, no mutation. The result list is built in reverse and reversed at the end. Partial application makes `running_sum = scan_left (+) 0` beautifully concise.

## Rust Approach

Custom `scan_left` function uses `&[T]` slice input and `Vec<A>` output. The built-in `Iterator::scan()` takes a mutable state reference — closures modify state in place via `*state += x`. This is more memory-efficient but less purely functional. Both approaches are available.

## Comparison Table

| Aspect        | OCaml                           | Rust                                 |
|---------------|---------------------------------|--------------------------------------|
| **Memory**    | Reverse + rev (2x list)         | Single Vec with push                 |
| **Null safety** | N/A                          | N/A                                  |
| **Errors**    | N/A                            | N/A                                  |
| **Iteration** | `fold_left` with pair acc       | `for` loop or `Iterator::scan()`     |
| **Mutation**  | None (purely functional)        | `scan()` uses `FnMut` (mutable state)|

## Things Rust Learners Should Notice

1. **`Iterator::scan()`** is built-in but uses mutable state — different philosophy from OCaml
2. **`A: Clone` bound** — needed to store intermediate values; OCaml's GC handles this implicitly
3. **Slices `&[T]`** — Rust's way of borrowing a view into a collection without owning it
4. **`vec![init.clone()]`** — initial value must be cloned since we also keep it as running state

## Further Reading

- [Iterator::scan](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan)
- [Prefix sums (Wikipedia)](https://en.wikipedia.org/wiki/Prefix_sum)
