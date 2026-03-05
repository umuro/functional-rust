# OCaml vs Rust: Parallel Iterators

## Parallel Map

### OCaml
```ocaml
let parallel_map f arr =
  let n = Array.length arr in
  let res = Array.make n (f arr.(0)) in
  let num_threads = 4 in
  let chunk = (n + num_threads - 1) / num_threads in
  let threads = Array.init num_threads (fun t ->
    let lo = t * chunk in
    let hi = min n ((t + 1) * chunk) in
    Thread.create (fun () ->
      for i = lo to hi - 1 do
        res.(i) <- f arr.(i)
      done
    ) ()
  ) in
  Array.iter Thread.join threads;
  res
```

### Rust (with rayon, this becomes trivial)
```rust
// With rayon crate:
use rayon::prelude::*;
let squares: Vec<_> = data.par_iter().map(|x| x * x).collect();

// Manual implementation:
fn parallel_map<T, U, F>(data: &[T], f: F) -> Vec<U>
where T: Sync, U: Send + Default + Clone, F: Fn(&T) -> U + Sync
{
    let chunk_size = data.len() / num_cpus;
    let mut results = vec![U::default(); data.len()];
    
    thread::scope(|s| {
        for (chunk_in, chunk_out) in 
            data.chunks(chunk_size).zip(results.chunks_mut(chunk_size)) 
        {
            s.spawn(|| {
                for (input, output) in chunk_in.iter().zip(chunk_out.iter_mut()) {
                    *output = f(input);
                }
            });
        }
    });
    results
}
```

## Key Differences

| Feature | OCaml | Rust (Rayon) |
|---------|-------|--------------|
| Syntax | Manual chunking | `.par_iter().map()` |
| Work distribution | Static chunks | Work-stealing |
| Thread management | Manual | Automatic thread pool |
| Composability | Low | High (chain operations) |

## Parallel Sum

### OCaml
```ocaml
let parallel_sum arr =
  let chunk = Array.length arr / 4 in
  let partials = Array.init 4 (fun t ->
    Thread.create (fun () ->
      let lo = t * chunk in
      let hi = if t = 3 then Array.length arr else (t+1) * chunk in
      let sum = ref 0.0 in
      for i = lo to hi - 1 do sum := !sum +. arr.(i) done;
      !sum
    ) ()
  ) in
  (* ... join and sum partials *)
```

### Rust
```rust
// With rayon:
let sum: f64 = data.par_iter().sum();

// Manual:
let partial_sums: Vec<f64> = thread::scope(|s| {
    data.chunks(chunk_size)
        .map(|c| s.spawn(move || c.iter().sum::<f64>()))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect()
});
partial_sums.iter().sum()
```

## Rayon's Power

```rust
// Complex pipeline — all parallel, work-stealing balanced
let result: Vec<_> = data
    .par_iter()
    .filter(|x| x.is_valid())
    .map(|x| expensive_transform(x))
    .filter_map(|x| x.optional_step())
    .collect();
```
