# OCaml vs Rust: Const Type Arithmetic

## Type-Level Arithmetic

### Rust
```rust
pub struct Add<const A: usize, const B: usize>;

impl<const A: usize, const B: usize> Add<A, B> {
    pub const VALUE: usize = A + B;
}

// Compile-time sum
const SUM: usize = Add::<3, 4>::VALUE; // 7
```

### OCaml (GADTs)
```ocaml
(* Complex type-level naturals *)
type z = Z
type 'n s = S of 'n

type ('a, 'b, 'c) add =
  | Add_z : (z, 'b, 'b) add
  | Add_s : ('a, 'b, 'c) add -> ('a s, 'b, 'c s) add
```

## Matrix with Dimension Types

### Rust
```rust
pub fn matmul<const M: usize, const N: usize, const P: usize>(
    a: &Matrix<M, N>,
    b: &Matrix<N, P>,
) -> Matrix<M, P>  // Dimensions guaranteed!

let a: Matrix<2, 3> = ...;
let b: Matrix<3, 4> = ...;
let c = matmul(&a, &b);  // c: Matrix<2, 4>
```

### OCaml
```ocaml
(* Runtime dimension check *)
let matmul a b =
  if Array.length a.(0) <> Array.length b then
    invalid_arg "dimension mismatch";
  ...
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type-level numbers | GADTs (complex) | Native const generics |
| Dimension check | Runtime | Compile-time |
| Syntax | Witness types | `<const N: usize>` |
| Arithmetic | Custom types | Native operators |
