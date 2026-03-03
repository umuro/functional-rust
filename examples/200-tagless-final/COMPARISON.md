# OCaml vs Rust: Tagless Final

## Side-by-Side Code

### OCaml

```ocaml
module type EXPR = sig
  type 'a repr
  val int  : int  -> int repr
  val bool : bool -> bool repr
  val add  : int repr -> int repr -> int repr
  val mul  : int repr -> int repr -> int repr
  val leq  : int repr -> int repr -> bool repr
  val if_  : bool repr -> 'a repr -> 'a repr -> 'a repr
end

module Eval : EXPR = struct
  type 'a repr = 'a
  let int  n    = n
  let bool b    = b
  let add  a b  = a + b
  let mul  a b  = a * b
  let leq  a b  = a <= b
  let if_ c t e = if c then t else e
end

module Pretty : EXPR = struct
  type 'a repr = string
  let int  n    = string_of_int n
  let bool b    = string_of_bool b
  let add  a b  = Printf.sprintf "(%s + %s)" a b
  let mul  a b  = Printf.sprintf "(%s * %s)" a b
  let leq  a b  = Printf.sprintf "(%s <= %s)" a b
  let if_ c t e = Printf.sprintf "(if %s then %s else %s)" c t e
end

let program (type a) (module E : EXPR with type 'x repr = 'x a) =
  let open E in
  if_ (leq (add (int 3) (int 4)) (mul (int 2) (int 5)))
      (int 42) (int 0)
```

### Rust (idiomatic — GAT-based tagless final)

```rust
pub trait Expr {
    type Repr<T>;
    fn int(n: i64) -> Self::Repr<i64>;
    fn bool_val(b: bool) -> Self::Repr<bool>;
    fn add(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<i64>;
    fn mul(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<i64>;
    fn leq(a: Self::Repr<i64>, b: Self::Repr<i64>) -> Self::Repr<bool>;
    fn if_<T>(c: Self::Repr<bool>, t: Self::Repr<T>, e: Self::Repr<T>) -> Self::Repr<T>;
}

pub struct Eval;
impl Expr for Eval {
    type Repr<T> = T;
    fn int(n: i64) -> i64 { n }
    fn bool_val(b: bool) -> bool { b }
    fn add(a: i64, b: i64) -> i64 { a + b }
    fn mul(a: i64, b: i64) -> i64 { a * b }
    fn leq(a: i64, b: i64) -> bool { a <= b }
    fn if_<T>(c: bool, t: T, e: T) -> T { if c { t } else { e } }
}

pub struct Pretty;
impl Expr for Pretty {
    type Repr<T> = String;
    fn int(n: i64) -> String { n.to_string() }
    fn bool_val(b: bool) -> String { b.to_string() }
    fn add(a: String, b: String) -> String { format!("({a} + {b})") }
    fn mul(a: String, b: String) -> String { format!("({a} * {b})") }
    fn leq(a: String, b: String) -> String { format!("({a} <= {b})") }
    fn if_<T>(c: String, t: String, e: String) -> String {
        format!("(if {c} then {t} else {e})")
    }
}

pub fn program<E: Expr>() -> E::Repr<i64> {
    E::if_(
        E::leq(E::add(E::int(3), E::int(4)), E::mul(E::int(2), E::int(5))),
        E::int(42),
        E::int(0),
    )
}
```

### Rust (initial encoding — explicit AST for contrast)

```rust
pub enum Ast {
    Int(i64),
    Bool(bool),
    Add(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Leq(Box<Ast>, Box<Ast>),
    If(Box<Ast>, Box<Ast>, Box<Ast>),
}

pub fn eval_ast(ast: &Ast) -> Value {
    match ast {
        Ast::Int(n) => Value::Int(*n),
        Ast::Add(a, b) => match (eval_ast(a), eval_ast(b)) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
            _ => panic!("type error"),
        },
        // ... and so on for every node type
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| DSL interface | `module type EXPR` | `trait Expr` |
| Type constructor | `type 'a repr` | `type Repr<T>` (GAT) |
| Identity interpreter | `type 'a repr = 'a` | `type Repr<T> = T` |
| Constant interpreter | `type 'a repr = string` | `type Repr<T> = String` |
| Polymorphic program | `(module E : EXPR with type 'x repr = 'x a)` | `fn program<E: Expr>()` |
| Dispatch | First-class module | Monomorphisation |

## Key Insights

1. **GATs are the critical enabler.** Before Rust 1.65, tagless final required workarounds (wrapper structs, `PhantomData`). The `type Repr<T>` GAT directly mirrors OCaml's `type 'a repr` with no ceremony.

2. **Monomorphisation replaces first-class modules.** OCaml passes the interpreter as a runtime module value. Rust bakes the choice into the type at compile time via `<E: Expr>` — zero overhead, no boxing.

3. **`Repr<T> = T` is the identity type function.** For `Eval`, the "representation" of an `i64` is literally an `i64`. This is the same trick OCaml uses with `type 'a repr = 'a`, and Rust expresses it identically.

4. **`Repr<T> = String` ignores the phantom type.** `Pretty` always returns `String` regardless of what `T` is. This is the "constant functor" pattern — `T` is phantom, present only to satisfy the trait interface and preserve type-level information for callers.

5. **Initial vs. final encoding trade-off.** The *initial* encoding (AST enum) centralises interpretation in one `eval_ast` function — easy to add new expression types, hard to add new interpreters without touching the match arms. The *final* encoding (tagless trait) distributes interpretation — easy to add new interpreters as `impl` blocks, but extending the DSL with new operations requires updating every existing interpreter.

## When to Use Each Style

**Use tagless final (trait-based) when:** you want multiple independent interpreters (evaluate, pretty-print, type-check, compile) that must not depend on each other, or when you're building an extensible embedded DSL where new backends are added by library consumers.

**Use initial encoding (AST enum) when:** you need to inspect, transform, or optimise the expression tree itself (e.g., constant folding, serialisation, pattern matching over the structure), or when adding new expression forms is the primary axis of extension.
