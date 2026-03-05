# OCaml vs Rust: Macro Repetition

## Side-by-Side Code

### OCaml — Variadic via lists
```ocaml
let sum_list xs = List.fold_left (+) 0 xs
let product_list xs = List.fold_left ( * ) 1 xs

let () =
  Printf.printf "sum: %d\n" (sum_list [1;2;3;4;5]);
  Printf.printf "product: %d\n" (product_list [2;3;4])
```

### Rust — Macro repetition
```rust
macro_rules! sum {
    () => { 0 };
    ($first:expr $(, $rest:expr)*) => {
        $first $(+ $rest)*
    };
}

macro_rules! product {
    () => { 1 };
    ($first:expr $(, $rest:expr)*) => {
        $first $(* $rest)*
    };
}

fn main() {
    println!("sum: {}", sum!(1, 2, 3, 4, 5));
    println!("product: {}", product!(2, 3, 4));
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Variadic args | Lists | Macro repetition |
| Syntax | `[1;2;3]` | `sum!(1, 2, 3)` |
| Type checking | At call site | During expansion |
| Performance | List allocation | Zero-cost (inlined) |

---

## Repetition Patterns

```rust
// Zero or more (*)
$(pattern),* 

// One or more (+)
$(pattern),+

// Zero or one (?)
$(pattern)?
```

### Examples
```rust
// Zero or more items
macro_rules! vec_of {
    ($($x:expr),* $(,)?) => { vec![$($x),*] };
}

// At least one item
macro_rules! min {
    ($x:expr $(, $y:expr)+) => { ... };
}

// Optional trailing comma
macro_rules! map {
    ($($k:expr => $v:expr),+ $(,)?) => { ... };
}
```

---

## 5 Takeaways

1. **OCaml uses lists for variadic; Rust uses macro repetition.**
   `[1;2;3]` vs `sum!(1, 2, 3)`.

2. **Rust macros expand at compile time.**
   No runtime overhead from list construction.

3. **`$(...)*` = zero or more; `$(...)+` = one or more.**
   Choose based on whether empty input is valid.

4. **Trailing comma: `$(,)?` makes it optional.**
   Common pattern for comma-separated macros.

5. **Repetition in expansion mirrors capture.**
   `$(+ $rest)*` repeats the `+` for each captured `$rest`.
