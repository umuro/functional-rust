# OCaml vs Rust: Reborrowing

## OCaml
```ocaml
(* No concept of reborrowing — refs work differently *)
let x = ref 42
let read_value r = !r
let increment r = r := !r + 1

let () =
  let _ = read_value x in
  increment x
```

## Rust
```rust
// Implicit reborrow: &mut T -> &T
pub fn demo() {
    let mut x = 42;
    let r = &mut x;

    let val = read_value(r);  // reborrows as &i32
    *r += 1;                   // original borrow still valid
}

// Explicit reborrow: &*r
let shared: &i32 = &*r;
```

## Key Differences

1. **OCaml**: ref cells with deref/assign operators
2. **Rust**: Implicit reborrowing for ergonomics
3. **Rust**: &mut T automatically reborrows as &T
4. **Rust**: Enables method chaining with &mut self
5. Both: Allow temporary shared access to mutable data
