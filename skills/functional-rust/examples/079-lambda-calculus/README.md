# 079: Lambda Calculus Interpreter

**Difficulty:** 4  **Level:** Advanced

Evaluate a minimal functional language — closures, application, and environments — using Rust enums.

## The Problem This Solves

Every time you write a function in any language, you're using lambda calculus. Understanding how to *implement* it is the foundation for: building your own scripting language, understanding how closures capture environments in Rust and OCaml, and seeing why garbage collection and ownership interact with closures in fundamentally different ways.

This example implements a tiny lambda calculus with integers, variables, lambda abstraction (`\x -> body`), function application, and addition. The interpreter uses an **environment model** (a list of variable bindings) to evaluate expressions. When a lambda is created, it *captures* the current environment — that's what makes it a closure.

The practical applications are immediate: embedded DSLs for rule engines, configuration languages, and formula evaluators all reduce to this pattern.

## The Intuition

**Lambda calculus** has three constructs:
- **Variable**: `x` — look up a name in the environment
- **Abstraction**: `\x -> body` — create a function that takes `x` and returns `body`
- **Application**: `f arg` — call function `f` with argument `arg`

Everything else (booleans, numbers, pairs, lists) can be encoded in pure lambda calculus using **Church encoding** — but for a practical interpreter we add integers and addition directly.

**The environment** is a list of `(name, value)` pairs. Looking up a variable means scanning the list from the end (most recent binding wins — this handles `let`-like shadowing). When a lambda is created, it captures a *snapshot* of the current environment. This snapshot becomes part of the closure's value.

**The key ownership question**: when you create a closure `\x -> x + y` in an environment where `y = 5`, you need to store that `y = 5` binding inside the closure. In OCaml, the garbage collector manages this — the environment is shared by reference. In Rust, the closure owns a clone of the environment. This means creating closures is O(n) in the environment size — a trade-off for memory safety without GC.

## How It Works in Rust

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Int(i64),
    Var(String),
    Lam(String, Box<Expr>),        // Box: recursive type must have known size
    App(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    VInt(i64),
    VClosure(String, Box<Expr>, Env),  // closure captures its environment
}

type Env = Vec<(String, Value)>;       // association list: [(name, value)]
```

The interpreter — each arm returns a `Result<Value, String>` for error handling:
```rust
pub fn eval(env: &Env, expr: &Expr) -> Result<Value, String> {
    match expr {
        Expr::Int(n) => Ok(Value::VInt(*n)),

        Expr::Var(x) => env.iter().rev()  // scan from end (most recent binding first)
            .find(|(k, _)| k == x)
            .map(|(_, v)| v.clone())
            .ok_or_else(|| format!("unbound variable: {}", x)),

        // Lam: capture the current environment — this is the closure
        Expr::Lam(x, body) => Ok(Value::VClosure(x.clone(), body.clone(), env.clone())),

        // App: evaluate function, evaluate argument, extend the closure's environment
        Expr::App(f, arg) => {
            let fv = eval(env, f)?;
            let av = eval(env, arg)?;
            match fv {
                Value::VClosure(x, body, mut cenv) => {
                    cenv.push((x, av));     // bind the argument in the closure's env
                    eval(&cenv, &body)      // evaluate body in extended environment
                }
                _ => Err("not a function".into()),
            }
        }

        Expr::Add(a, b) => match (eval(env, a)?, eval(env, b)?) {
            (Value::VInt(x), Value::VInt(y)) => Ok(Value::VInt(x + y)),
            _ => Err("type error in add".into()),
        },
    }
}
```

Example: applying the identity function `(\x -> x) 42`:
```rust
let identity_app = Expr::App(
    Box::new(Expr::Lam("x".into(), Box::new(Expr::Var("x".into())))),
    Box::new(Expr::Int(42)),
);
assert_eq!(eval(&vec![], &identity_app), Ok(Value::VInt(42)));
```

## What This Unlocks

- **Embedding DSLs**: this pattern is the core of every rule engine, formula evaluator, and scripting language embedded in Rust applications.
- **Understanding closures**: see exactly how closures capture environments — the `env.clone()` in `Lam` makes the capture concrete and visible.
- **Foundation for type checkers**: add a type annotation field to `Expr` and a `type_check` function alongside `eval` — you now have a typed lambda calculus.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive types | `type expr = Lam of string * expr` — allowed directly | Requires `Box<Expr>` to give the enum a finite size |
| Closure capture | Environment shared by reference (GC) | `env.clone()` — full copy at closure creation time |
| Variable lookup | `List.assoc` (raises `Not_found`) | `iter().rev().find()` + `.ok_or_else()` |
| Error handling | Exceptions (`Not_found`, custom) | `Result<Value, String>` + `?` operator |
| Sharing expression trees | `Gc` or `Rc` for sharing subtrees | `Box` owns exclusively; use `Rc<Expr>` for sharing |
