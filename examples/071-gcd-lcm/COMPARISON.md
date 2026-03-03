# GCD and LCM — OCaml vs Rust Comparison

## Core Insight

Euclid's algorithm is one of the oldest algorithms and maps perfectly to both languages. The recursive version is nearly character-for-character identical. The interesting differences emerge in the list version and overflow handling.

## OCaml Approach

`let rec gcd a b = if b = 0 then a else gcd b (a mod b)` — one line, crystal clear. The list version uses `List.fold_left gcd h t` with pattern matching to handle the empty list. OCaml's arbitrary-precision integers (via Zarith) can avoid overflow entirely.

## Rust Approach

`fn gcd(a: u64, b: u64) -> u64` — similarly concise. The list version uses `Iterator::reduce()` which returns `Option<T>` (handling the empty case). For LCM, we divide before multiplying (`a / gcd(a,b) * b`) to avoid intermediate overflow — a detail OCaml programmers often ignore due to big integers.

## Comparison Table

| Aspect        | OCaml                         | Rust                               |
|---------------|-------------------------------|-------------------------------------|
| **Memory**    | Stack frames (recursive)      | Stack frames or iterative           |
| **Null safety** | Pattern match on list      | `Option` from `reduce()`            |
| **Errors**    | `abs` handles negatives       | `u64` — no negatives to handle      |
| **Iteration** | `fold_left` on list           | `reduce()` on iterator              |
| **Overflow**  | Use Zarith for safety         | Must divide-before-multiply         |

## Things Rust Learners Should Notice

1. **`reduce()` vs `fold()`** — reduce uses the first element as init, returns `Option`; fold takes explicit init
2. **Overflow prevention** — `a / gcd(a,b) * b` avoids the `a * b` overflow that `lcm` could hit
3. **`u64` means no negatives** — simpler than OCaml's signed `int`, but requires type choice upfront
4. **Tail recursion not guaranteed** — Rust doesn't guarantee TCO; the iterative version is safer for huge inputs
5. **`.copied()`** — converts `&u64` to `u64` in the iterator chain (like OCaml's implicit value semantics)

## Further Reading

- [Euclidean algorithm (Wikipedia)](https://en.wikipedia.org/wiki/Euclidean_algorithm)
- [Iterator::reduce](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce)
