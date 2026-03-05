# OCaml vs Rust: Join (Fork-Join Parallelism)

## Basic Join Pattern

### OCaml
```ocaml
let join f g =
  let result_g = ref None in
  let thread = Thread.create (fun () ->
    result_g := Some (g ())
  ) () in
  let result_f = f () in
  Thread.join thread;
  (result_f, Option.get !result_g)
```

### Rust
```rust
fn join<A, B>(f: impl FnOnce() -> A + Send, 
              g: impl FnOnce() -> B + Send) -> (A, B)
{
    let handle = thread::spawn(f);
    let b = g();
    let a = handle.join().unwrap();
    (a, b)
}
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Return value | Via `ref` cell | Direct from `join()` |
| Type safety | `Option.get` can fail | Compile-time guaranteed |
| Thread spawning | `Thread.create f ()` | `thread::spawn(closure)` |
| Result extraction | Manual unwrap | Built into handle |

## Parallel Sum Example

### OCaml
```ocaml
let rec psum arr lo hi =
  if hi - lo <= 500 then
    Array.fold_left (+) 0 (Array.sub arr lo (hi - lo))
  else
    let mid = (lo + hi) / 2 in
    let (l, r) = join
      (fun () -> psum arr lo mid)
      (fun () -> psum arr mid hi)
    in l + r
```

### Rust
```rust
fn parallel_sum(data: &[i64]) -> i64 {
    if data.len() <= 1000 {
        return data.iter().sum();
    }
    
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    
    let (sum_l, sum_r) = scoped_join(
        || parallel_sum(left),
        || parallel_sum(right),
    );
    
    sum_l + sum_r
}
```

## Parallel Merge Sort

### Rust
```rust
fn parallel_sort(mut v: Vec<i64>) -> Vec<i64> {
    if v.len() <= 512 {
        v.sort();
        return v;
    }
    
    let right = v.split_off(v.len() / 2);
    let left = v;
    
    let (sorted_l, sorted_r) = join(
        move || parallel_sort(left),
        move || parallel_sort(right),
    );
    
    merge(sorted_l, sorted_r)
}
```

## With Rayon (One-liner)

```rust
use rayon::prelude::*;

// Rayon's join handles work-stealing automatically
rayon::join(
    || expensive_computation_a(),
    || expensive_computation_b(),
);
```
