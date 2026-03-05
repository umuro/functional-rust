# OCaml vs Rust: Pipe Operator

## OCaml
```ocaml
(* Built-in pipe forward operator *)
let ( |> ) x f = f x

(* Usage *)
let result = 5 |> double |> add1 |> square  (* 121 *)

(* Multiple types *)
let result = 42 |> string_of_int |> fun s -> "Result: " ^ s
```

## Rust
```rust
pub trait Pipe: Sized {
    fn pipe<B, F: FnOnce(Self) -> B>(self, f: F) -> B { f(self) }
}
impl<T> Pipe for T {}

// Usage
let result = 5.pipe(double).pipe(add1).pipe(square);  // 121

// Type change
let result = 42.pipe(to_string).pipe(prefix);
```

## Key Differences

1. **OCaml**: Built-in `|>` operator
2. **Rust**: Extension trait method `.pipe()`
3. Both: Left-to-right data flow
4. **Rust**: Also provides pipe_ref and pipe_mut variants
5. Both enable point-free style programming
