# Comparison: Expression Tree — OCaml vs Rust

## OCaml

```ocaml
type expr =
  | Num of float
  | Add of expr * expr
  | Mul of expr * expr  (* etc. *)

let rec eval = function
  | Num n      -> n
  | Add (l, r) -> eval l +. eval r
  | Mul (l, r) -> eval l *. eval r
  (* ... *)

let rec to_string = function
  | Num n      -> string_of_float n
  | Add (l, r) -> Printf.sprintf "(%s + %s)" (to_string l) (to_string r)
  (* ... *)
```

## Rust — Idiomatic (Box + impl)

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),  // etc.
}

impl Expr {
    pub fn new_add(l: Expr, r: Expr) -> Self {
        Expr::Add(Box::new(l), Box::new(r))
    }

    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(n)      => *n,
            Expr::Add(l, r)   => l.eval() + r.eval(),
            Expr::Mul(l, r)   => l.eval() * r.eval(),
            // ...
        }
    }
}
```

## Rust — Safe Division (Result)

```rust
pub fn eval_safe(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Div(l, r) => {
            let divisor = eval_safe(r)?;
            if divisor == 0.0 { Err("Division by zero".into()) }
            else { Ok(eval_safe(l)? / divisor) }
        }
        // ...
    }
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Recursive type | `type expr = Num of float \| Add of expr * expr` | `enum Expr { Num(f64), Add(Box<Expr>, Box<Expr>) }` |
| Heap allocation | Implicit (GC-managed) | Explicit with `Box<T>` |
| Constructor | `Add (l, r)` directly | `Expr::Add(Box::new(l), Box::new(r))` or helper |
| Pattern matching | `\| Add (l, r) -> ...` | `Expr::Add(l, r) => ...` (l, r are `&Box<Expr>`) |
| Float ops | `+.` `-*` `*.` `/.` (separate from int) | `+` `-` `*` `/` (overloaded via traits) |
| Division safety | Returns `infinity` | Can return `Result` with error |
| Pretty-print | Custom `to_string` function | `impl Display` trait |

## Type Signatures Explained

**OCaml:** `val eval : expr -> float` — takes an expression, returns a float. The recursive structure is handled by the GC.

**Rust:** `fn eval(&self) -> f64` — borrows `self` (`&Expr`). When matching `Add(l, r)`, `l` and `r` are `&Box<Expr>`, which auto-derefs to `&Expr` for the recursive call.

## Takeaways

1. **Box is the price of ownership:** OCaml's GC handles recursive types transparently; Rust makes heap allocation explicit
2. **Constructor boilerplate:** Helper functions like `new_add()` are a common Rust pattern to hide `Box::new`
3. **Safety upgrade:** Rust's `Result` type makes error handling for division explicit and composable with `?`
4. **Display trait** integrates with `format!`, `println!`, and `to_string()` automatically
5. **Auto-deref** makes working with `Box<Expr>` ergonomic — you rarely notice the indirection in pattern matching
