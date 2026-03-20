📖 **[View on hightechmind.io →](https://hightechmind.io/rust/530-closure-benchmark-idioms)**

---

# Closures in Benchmarking

## Problem Statement

Micro-benchmarking is notoriously tricky: compilers optimize aggressively, CPUs speculate and prefetch, and without careful technique, what you measure may not reflect what actually runs in production. Closures are central to benchmarking APIs because they encapsulate the code under test while providing the benchmark harness control over setup, teardown, and iteration. `std::hint::black_box` prevents the optimizer from eliminating computations whose results are discarded. This example shows how to build closure-based benchmarking utilities similar to `criterion`.

## Learning Outcomes

- How `FnMut() -> T` encapsulates the code under test in a reusable way
- Why `std::hint::black_box` is essential for preventing dead code elimination
- How warmup iterations reduce JIT and cache effects in micro-benchmarks
- How `bench_compare` takes two `FnMut` closures and reports their relative performance
- How setup and teardown closures enable fair measurement of just the code under test

## Rust Application

`bench<T, F: FnMut() -> T>(name, iterations, f)` runs warmup (10% of iterations), then times the main loop, calling `black_box(f())` to prevent optimization. `bench_compare` calls `bench` on two closures and prints their ratio. `consume<T>(value: T) -> T` is an alias for `black_box`. `bench_with_setup<S, T, Setup, Test, Teardown>` accepts three closures: one to create test data, one to run the test, and one to clean up — measuring only the test phase.

Key patterns:
- `black_box(f())` — opaque barrier preventing optimization of the benchmark body
- Warmup loop before timing: `for _ in 0..iterations/10 { black_box(f()); }`
- Generic `FnMut` — supports both closures and named functions as benchmarks

## OCaml Approach

OCaml benchmarking uses the `Core_bench` or `Bechamel` library. Benchmarks are registered as closures:

```ocaml
let bench name f =
  let t0 = Unix.gettimeofday () in
  for _ = 1 to 1000 do ignore (f ()) done;
  let t1 = Unix.gettimeofday () in
  Printf.printf "%s: %.2fus\n" name ((t1 -. t0) /. 1000.0 *. 1e6)
```

OCaml lacks a `black_box` equivalent in stdlib — `ignore` or `Sys.opaque_identity` is used instead.

## Key Differences

1. **Dead code prevention**: Rust has `std::hint::black_box` as a stable stdlib function; OCaml uses `Sys.opaque_identity` (internal, not guaranteed stable) or `ignore` which the optimizer may eliminate.
2. **Timing granularity**: Rust's `std::time::Instant` has nanosecond resolution; OCaml's `Unix.gettimeofday` has microsecond resolution on most platforms.
3. **Generic test body**: Rust's `FnMut() -> T` allows the benchmark to observe the return value via `black_box`; OCaml's `unit -> unit` discards the result, potentially allowing optimization.
4. **Library ecosystem**: Rust has `criterion` (statistics-based) and `divan` as mature benchmark frameworks; OCaml has `Core_bench` from Jane Street and `Bechamel`.

## Exercises

1. **Statistics bench**: Extend `bench` to return not just total duration but also mean, standard deviation, and min/max per iteration over multiple runs.
2. **Comparison macro**: Write a macro `bench_vs!(name1 => expr1, name2 => expr2, iterations: N)` that benchmarks two expressions and asserts the ratio is within a given tolerance.
3. **Setup benchmark**: Use `bench_with_setup` to benchmark `Vec::sort` vs `Vec::sort_unstable` on random data — ensure the setup closure generates a new random vector each iteration so sorting is never pre-sorted.
