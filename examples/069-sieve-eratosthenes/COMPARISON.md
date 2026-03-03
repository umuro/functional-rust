# Sieve of Eratosthenes (Functional): OCaml vs Rust

## The Core Insight
The prime sieve beautifully illustrates the elegance-vs-efficiency tradeoff. OCaml's recursive filter version is a gorgeous three-liner that reads like mathematics. Rust can replicate it, but the idiomatic approach is a mutable boolean array that's orders of magnitude faster — and Rust makes that mutation safe.

## OCaml Approach
OCaml's `sieve` function is textbook elegance: take the first element as a prime, filter out its multiples, recurse on the rest. `List.filter (fun n -> n mod p <> 0) rest` does the heavy lifting. Each recursive call produces a new filtered list — the GC handles all the intermediate allocations. This isn't the true Sieve of Eratosthenes (it's trial division), but it captures the spirit beautifully.

## Rust Approach
Rust offers both styles. The functional version uses `iter().filter().copied().collect()` to mirror OCaml's approach, but each step allocates a new Vec. The imperative sieve uses `vec![true; n+1]` as a boolean array, marking composites in-place — O(n log log n) with minimal allocation. Rust's ownership system ensures the mutable array can't be accessed from multiple threads accidentally, making the imperative version both safe and fast.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Functional sieve | `p :: sieve (List.filter ...)` | `vec![p]; result.extend(sieve(...))` |
| Imperative sieve | Not idiomatic | `vec![true; n+1]` + mutation |
| Complexity | O(n × #primes) | O(n log log n) imperative |
| Memory | GC handles intermediate lists | Single boolean array |
| Style | Recursive filtering | Iterator or loop |

## What Rust Learners Should Notice
- The functional sieve works in Rust but allocates a new Vec per prime — expensive compared to OCaml's GC-optimized list allocation
- The imperative sieve in Rust is both idiomatic and performant — `vec![true; n]` is a single allocation
- `iter().filter().copied().collect()` is Rust's equivalent of OCaml's `List.filter` — note `copied()` to go from `&u64` to `u64`
- Rust lets you choose: functional elegance for small inputs, imperative efficiency for large ones — both are safe
- This is a case where OCaml's style is more natural; Rust's strength is making the efficient version equally safe

## Further Reading
- [Rust by Example — Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)
- [Sieve of Eratosthenes on Rosetta Code](https://rosettacode.org/wiki/Sieve_of_Eratosthenes#OCaml)
