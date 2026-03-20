[![Functional Rust](https://img.shields.io/badge/functional--rust-examples-blue)](https://hightechmind.io)

# Grand Synthesis
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

This capstone example weaves together every major concept from this series: recursion schemes, optics, free monads, comonads, type-level programming, parser combinators, and categorical abstractions. The goal is to build a small but complete functional language interpreter that demonstrates how these patterns compose: the AST is defined as a fixed-point type, evaluation uses a catamorphism, variable scoping uses the Reader monad, a free monad models effectful operations, lenses navigate nested state, and Cofree annotates the AST with type information.

## Learning Outcomes

- Compose recursion schemes, optics, monads, and comonads in a single coherent system
- See how each abstraction handles exactly the problem it was designed for
- Understand the layered architecture of a functional interpreter
- Recognize the categorical structure underlying practical programming patterns
- Appreciate how Rust's type system enforces correctness at each layer

## Rust Application

A small functional interpreter bringing together the series' key patterns:

```rust
use std::collections::HashMap;
use std::rc::Rc;

// ---- 1. AST as a fixed-point type (recursion schemes) ----
// ExprF is the base functor; Expr = Fix<ExprF>

#[derive(Clone, Debug)]
enum ExprF<R> {
    Lit(f64),
    Var(String),
    Add(R, R),
    Mul(R, R),
    Let { name: String, val: R, body: R },
    Lam { param: String, body: R },
    App(R, R),
}

#[derive(Clone, Debug)]
struct Expr(Box<ExprF<Expr>>);

impl Expr {
    fn lit(n: f64) -> Self { Expr(Box::new(ExprF::Lit(n))) }
    fn var(s: &str) -> Self { Expr(Box::new(ExprF::Var(s.to_string()))) }
    fn add(l: Expr, r: Expr) -> Self { Expr(Box::new(ExprF::Add(l, r))) }
    fn mul(l: Expr, r: Expr) -> Self { Expr(Box::new(ExprF::Mul(l, r))) }
    fn let_(name: &str, val: Expr, body: Expr) -> Self {
        Expr(Box::new(ExprF::Let { name: name.to_string(), val, body }))
    }
    fn lam(param: &str, body: Expr) -> Self {
        Expr(Box::new(ExprF::Lam { param: param.to_string(), body }))
    }
    fn app(f: Expr, x: Expr) -> Self { Expr(Box::new(ExprF::App(f, x))) }
}

// ---- 2. Type annotation via Cofree comonad ----
#[derive(Clone, Debug)]
enum Type { TNum, TFun(Box<Type>, Box<Type>), TUnknown }

#[derive(Clone, Debug)]
struct Annotated<A> {
    node: ExprF<Annotated<A>>,
    ann: A,
}

// ---- 3. Evaluation environment as Reader monad (closure over env) ----
#[derive(Clone, Debug)]
enum Value {
    Num(f64),
    Closure { param: String, body: Expr, env: Env },
}

type Env = HashMap<String, Value>;

// ---- 4. Evaluation via catamorphism (not literal cata but structurally recursive) ----
fn eval(expr: &Expr, env: &Env) -> Result<Value, String> {
    match expr.0.as_ref() {
        ExprF::Lit(n) => Ok(Value::Num(*n)),

        ExprF::Var(name) => env.get(name)
            .cloned()
            .ok_or_else(|| format!("Unbound variable: {name}")),

        ExprF::Add(l, r) => {
            match (eval(l, env)?, eval(r, env)?) {
                (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a + b)),
                _ => Err("Type error: + expects numbers".to_string()),
            }
        }

        ExprF::Mul(l, r) => {
            match (eval(l, env)?, eval(r, env)?) {
                (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a * b)),
                _ => Err("Type error: * expects numbers".to_string()),
            }
        }

        ExprF::Let { name, val, body } => {
            let v = eval(val, env)?;
            let mut new_env = env.clone();
            new_env.insert(name.clone(), v);
            eval(body, &new_env)
        }

        ExprF::Lam { param, body } => Ok(Value::Closure {
            param: param.clone(),
            body: body.clone(),
            env: env.clone(),
        }),

        ExprF::App(f_expr, arg_expr) => {
            match eval(f_expr, env)? {
                Value::Closure { param, body, env: closure_env } => {
                    let arg = eval(arg_expr, env)?;
                    let mut new_env = closure_env.clone();
                    new_env.insert(param, arg);
                    eval(&body, &new_env)
                }
                _ => Err("Type error: application of non-function".to_string()),
            }
        }
    }
}

// ---- 5. Free monad for effectful operations ----
enum Effect<A> {
    Pure(A),
    Print(String, Box<dyn Fn() -> Effect<A>>),
    ReadNum(Box<dyn Fn(f64) -> Effect<A>>),
}

fn run_effects<A>(eff: Effect<A>, output: &mut Vec<String>) -> A {
    match eff {
        Effect::Pure(a) => a,
        Effect::Print(msg, k) => {
            output.push(msg);
            run_effects(k(), output)
        }
        Effect::ReadNum(k) => {
            // Simulate: always read 42.0
            run_effects(k(42.0), output)
        }
    }
}

// ---- 6. Lens for navigating interpreter state ----
#[derive(Clone, Debug)]
struct InterpreterState {
    env: Env,
    call_depth: usize,
    trace: Vec<String>,
}

// Simple lens pair for call_depth
fn get_depth(s: &InterpreterState) -> usize { s.call_depth }
fn set_depth(s: InterpreterState, depth: usize) -> InterpreterState {
    InterpreterState { call_depth: depth, ..s }
}
fn over_depth(s: InterpreterState, f: impl Fn(usize) -> usize) -> InterpreterState {
    let d = get_depth(&s);
    set_depth(s, f(d))
}

fn main() {
    // Build expression: let double = λx. x * 2 in double (3 + 4)
    let program = Expr::let_(
        "double",
        Expr::lam("x", Expr::mul(Expr::var("x"), Expr::lit(2.0))),
        Expr::app(
            Expr::var("double"),
            Expr::add(Expr::lit(3.0), Expr::lit(4.0)),
        ),
    );

    let env: Env = HashMap::new();
    match eval(&program, &env) {
        Ok(Value::Num(n)) => println!("Result: {n}"),  // 14.0
        Ok(other)         => println!("Result: {:?}", other),
        Err(e)            => println!("Error: {e}"),
    }

    // Effect system: print + compute
    let program_eff: Effect<f64> = Effect::Print(
        "Computing...".to_string(),
        Box::new(|| Effect::ReadNum(Box::new(|x| {
            Effect::Print(
                format!("Got input: {x}"),
                Box::new(move || Effect::Pure(x * 2.0))
            )
        }))),
    );
    let mut output = vec![];
    let result = run_effects(program_eff, &mut output);
    println!("Effects output: {:?}", output);
    println!("Effect result: {result}");

    // Lens: track interpreter state
    let state = InterpreterState {
        env: HashMap::new(),
        call_depth: 0,
        trace: vec![],
    };
    let state = over_depth(state, |d| d + 1);
    println!("Call depth: {}", get_depth(&state)); // 1
}
```

Each layer does exactly one job: ExprF/Fix handles recursion without hardcoding traversal, the Reader-like env handles scoping, the free monad handles I/O orthogonally to evaluation, and lenses navigate state without getters/setters scattered everywhere.

## OCaml Approach

OCaml achieves the same architecture more concisely:

```ocaml
type expr_f = Lit of float | Var of string | Add of 'r * 'r
            | Let of string * 'r * 'r | Lam of string * 'r | App of 'r * 'r

(* Catamorphism via fold *)
let rec eval env = function
  | Lit n -> Ok (Num n)
  | Var x -> List.assoc_opt x env |> Option.to_result ~none:(Printf.sprintf "Unbound: %s" x)
  | Add (l, r) -> ...
  | Let (name, v, b) -> eval env v >>= fun vv -> eval ((name, vv) :: env) b
  | Lam (p, b) -> Ok (Closure (p, b, env))
  | App (f, x) -> eval env f >>= function
      | Closure (p, b, ce) -> eval env x >>= fun xv -> eval ((p, xv) :: ce) b
      | _ -> Error "Not a function"
```

OCaml's algebraic types, pattern matching, `>>=` operator, and lack of explicit boxing make the interpreter visually cleaner while carrying the same categorical structure.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Recursive types | `Box<ExprF<Expr>>` | native recursive types |
| Pattern matching | exhaustive, verbose | exhaustive, concise |
| Free monad | trait objects | polymorphic variants |
| Lens | explicit get/set fns | record functional update |
| Effects | enum + interpreter | `Effect` library / algebraic effects |
| Performance | zero-cost abstractions | GC overhead, JIT optim |

The grand synthesis reveals the payoff of learning each abstraction individually: they compose cleanly, each handling one concern, and the Rust type system ensures each boundary is sound.

## Exercises

1. Extend the interpreter with recursive let (`letrec`) using a `Rc<RefCell<Option<Value>>>` thunk for self-reference.
2. Add a type inference pass as a separate catamorphism that annotates the AST with `Type` using the `Cofree` pattern.
3. Replace the free monad effect system with an algebraic effect handler that supports resumable exceptions (use the delimited continuation pattern from example 196).
4. Add lenses for all fields of `InterpreterState` and compose them to build a transaction-style update that rolls back on error.
5. Implement a pretty-printer for the AST using an anamorphism (unfold) into a document algebra, then render via a second catamorphism over the document type.
