# Comparison: Monad Laws

## Left Identity: return a >>= f ≡ f a

**OCaml:**
```ocaml
assert ((return_ 5 >>= double) = double 5)
(* Some 5 >>= double = double 5 = Some 10 *)
```

**Rust:**
```rust
assert_eq!(Some(5).and_then(double), double(5));
```

## Right Identity: m >>= return ≡ m

**OCaml:**
```ocaml
assert ((Some 42 >>= return_) = Some 42)
assert ((None >>= return_) = None)
```

**Rust:**
```rust
assert_eq!(Some(42).and_then(Some), Some(42));
assert_eq!(None::<i32>.and_then(Some), None);
```

## Associativity: (m >>= f) >>= g ≡ m >>= (fun x -> f x >>= g)

**OCaml:**
```ocaml
let left = (Some 5 >>= double) >>= inc in
let right = Some 5 >>= (fun x -> double x >>= inc) in
assert (left = right)
```

**Rust:**
```rust
let left = Some(5).and_then(double).and_then(inc);
let right = Some(5).and_then(|x| double(x).and_then(inc));
assert_eq!(left, right);
```

## List Monad Laws

**OCaml:**
```ocaml
let lbind xs f = List.concat_map f xs
assert (lbind [3] expand = expand 3)
```

**Rust:**
```rust
let result: Vec<_> = vec![3].iter().flat_map(expand).collect();
assert_eq!(result, expand(&3));
```
