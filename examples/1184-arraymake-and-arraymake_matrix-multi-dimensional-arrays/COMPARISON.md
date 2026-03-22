# OCaml vs Rust: Array.make and Array.make_matrix — Multi-Dimensional Arrays

## Side-by-Side Code

### OCaml
```ocaml
let zeros = Array.make 5 0
let matrix = Array.make_matrix 3 4 0.0
let () = matrix.(1).(2) <- 42.0
let () =
  Array.iter (fun row ->
    Array.iter (fun x -> Printf.printf "%.0f " x) row;
    print_newline ()
  ) matrix
```

### Rust (idiomatic)
```rust
// 1D: vec![value; n] mirrors Array.make n value
let zeros: Vec<i32> = vec![0; 5];

// 2D: Vec<Vec<T>> mirrors Array.make_matrix rows cols value
let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; 4]; 3];
matrix[1][2] = 42.0;

for row in &matrix {
    for x in row {
        print!("{:.0} ", x);
    }
    println!();
}
```

### Rust (functional — helper functions)
```rust
pub fn make<T: Clone>(n: usize, value: T) -> Vec<T> {
    vec![value; n]
}

pub fn make_matrix<T: Clone>(rows: usize, cols: usize, value: T) -> Vec<Vec<T>> {
    vec![vec![value; cols]; rows]
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| 1D array | `Array.make : int -> 'a -> 'a array` | `vec![value; n]` or `fn make<T: Clone>(n: usize, v: T) -> Vec<T>` |
| 2D matrix | `Array.make_matrix : int -> int -> 'a -> 'a array array` | `vec![vec![value; cols]; rows]` → `Vec<Vec<T>>` |
| Index access | `arr.(i)` | `arr[i]` |
| Mutation | `arr.(i) <- v` (in-place) | `arr[i] = v` (in-place) |
| Bounds check | runtime exception | runtime panic (in debug), or use `.get_mut()` for `Option` |

## Key Insights

1. **Independent rows**: OCaml's `Array.make_matrix` creates independent row arrays — mutating `matrix.(1).(2)` does not affect row 0 or row 2. Rust's `vec![vec![value; cols]; rows]` has the same semantics because each inner `Vec` is an independent allocation.
2. **Mutation model**: Both languages support in-place mutation (`matrix.(1).(2) <- 42.0` in OCaml, `matrix[1][2] = 42.0` in Rust). OCaml arrays are mutable by default; Rust requires `mut`.
3. **Bounds checking**: OCaml raises `Invalid_argument` on out-of-bounds access at runtime; Rust panics in debug mode. Both languages offer safe alternatives (`Array.get` in OCaml, `slice.get()` in Rust) that return `Option`.
4. **Generic over type**: Both functions are generic — OCaml uses parametric polymorphism `'a`, Rust uses `T: Clone`. The `Clone` bound is needed because `vec![value; n]` clones the value `n - 1` times.
5. **Functional style**: OCaml encourages immutable data structures; arrays are the exception (the imperative escape hatch). Rust similarly prefers iterators but `Vec` is the standard mutable sequence.

## When to Use Each Style

**Use `vec![value; n]` / `vec![vec![...]; rows]` when:** creating fixed-size arrays in production code — it's the idiomatic, zero-boilerplate Rust approach.
**Use helper functions `make` / `make_matrix` when:** you want a direct parallel to OCaml's `Array.make` API for educational purposes or when building a uniform interface over matrix creation.
