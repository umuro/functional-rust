# OCaml vs Rust: Benchmark Harness Pattern

## Basic Benchmark Structure

### Rust
```rust
pub fn bench<F, R>(name: &str, iterations: usize, mut f: F) -> Stats
where
    F: FnMut() -> R,
{
    let mut samples = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        black_box(f());
        samples.push(start.elapsed());
    }
    compute_stats(samples)
}
```

### OCaml (Core_bench)
```ocaml
let () =
  Command.run (Bench.make_command [
    Bench.Test.create ~name:"fib_recursive" (fun () ->
      ignore (fib_recursive 20));
    Bench.Test.create ~name:"fib_iterative" (fun () ->
      ignore (fib_iterative 20));
  ])
```

## Preventing Optimization

### Rust
```rust
use std::hint::black_box;

// Prevent compiler from optimizing away the result
black_box(f());
```

### OCaml
```ocaml
(* Use Sys.opaque_identity *)
ignore (Sys.opaque_identity (f ()))
```

## Computing Statistics

### Rust
```rust
pub struct Stats {
    pub mean: Duration,
    pub min: Duration,
    pub p50: Duration,
    pub p90: Duration,
    pub p99: Duration,
    pub max: Duration,
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Library | Core_bench | criterion, or std-only |
| Anti-optimization | `Sys.opaque_identity` | `std::hint::black_box` |
| Timer | `Unix.gettimeofday` | `Instant::now()` |
| Statistics | Built into Core_bench | Manual or criterion |
| Warmup | Automatic | Manual |
