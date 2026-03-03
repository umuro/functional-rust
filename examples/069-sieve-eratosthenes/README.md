# Example 069: Sieve of Eratosthenes (Functional)

**Difficulty:** ⭐⭐
**Category:** Algorithms
**Concept:** A purely functional prime sieve using recursive filtering. Each prime eliminates its multiples from the remaining candidates. Shows the contrast between elegant functional style and efficient imperative implementation.
**OCaml → Rust insight:** OCaml's recursive filter sieve is concise but O(n × primes); Rust naturally gravitates toward the boolean-array sieve (O(n log log n)) because mutable arrays are zero-cost.
