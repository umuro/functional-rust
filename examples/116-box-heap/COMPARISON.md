# OCaml vs Rust: Box\<T\> — Heap Allocation

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml allocates everything on the GC heap automatically. *)
(* Recursive types work without indirection syntax: *)
type expr =
  | Num of int
  | Add of expr * expr
  | Mul of expr * expr

let rec eval = function
  | Num n -> n
  | Add (a, b) -> eval a + eval b
  | Mul (a, b) -> eval a * eval b

let () =
  let e = Add (Num 1, Mul (Num 2, Num 3)) in
  assert (eval e = 7);
  print_endline "ok"
```

### Rust (idiomatic — trait objects)
```rust
pub trait Shape { fn area(&self) -> f64; }

pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}
```

### Rust (functional/recursive — expression evaluator)
```rust
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),  // Box breaks the infinite-size cycle
    Mul(Box<Expr>, Box<Expr>),
}

pub fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Num(n)    => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Heap allocation | automatic (GC) | `Box::new(value)` |
| Recursive type | `type expr = Add of expr * expr` | `Add(Box<Expr>, Box<Expr>)` |
| Trait object | first-class module / object | `Box<dyn Trait>` |
| Pointer size | hidden (GC word) | `std::mem::size_of::<Box<T>>() == 8` |
| Deallocation | GC | automatic on `Drop` |

## Key Insights

1. **Implicit vs explicit heap allocation:** OCaml's GC heap-allocates nearly everything (tuples, variants, closures) with no syntax at all. Rust defaults to the *stack* and requires `Box::new(...)` to opt into heap allocation, making the memory location explicit and auditable.

2. **Recursive types:** OCaml's algebraic types are represented as GC-managed pointers internally, so `Add of expr * expr` just works. Rust needs the `Box` wrapper to make the recursive variant's size finite; without it the compiler rejects the type as having infinite size.

3. **No garbage collector:** `Box<T>` follows Rust's ownership rules — when the `Box` goes out of scope, `drop` is called and the heap memory is freed immediately. There is no pause, no collector thread, and no runtime overhead beyond the allocation itself.

4. **Trait objects and heterogeneous collections:** OCaml achieves polymorphism via parametric types or first-class modules. Rust uses `Box<dyn Trait>` to store different concrete types behind a uniform pointer, enabling `Vec<Box<dyn Shape>>` where each element may be a different struct.

5. **Zero overhead for `Box<T>`:** At runtime, a `Box<T>` compiles down to a raw pointer plus a `free` on drop — exactly what a C programmer would write by hand. There is no reference counting, no fat pointer (unlike `Rc` or `Arc`), and no metadata unless the pointee is a `dyn Trait` (which adds a vtable pointer).

## When to Use Each Style

**Use `Box<T>` (idiomatic Rust) when:** you need a single-owner heap allocation — a recursive enum, a large array you don't want on the stack, or a `Box<dyn Trait>` for runtime polymorphism.

**Use recursive Rust when:** modelling tree-structured data (ASTs, parse trees, linked lists) and you want the OCaml-like pattern-match style, just with explicit `Box` indirection at each recursive position.
