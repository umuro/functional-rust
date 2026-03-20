📖 **[View on hightechmind.io →](https://hightechmind.io/rust/069-sieve-eratosthenes)**

---

# 069 — Sieve of Eratosthenes

## Problem Statement

The Sieve of Eratosthenes (circa 240 BC) is one of the oldest algorithms. Starting with all integers from 2 to n, repeatedly mark the multiples of each prime as composite. The unmarked numbers are prime. The naive functional version — recursively filter multiples — is elegant but O(n² / log n). The imperative boolean-array sieve is O(n log log n).

Primes are fundamental in cryptography (RSA, Diffie-Hellman), hash functions, and number theory. Understanding both the elegant functional version and the efficient imperative version illustrates the tension between expressiveness and performance.

## Learning Outcomes

- Implement the functional sieve: filter out multiples of each head recursively
- Implement the imperative sieve: boolean array marking composites
- Understand the O(n log log n) vs O(n² / log n) complexity difference
- Recognize the functional version as elegant but impractical for large n
- Use `std::iter::from_fn` or `successors` for lazy prime generation

## Rust Application

`sieve_functional` mirrors the OCaml version: if the candidate list is empty, return `[]`. Otherwise take the head `p` as a prime, filter out all multiples of `p` from the tail, and recurse. `primes_up_to_functional(n)` starts with `(2..=n).collect()`. `primes_up_to` (imperative) uses a `Vec<bool>` marking composites, starting from 2 and marking `2p, 3p, ...` as composite.

## OCaml Approach

OCaml's functional sieve: `let rec sieve = function | [] -> [] | p :: rest -> p :: sieve (List.filter (fun n -> n mod p <> 0) rest)`. Start with `List.init (n - 1) (fun i -> i + 2)` to get `[2; 3; ...; n]`. Each recursive call filters one more prime's multiples. The elegance is striking; the performance for large n is not.

## Key Differences

1. **Functional vs imperative**: The functional sieve directly encodes the algorithm description. The imperative sieve is 3-5x faster due to array access patterns. Both are O(n log log n) amortized? No — the functional version is O(n√n / ln n) due to repeated filtering.
2. **Mutable boolean array**: The imperative Rust version uses `Vec<bool>` with in-place marking. OCaml's functional version creates new filtered lists at each step.
3. **`retain` vs `filter`**: Rust's `Vec::retain(|x| x % p != 0)` modifies in place. `filter` creates a new Vec. For the functional sieve, `filter` is cleaner; for performance, `retain` or the boolean array is better.
4. **Lazy primes**: Rust's lazy version using `(2..).filter(|&n| is_prime(n))` computes primality per element — trial division, O(√n) per prime check.

## Exercises

1. **Segmented sieve**: For large n (up to 10^9), implement a segmented sieve that processes the range in cache-sized blocks. This avoids allocating a Vec<bool> of size n.
2. **Prime factorization**: Using the sieve output, write `factorize(n: u64, primes: &[u64]) -> Vec<u64>` that returns the prime factorization of n by trial division with known primes.
3. **Prime gaps**: Write `prime_gaps(n: u64) -> Vec<u64>` that returns the gaps between consecutive primes up to n. Find the first prime gap larger than 100.
