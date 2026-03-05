# OCaml vs Rust: Const Generics Basics

## Array with Compile-Time Size

### Rust
```rust
pub struct Array<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Array<T, N> {
    pub fn new() -> Self {
        Array { data: [T::default(); N] }
    }
    
    pub const fn len(&self) -> usize { N }
}

let arr: Array<i32, 5> = Array::new();
```

### OCaml
OCaml doesn't have const generics. Closest is:
```ocaml
(* Using Bigarray with fixed dimensions *)
let arr = Bigarray.Array1.create Bigarray.int Bigarray.c_layout 5

(* Or GADTs for type-level naturals *)
type z
type 'n s

type ('a, 'n) vec =
  | Nil : ('a, z) vec
  | Cons : 'a * ('a, 'n) vec -> ('a, 'n s) vec
```

## Matrix with Dimensions

### Rust
```rust
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

let m: Matrix<f64, 3, 4> = Matrix::new();
assert_eq!(m.rows(), 3);  // Compile-time known
```

### OCaml (GADTs approach)
```ocaml
(* Requires type-level naturals encoding *)
type ('rows, 'cols) matrix = float array array
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Const params | Not native | `const N: usize` |
| Type-level numbers | GADTs (complex) | Native const generics |
| Array size | Runtime | Compile-time |
| Zero-cost | Not applicable | Yes |
| Compile errors | Runtime if wrong | Compile-time |
