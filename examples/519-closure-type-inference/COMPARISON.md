# OCaml vs Rust: Closure Type Inference

## OCaml
```ocaml
(* Full Hindley-Milner inference *)
let double = fun x -> x * 2  (* inferred: int -> int *)
let id x = x                 (* polymorphic: 'a -> 'a *)
```

## Rust
```rust
// Inferred from first use, then fixed
let double = |x| x * 2;
let _ = double(5i32);  // fixes type forever

// Closures are monomorphic — not polymorphic
let id = |x| x;
let _: i32 = id(5);    // fixed as i32
// id("hello");        // ERROR: already fixed
```

## Key Differences

1. **OCaml**: Full polymorphic inference (Hindley-Milner)
2. **Rust**: Closures are monomorphic — fixed by first use
3. **OCaml**: `let id x = x` is polymorphic
4. **Rust**: Need generics for polymorphism: `fn id<T>(x: T) -> T`
5. Both infer types from context in many cases
