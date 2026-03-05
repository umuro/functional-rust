# Matrix Operations — OCaml vs Rust Comparison

## Core Insight

Both languages represent matrices as nested lists/vectors. OCaml's `List.map` and `List.fold_left2` provide elegant functional matrix operations. Rust's `iter().zip().map().sum()` is equally clean but operates on contiguous memory rather than linked lists — a significant performance advantage for numeric work.

## OCaml Approach

Matrices are `int list list`. Transpose uses `List.init` with `List.nth` (O(n) per access — not ideal for large matrices). Dot product uses `List.fold_left2` which zips and folds simultaneously. Clean and readable but O(n²) memory access patterns due to linked lists.

## Rust Approach

Matrices are `Vec<Vec<i64>>`. Transpose uses range-based column iteration. Dot product uses `iter().zip().map().sum()` — a zero-allocation iterator chain. All data is contiguous in memory, making this cache-friendly. Borrows (`&Matrix`) ensure the input isn't consumed.

## Comparison Table

| Aspect        | OCaml                          | Rust                                 |
|---------------|--------------------------------|--------------------------------------|
| **Memory**    | Linked lists (scattered)       | Vec<Vec> (contiguous rows)           |
| **Null safety** | N/A                         | N/A                                  |
| **Errors**    | Index out of bounds            | Panic on OOB (or use `.get()`)       |
| **Iteration** | `List.map` + `List.nth`       | `iter().zip().map().sum()`           |
| **Performance**| O(n) per element access       | O(1) per element access              |

## Things Rust Learners Should Notice

1. **`&Matrix` borrows** — functions take references, don't consume the input
2. **`zip()` + `map()` + `sum()`** — the idiomatic dot product pattern, zero allocation
3. **`collect::<Vec<_>>()`** — type annotation drives the output collection type
4. **Cache locality** — `Vec<Vec<i64>>` keeps each row contiguous; huge perf win over linked lists
5. **Type alias** — `type Matrix = Vec<Vec<i64>>` keeps signatures readable

## Further Reading

- [Iterator::zip](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
- [ndarray crate](https://docs.rs/ndarray/) — for serious matrix work in Rust
