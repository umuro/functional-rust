# OCaml vs Rust: Benchmark Closures

## OCaml
```ocaml
let bench name iterations f =
  let start = Unix.gettimeofday () in
  for _ = 1 to iterations do
    ignore (f ())
  done;
  let elapsed = Unix.gettimeofday () -. start in
  Printf.printf "%s: %f sec\n" name elapsed

let () = bench "sum" 1000 (fun () -> List.fold_left (+) 0 [1;2;3])
```

## Rust
```rust
use std::hint::black_box;

pub fn bench<T, F: FnMut() -> T>(name: &str, iters: usize, mut f: F) {
    let start = Instant::now();
    for _ in 0..iters {
        black_box(f());  // prevent optimization
    }
    println!("{}: {:?}", name, start.elapsed());
}
```

## Key Differences

1. **Rust**: `black_box` prevents compiler from optimizing away results
2. **OCaml**: `ignore` discards result but doesn't prevent optimization
3. Both: Closures enable flexible benchmark setup
4. **Rust**: Setup/teardown closures for stateful benchmarks
5. **Rust**: Criterion crate for production benchmarking
