📖 **[View on hightechmind.io →](https://hightechmind.io/rust/883-lazy-sequences)**

---

# 883-lazy-sequences — Lazy Sequences

## Problem Statement

Computing with infinite mathematical sequences — natural numbers, primes, Fibonacci numbers, powers — requires that elements be generated on demand rather than all at once. Lazy evaluation defers computation until results are needed, enabling programs to express "the first 100 primes" as concisely as "all primes." Haskell is lazy by default; OCaml uses `Seq` for explicit laziness. Rust's iterators are lazy by design: adapters like `.map()` and `.filter()` do nothing until consumed by `.collect()`, `.take()`, or `.fold()`. This makes Rust iterators ideal for modeling infinite mathematical sequences.

## Learning Outcomes

- Build infinite lazy iterators using ranges, `std::iter::successors`, and `from_fn`
- Combine lazy generators with `.filter()` for sieve-like constructions
- Use `.take_while()` and `.find()` to safely consume infinite iterators
- Implement `powers_of` and `triangle_numbers` as lazy generators
- Compare Rust's lazy iterators with OCaml's explicit `Seq` laziness

## Rust Application

`naturals()` returns `0u64..` (an infinite range). `squares()` maps `n*n` over naturals lazily — no allocation until consumed. `primes()` filters naturals with `is_prime` — a lazy sieve. `powers_of(base)` uses `std::iter::successors(Some(1), move |&prev| prev.checked_mul(base))`, stopping naturally on overflow. `triangle_numbers()` chains `successors` with `scan`. Consumer examples: `primes_below(100)` uses `.take_while(|&p| p < 100).collect()`, and `first_prime_over(1000)` uses `.find(|&p| p > 1000)`.

## OCaml Approach

OCaml `Seq` uses explicit thunking: `let naturals () = Seq.unfold (fun n -> Some(n, n+1)) 0`. Each element is a closure that evaluates the next on demand. `Seq.filter is_prime (naturals ())` creates a lazy primes sequence. `List.of_seq (Seq.take 10 primes_seq)` materializes the first 10. OCaml's `Seq` is more explicit about laziness (each `Cons` node is a `unit -> _` closure), while Rust's iterator laziness is baked into the trait design.

## Key Differences

1. **Implicit vs explicit laziness**: Rust iterators are always lazy (adapters don't evaluate); OCaml `Seq` makes laziness explicit via `unit ->` thunks.
2. **State representation**: Rust lazy state lives in the struct (e.g., `Fibonacci { a, b }`); OCaml `Seq.unfold` threads state functionally.
3. **Overflow safety**: `successors` with `checked_mul` gracefully terminates on overflow; OCaml requires explicit bounds checking.
4. **Composability**: Both compose lazy sequences using map/filter/take; Rust uses method chaining, OCaml uses `|>` pipe.

## Exercises

1. Implement a lazy `happy_numbers()` iterator that yields numbers whose digit-square-sum eventually reaches 1.
2. Write `prime_pairs()` that lazily yields twin primes `(p, p+2)` using `zip` and `filter` over the primes iterator.
3. Implement `memoized_fibonacci()` as a lazy iterator backed by a cache to avoid recomputing previous values.
