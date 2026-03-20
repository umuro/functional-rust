[perfect-numbers on hightechmind.io](https://hightechmind.io/posts/functional-rust/perfect-numbers)

---

## Problem Statement

Classify integers as perfect, abundant, or deficient based on their aliquot sum (sum of proper divisors). A perfect number equals its aliquot sum (6 = 1+2+3), abundant exceeds it (12 > 1+2+3+4+6), and deficient falls short (8 < 1+2+4). Implement three variants of `sum_of_divisors`: brute-force filter, optimized square-root scan, and a `flat_map` divisor-pair version.

## Learning Outcomes

- Compute aliquot sum with `(1..n).filter(|&d| n.is_multiple_of(d)).sum()`
- Classify using `sum.cmp(&n)` matched against `Ordering::Equal/Greater/Less`
- Optimize to O(√n) by iterating `i` up to `sqrt(n)` and including both `i` and `n/i`
- Implement a `flat_map` version that generates divisor pairs `(i, n/i)` as an iterator
- Handle edge case: n=0 is Invalid; 1 has sum 0 (deficient)

## Rust Application

```rust
#[derive(Debug, PartialEq)]
pub enum Classification { Perfect, Abundant, Deficient, Invalid }

pub fn sum_of_divisors(n: u64) -> u64 {
    (1..n).filter(|&d| n.is_multiple_of(d)).sum()
}

pub fn classify(n: u64) -> Classification {
    if n == 0 { return Classification::Invalid; }
    let s = sum_of_divisors(n);
    match s.cmp(&n) {
        std::cmp::Ordering::Equal   => Classification::Perfect,
        std::cmp::Ordering::Greater => Classification::Abundant,
        std::cmp::Ordering::Less    => Classification::Deficient,
    }
}

// O(sqrt(n)) version
pub fn sum_of_divisors_fast(n: u64) -> u64 {
    if n <= 1 { return 0; }
    let mut sum = 1u64;
    let mut i = 2;
    while i * i <= n {
        if n.is_multiple_of(i) {
            sum += i;
            if i != n / i { sum += n / i; }
        }
        i += 1;
    }
    sum
}
```

`n.is_multiple_of(d)` is equivalent to `n % d == 0` but is more readable. The brute-force version runs `(1..n)` — O(n) — which is fine for small inputs. The fast version checks up to `√n`, collecting both `i` and `n/i` per divisor found; the guard `i != n/i` prevents double-counting perfect squares.

The classification uses `Ordering::cmp` — expressing three-way comparison cleanly without chained `if/else`.

## OCaml Approach

```ocaml
type classification = Perfect | Abundant | Deficient | Invalid

let sum_of_divisors n =
  if n <= 0 then 0
  else
    List.init (n - 1) (fun i -> i + 1)
    |> List.filter (fun d -> n mod d = 0)
    |> List.fold_left ( + ) 0

let classify n =
  if n = 0 then Invalid
  else
    let s = sum_of_divisors n in
    if s = n then Perfect
    else if s > n then Abundant
    else Deficient

(* Optimized *)
let sum_of_divisors_fast n =
  if n <= 1 then 0
  else
    let sum = ref 1 in
    let i = ref 2 in
    while !i * !i <= n do
      if n mod !i = 0 then begin
        sum := !sum + !i;
        if !i <> n / !i then sum := !sum + n / !i
      end;
      incr i
    done;
    !sum
```

OCaml's immutable `List` approach for the brute-force version mirrors the Rust iterator chain exactly. The optimized version uses OCaml's `ref`-based mutation (idiomatic for loops with mutable counters).

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Divisibility | `n.is_multiple_of(d)` | `n mod d = 0` |
| Three-way compare | `Ordering::Equal/Greater/Less` via `.cmp()` | Chained `if/else if/else` |
| Iterator range | `(1..n)` | `List.init (n-1) (fun i -> i+1)` |
| Mutable fast version | `while` loop with mutable `i` | `ref` counters in `while` |

Perfect numbers are rare: 6, 28, 496, 8128, 33550336. The O(√n) optimization is essential for classifying large numbers like 33,550,336 without waiting.

## Exercises

1. Implement `find_perfect_numbers(limit)` that returns all perfect numbers up to the limit.
2. Verify that all even perfect numbers follow the Euclid-Euler form `2^(p-1) * (2^p - 1)` for Mersenne prime `2^p - 1`.
3. Use the O(√n) version to classify all numbers from 1 to 10,000 and count how many are perfect, abundant, and deficient.
4. Implement `is_amicable(n)` — n and m are amicable if `sum_of_divisors(n) == m` and `sum_of_divisors(m) == n` (and n ≠ m).
5. Extend the flat_map divisor-pair version to also compute the product of all proper divisors.
