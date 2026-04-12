📖 **[View on hightechmind.io →](https://hightechmind.io/rust/069-sieve-eratosthenes)**

---

# 069 — Sieve of Eratosthenes
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

The Sieve of Eratosthenes (circa 240 BC) is one of the oldest known algorithms, described by the Greek mathematician Eratosthenes of Cyrene. The algorithm starts with all integers from 2 to n and repeatedly marks the multiples of each prime as composite. The unmarked numbers that remain are exactly the primes. This produces all primes up to n in a single pass.

Primes are foundational in modern computing: RSA encryption relies on the difficulty of factoring large primes, Diffie-Hellman key exchange works modulo prime numbers, hash table sizes are often chosen as primes to minimize collisions, and pseudo-random number generators use prime moduli. Generating primes efficiently is therefore not merely an academic exercise.

The naive functional version — recursively filter multiples — is elegant and short but runs in O(n² / log n) time. The imperative boolean-array sieve achieves O(n log log n), which grows extremely slowly. This example shows both and explains when each is appropriate.

## Learning Outcomes

- Implement the functional sieve: filter out multiples of each head element recursively
- Implement the imperative sieve: a boolean array that marks composites in-place
- Understand the O(n log log n) vs O(n² / log n) complexity difference and what drives it
- Recognize the functional version as elegant but impractical for large n due to repeated filtering
- Use `std::iter::from_fn` or `successors` for lazy prime generation without upfront allocation
- Understand why the sieve is much faster than trial division for generating all primes up to n

## Rust Application

`sieve_functional` mirrors the OCaml version using slice pattern matching:
- `[]` returns an empty vector — base case, no candidates remain
- `[p, ..]` takes the first element as the next prime, filters all multiples of `p` from the tail, and recurses

`primes_up_to_functional(n)` seeds it with `(2..=n).collect()`. This version is correct but slow — each level of recursion allocates a new filtered `Vec`.

`primes_up_to` (imperative) allocates `Vec<bool>` of size `n+1`, initializes all to `true`, then for each prime `p` starting at 2, marks `2p, 3p, 4p, ...` as `false`. It only needs to check up to `√n` for the outer loop. The result collects all indices still marked `true`.

## OCaml Approach

OCaml's functional sieve uses list pattern matching directly:

```
let rec sieve = function
  | [] -> []
  | p :: rest -> p :: sieve (List.filter (fun n -> n mod p <> 0) rest)
```

The seed list `List.init (n - 1) (fun i -> i + 2)` gives `[2; 3; ...; n]`. Each recursive call applies one more filter. OCaml has no built-in sieve in its standard library, so this manual implementation is standard in functional programming courses.

## Key Differences

1. **Functional vs imperative complexity**: The functional sieve is O(n² / log n) because each element is checked against every prime up to the square root. The boolean-array sieve is O(n log log n) because each composite is marked at most once per prime factor.
2. **Memory allocation**: The functional version allocates a new `Vec` at each recursion level. The imperative version allocates exactly one boolean array and modifies it in place. At n = 10,000, the functional version performs thousands of allocations.
3. **`retain` vs `filter`**: Rust's `Vec::retain(|x| x % p != 0)` mutates in place. `filter` creates a new `Vec`. For the functional sieve style, `filter` is semantically cleaner; for the imperative style, in-place mutation is preferred.
4. **Laziness**: Rust's `(2..).filter(|&n| is_prime(n))` with trial division is a lazy infinite iterator. Both the functional and imperative sieves are eager. Lazy sieves require more complex state management.
5. **Stack overflow**: The functional version recurses once per prime up to n. For large n, Rust's default stack size will overflow before OCaml (which uses continuations). The imperative version is iterative and stack-safe.

## Exercises

1. **Segmented sieve**: For large n (up to 10^9), a single boolean array is impractical. Implement a segmented sieve that computes primes in cache-sized blocks (e.g., 2^20 elements), using precomputed small primes to mark composites in each segment.
2. **Prime factorization**: Given the primes from the sieve, write `factorize(n: u64, primes: &[u64]) -> Vec<u64>` that returns the prime factorization of n by trial division using the known primes as candidates. This is much faster than naive trial division.
3. **Goldbach verification**: Goldbach's conjecture states every even integer > 2 is the sum of two primes. Write a function that verifies this for all even numbers up to n using the sieve output, finding the prime pair for each.
