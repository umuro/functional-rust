# OCaml vs Rust: Tap Pattern

## OCaml
```ocaml
let tap f x = f x; x

(* Usage in pipeline *)
let result =
  [1; 2; 3]
  |> List.map (fun x -> x * 2)
  |> tap (fun xs -> Printf.printf "doubled: %d items\n" (List.length xs))
  |> List.filter (fun x -> x > 2)
```

## Rust
```rust
pub trait Tap: Sized {
    fn tap(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }
}

impl<T> Tap for T {}

// Usage
let result = vec![1, 2, 3]
    .into_iter()
    .map(|x| x * 2)
    .collect::<Vec<_>>()
    .tap(|v| println!("doubled: {} items", v.len()));
```

## Key Differences

1. **OCaml**: Simple function `let tap f x = f x; x`
2. **Rust**: Extension trait for method chaining
3. Both: Inspect values without breaking data flow
4. **Rust**: Different variants (tap, tap_mut, tap_dbg)
5. Both useful for debugging pipelines
