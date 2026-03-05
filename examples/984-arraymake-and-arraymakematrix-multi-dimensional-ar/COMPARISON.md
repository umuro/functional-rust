# OCaml vs Rust: Array.make and Array.make_matrix — Multi-dimensional Arrays

## Side-by-Side Code

### OCaml

```ocaml
(* 1D array — idiomatic *)
let zeros = Array.make 5 0

(* 2D matrix — idiomatic *)
let matrix = Array.make_matrix 3 4 0.0
let () = matrix.(1).(2) <- 42.0

(* Print the matrix *)
let () =
  Array.iter (fun row ->
    Array.iter (fun x -> Printf.printf "%.0f " x) row;
    print_newline ()
  ) matrix
```

### Rust (idiomatic — `vec!` macro)

```rust
// 1D vector
let zeros = vec![0; 5];

// 2D matrix — each row is an independent Vec<f64>
let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; 4]; 3];
matrix[1][2] = 42.0;

// Print the matrix
for row in &matrix {
    let formatted: Vec<String> = row.iter().map(|x| format!("{:.0}", x)).collect();
    println!("[{}]", formatted.join(", "));
}
```

### Rust (functional / iterator style)

```rust
// 1D vector — repeat exactly n times (std::iter::repeat_n, stabilised Rust 1.82)
fn make_iter<T: Clone>(n: usize, val: T) -> Vec<T> {
    std::iter::repeat_n(val, n).collect()
}

// 2D matrix — nested maps make cloning explicit
fn make_matrix_iter<T: Clone>(rows: usize, cols: usize, val: T) -> Vec<Vec<T>> {
    (0..rows)
        .map(|_| (0..cols).map(|_| val.clone()).collect())
        .collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| 1D array type | `'a array` | `Vec<T>` |
| 2D matrix type | `'a array array` | `Vec<Vec<T>>` |
| Make 1D | `val Array.make : int -> 'a -> 'a array` | `fn make<T: Clone>(n: usize, val: T) -> Vec<T>` |
| Make 2D | `val Array.make_matrix : int -> int -> 'a -> 'a array array` | `fn make_matrix<T: Clone>(rows: usize, cols: usize, val: T) -> Vec<Vec<T>>` |
| Index read | `arr.(i)` | `arr[i]` |
| Index write | `arr.(i) <- v` | `arr[i] = v` (requires `mut`) |
| Row independence | guaranteed by `Array.make_matrix` | guaranteed by `vec![vec![val; cols]; rows]` |

## Key Insights

1. **`vec![val; n]` is `Array.make`:** The `vec!` macro is Rust's built-in equivalent of `Array.make n val`. Both fill n slots with copies of `val`, and both require `val` to be cloneable (OCaml copies the value polymorphically; Rust requires the `Clone` bound).

2. **Row independence is structural:** OCaml's `Array.make_matrix` guarantees that each row is an independent allocation — unlike the footgun `Array.make n (Array.make m 0)` which shares one inner array. Rust's `vec![vec![val; cols]; rows]` has the same safe behaviour: the outer `vec!` clones the entire inner `Vec<T>`, producing fresh storage for every row.

3. **`std::iter::repeat_n` mirrors lazy lists:** `std::iter::repeat_n(val, n)` (stabilised in Rust 1.82) captures the "repeat this value exactly n times" idiom common in functional languages. The older pattern `std::iter::repeat(val).take(n)` is equivalent but `repeat_n` is preferred — it avoids the infinite iterator and makes intent explicit, similar to `List.init n (fun _ -> val)` in OCaml.

4. **Mutability is opt-in:** OCaml arrays are always mutable — `<-` needs no declaration. In Rust, `let mut matrix = ...` explicitly marks the binding as mutable. This makes mutation visible at the declaration site, not just at the use site.

5. **Indexing syntax:** OCaml uses dot-paren syntax `arr.(i)` for arrays and `arr.(i).(j)` for matrices. Rust uses bracket syntax `arr[i]` and `arr[i][j]`, matching C/Java convention while retaining bounds-checking at runtime.

## When to Use Each Style

**Use idiomatic Rust (`vec!` macro) when:** You want the most readable, zero-ceremony initialisation. `vec![0; n]` and `vec![vec![0.0; cols]; rows]` are immediately clear to any Rust reader and match the mental model of "fill with a default".

**Use iterator style (`repeat` / nested maps) when:** You want to make the element-level cloning explicit, or you are building on top of a more complex per-cell computation. The iterator form generalises naturally: replace `val.clone()` with any expression to compute each cell's initial value.
