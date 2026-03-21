📖 **[View on hightechmind.io →](https://hightechmind.io/rust/593-interpreter-pattern)**

---

# Interpreter Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Building a small interpreter is the classic exercise in language implementation and a demonstration of algebraic data types at their most powerful. An AST (Abstract Syntax Tree) is a recursive enum; evaluation is a recursive function over it. This pattern underpins every compiler, template engine, query language, and rule engine. The `Expr` enum with `Lit`, `Var`, `Add`, `Mul`, `Let`, `If` variants covers the core of any expression language.

## Learning Outcomes

- How a recursive `Expr` enum models an arithmetic expression language
- How `eval(expr: &Expr, env: &HashMap<String, f64>) -> Result<f64, Error>` interprets the AST
- How `Let` binding and `If` conditional work in the evaluator
- How `Box<Expr>` enables recursive types without infinite size
- Where interpreter pattern appears: template engines, config DSLs, query languages, scripting

## Rust Application

`Expr` has `Lit(f64)`, `Var(String)`, `Add(Box<Expr>, Box<Expr>)`, `Sub`, `Mul`, `Div`, `Let { name, value, body }`, and `If { cond, then_, else_ }`. `eval` recursively matches each variant — `Add` evaluates both subexpressions and adds. `Let` evaluates `value`, inserts it into a clone of the environment, and evaluates `body`. Division checks for zero and returns an error.

Key patterns:
- `Box<Expr>` for recursive variants — prevents infinite-size type
- `eval(expr, env) -> Result<f64, Error>` — recursive evaluation
- Pattern matching on variants with nested patterns
- `HashMap` environment for variable lookup

## OCaml Approach

OCaml's ADTs make this the canonical example — LISP interpreters, mini-ML, and tutorial compilers all use this structure:

```ocaml
type expr = Lit of float | Var of string | Add of expr * expr
  | Let of { name: string; value: expr; body: expr }
let rec eval env = function
  | Lit n -> Ok n
  | Var x -> (match List.assoc_opt x env with Some v -> Ok v | None -> Error ("unbound: " ^ x))
  | Add (a, b) -> let* va = eval env a in let* vb = eval env b in Ok (va +. vb)
  | Let { name; value; body } -> let* v = eval env value in eval ((name, v) :: env) body
```

## Key Differences

1. **Box requirement**: Rust requires `Box<Expr>` for recursive types (to bound the size); OCaml's heap-allocated values make recursive types natural without boxing annotation.
2. **Environment cloning**: Rust's immutable HashMap requires cloning for `Let` extension or using a persistent map; OCaml uses association lists or functional maps with O(1) consing.
3. **Error propagation**: Rust uses `?` and `Result` for division-by-zero and unbound variable; OCaml uses `result` with `let*` or exceptions.
4. **Type inference**: Both languages infer the type of the `eval` function without annotation in most cases.

## Exercises

1. **Add function calls**: Extend `Expr` with `Call { func: String, args: Vec<Expr> }` and add built-in functions `sqrt`, `abs`, `max` to the evaluator.
2. **Pretty printer**: Write `fn pretty(expr: &Expr) -> String` that produces readable infix notation for arithmetic expressions with minimal parentheses.
3. **Type checker**: Write `fn type_check(expr: &Expr) -> Result<Type, TypeError>` where `Type` is `Num | Bool` — detect type mismatches before evaluation.
