# OCaml vs Rust: Non-Lexical Lifetimes

## OCaml
```ocaml
(* No concept of borrows — GC manages memory *)
let example () =
  let v = [1; 2; 3] in
  let first = List.hd v in
  (* No restrictions on v after "borrowing" *)
  first
```

## Rust (NLL - Rust 2018+)
```rust
pub fn nll_basic() -> Vec<i32> {
    let mut v = vec![1, 2, 3];
    let first = v[0];  // borrow ends here (NLL)
    v.push(6);         // OK: borrow already ended
    v
}

// Pre-NLL: error! borrow lasted until end of block
```

## Key Differences

1. **OCaml**: No borrow tracking, GC handles all
2. **Rust NLL**: Borrows end at last use, not scope end
3. **Rust**: Enables more natural code patterns
4. **Rust**: Conditional borrows work correctly
5. Both: Prevent use-after-free, different mechanisms
