# OCaml vs Rust: Collection Literal Macros

## Rust Collection Macros

```rust
// vec! is built-in
let v = vec![1, 2, 3];

// Custom macros for other collections
let s = set![1, 2, 3];
let m = map!{"a" => 1, "b" => 2};
let d = deque![1, 2, 3];
```

## OCaml Collection Literals

```ocaml
(* Lists are literal *)
let l = [1; 2; 3]

(* Sets/Maps via modules *)
let s = IntSet.(empty |> add 1 |> add 2 |> add 3)
let m = StringMap.(empty |> add "a" 1 |> add "b" 2)

(* Or via List.to_seq *)
let s = IntSet.of_list [1; 2; 3]
```

## 5 Takeaways

1. **`vec!` is built-in; others need custom macros.**
2. **OCaml uses module functions, not literals.**
3. **Macros enable natural syntax: `set![1, 2, 3]`.**
4. **All standard collections can have literal macros.**
5. **Trailing commas: `$(,)?` makes them optional.**
