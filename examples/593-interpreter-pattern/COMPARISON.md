# OCaml vs Rust: Interpreter Pattern

## Expression Type

### OCaml
```ocaml
type expr =
  | Lit of float
  | Var of string
  | Add of expr * expr
  | Let of string * expr * expr
  | If  of expr * expr * expr
```

### Rust
```rust
enum Expr {
    Lit(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Let { name: String, value: Box<Expr>, body: Box<Expr> },
    If { cond: Box<Expr>, then_: Box<Expr>, else_: Box<Expr> },
}
```

## Evaluation

### OCaml
```ocaml
let rec eval env = function
  | Lit n         -> n
  | Var x         -> List.assoc x env
  | Add(l, r)     -> eval env l +. eval env r
  | Let(x, e, b)  -> eval ((x, eval env e) :: env) b
  | If(c, t, f)   -> if eval env c <> 0.0 then eval env t else eval env f
```

### Rust
```rust
fn eval(env: &Env, e: &Expr) -> Result<f64, String> {
    match e {
        Expr::Lit(n) => Ok(*n),
        Expr::Var(x) => env.get(x).copied().ok_or_else(|| format!("undefined: {}", x)),
        Expr::Add(l, r) => Ok(eval(env, l)? + eval(env, r)?),
        Expr::Let { name, value, body } => {
            let v = eval(env, value)?;
            let mut env2 = env.clone();
            env2.insert(name.clone(), v);
            eval(&env2, body)
        }
        // ...
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Recursion** | Direct | Requires `Box<>` for size |
| **Errors** | Exceptions | `Result<T, E>` |
| **Environment** | Association list | `HashMap` |
| **Struct variants** | Record syntax | Named fields in enum |
