# OCaml vs Rust: Async Closures

## OCaml (Lwt)
```ocaml
open Lwt.Infix

(* Async closure pattern *)
let async_map f items =
  Lwt_list.map_p f items

let double x = Lwt.return (x * 2)
let _ = async_map double [1; 2; 3]
```

## Rust
```rust
// Closure returning a future (stable pattern)
let double = |x: i32| async move { x * 2 };

// Async map
pub async fn async_map<T, U, F, Fut>(items: Vec<T>, f: F) -> Vec<U>
where F: Fn(T) -> Fut, Fut: Future<Output = U> {
    let mut results = Vec::new();
    for item in items { results.push(f(item).await); }
    results
}
```

## Key Differences

1. **OCaml**: Lwt/Async libraries provide async primitives
2. **Rust**: async/await is built into the language
3. **Rust**: `|x| async move { }` pattern for async closures
4. **Rust**: True async closures (`async |x| {}`) are nightly-only
5. Both: Closures can return async computations
