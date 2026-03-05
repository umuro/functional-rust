# OCaml vs Rust: stringify! and concat!

## Rust stringify!

```rust
let name = stringify!(my_variable);  // "my_variable"
let expr = stringify!(x + y * z);    // "x + y * z"
```

## Rust concat!

```rust
let s = concat!("hello", " ", "world");  // "hello world"
let path = concat!(env!("HOME"), "/.config");
```

## OCaml Equivalent

```ocaml
(* No direct equivalent *)
(* ppx can generate strings from AST *)
let name = [%string_of_expr my_variable]  (* hypothetical ppx *)
```

## 5 Takeaways

1. **`stringify!` turns tokens into string literals.**
2. **`concat!` joins strings at compile time.**
3. **`file!()` and `line!()` for location info.**
4. **Useful for debug macros and assertions.**
5. **OCaml needs ppx for similar metaprogramming.**
