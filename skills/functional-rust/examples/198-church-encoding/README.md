# 198: Church Encoding — Numbers and Booleans as Functions

**Difficulty:** 4  **Level:** Expert

Build all data — numbers, booleans, pairs — from nothing but functions. No integers, no `bool`, no structs.

## The Problem This Solves

This is the deepest question in computing: what is the *minimum* you need to do computation?

Turing machines need tape and states. Von Neumann machines need registers and memory. But Alonzo Church proved in 1936 that you need *only* functions. Not integers. Not booleans. Not data structures. Just functions — nothing else. From functions alone, you can build everything.

This is called lambda calculus, and every programming language — including Rust — is built on top of it. When you write a closure in Rust, you're writing a lambda expression. When you compose functions, you're doing lambda calculus. Church encoding is the proof that this is sufficient: it shows you how to *build* integers and booleans out of pure functions.

In practice, you'd never write production code this way. But understanding it answers the question "why do closures feel so powerful?" Because closures are the primitive — everything else is built on them. This example exists to show you that full picture.

## The Intuition

**Numbers**: The number N IS the function "apply f N times to x."

```
zero  = apply f 0 times to x = return x
one   = apply f 1 time  to x = return f(x)
two   = apply f 2 times to x = return f(f(x))
three = apply f 3 times to x = return f(f(f(x)))
```

To read a number: plug in `f = |x| x+1` and `x = 0`. Count the applications.

**Booleans**: `true` and `false` are functions that *choose* between two values.

```
true  = take two things, return the FIRST
false = take two things, return the SECOND
```

`if` is then just: "apply the condition to the two branches."

**Pairs**: A pair (a, b) is a function that, given a "selector", returns what you selected.

```
pair(a, b) = a function that, when you say "give me first", gives a
                           and when you say "give me second", gives b
```

All of these look like magic until you see them in code.

## How It Works in Rust

Rust complicates pure Church encoding because closures each have unique types. We use `Rc<dyn Fn>` for shared, type-erased closures.

**Church numerals:**
```rust
use std::rc::Rc;

type FnI = Rc<dyn Fn(i32) -> i32>;
// A Church numeral: given f and x, apply f N times to x
type Church = Rc<dyn Fn(FnI) -> FnI>;

fn church(n: u32) -> Church {
    Rc::new(move |f: FnI| {
        // Build up the N-application chain
        let mut result: FnI = Rc::new(|x| x);  // start: identity (zero applications)
        for _ in 0..n {
            let prev = result.clone();
            let f2 = f.clone();
            result = Rc::new(move |x| f2(prev(x)));  // wrap: one more application
        }
        result
    })
}

// Convert to integer by applying "add 1" to 0:
fn to_int(n: Church) -> i32 {
    let succ: FnI = Rc::new(|x| x + 1);
    n(succ)(0)  // apply "add 1" N times to 0
}

// Arithmetic falls out naturally:
fn add(m: Church, n: Church) -> Church {
    Rc::new(move |f: FnI| {
        let mf = m(f.clone());  // apply f m times...
        let nf = n(f);           // ...then n more times
        Rc::new(move |x| mf(nf(x)))
    })
}

fn mul(m: Church, n: Church) -> Church {
    // m × n: apply n's function, m times
    Rc::new(move |f: FnI| m(n(f)))
}

assert_eq!(to_int(add(church(2), church(3))), 5);
assert_eq!(to_int(mul(church(3), church(4))), 12);
```

**Church booleans — selectors:**
```rust
// A Church bool takes two values and returns one of them
type CBool = Rc<dyn Fn(i32) -> Rc<dyn Fn(i32) -> i32>>;

fn ctrue() -> CBool {
    Rc::new(|t| Rc::new(move |_f| t))  // return FIRST argument
}

fn cfalse() -> CBool {
    Rc::new(|_t| Rc::new(move |f| f))  // return SECOND argument
}

// If-then-else is just application!
fn cif(b: CBool, t: i32, f: i32) -> i32 {
    b(t)(f)  // the boolean IS the if-then-else
}

assert_eq!(cif(ctrue(), 42, 0), 42);   // true selects first branch
assert_eq!(cif(cfalse(), 42, 0), 0);   // false selects second branch
```

**Church pairs:**
```rust
type Pair = Rc<dyn Fn(Rc<dyn Fn(i32, i32) -> i32>) -> i32>;

// A pair IS a function: given "how to combine a and b", produce the result
fn pair(a: i32, b: i32) -> Pair {
    Rc::new(move |f: Rc<dyn Fn(i32, i32) -> i32>| f(a, b))
}

fn fst(p: &Pair) -> i32 { p(Rc::new(|a, _| a)) }  // select first
fn snd(p: &Pair) -> i32 { p(Rc::new(|_, b| b)) }  // select second

let p = pair(10, 20);
assert_eq!(fst(&p), 10);
assert_eq!(snd(&p), 20);
```

## What This Unlocks

- **Lambda calculus foundation**: You can now read type theory papers, proofs of Turing completeness, and the foundations of programming language semantics. Church encoding IS the proof that lambda calculus is complete.
- **Deriving pattern matching**: `cif` above is not "if-then-else" built into the language — it's just a function call. This shows that pattern matching (`match`) is syntactic sugar for function application. Scott encoding (example 199) formalizes this insight.
- **Understand why `Fn`, `FnOnce`, `FnMut` exist**: Once you see that "a number IS a function," you see that Rust's closure traits are the type-system machinery for this whole edifice. The three traits exist because of ownership — and ownership was invented precisely for the kinds of sharing problems Church encoding exposes.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Church numeral type | `('a->'a)->'a->'a` (rank-2 polymorphic) | `Rc<dyn Fn(Rc<dyn Fn(i32)->i32>)->Rc<dyn Fn(i32)->i32>>` |
| Boolean as function | `let ctrue t _ = t` (trivial) | `Rc<dyn Fn(i32)->Rc<dyn Fn(i32)->i32>>` |
| Sharing closures | Free (GC) | `Rc` for shared ownership |
| Power (exponentiation) | `let pow m n = n m` (one line!) | Requires iterative simulation |
| Type inference | Figures it all out | Needs explicit type aliases |
| Practicality | Educational | Educational |
