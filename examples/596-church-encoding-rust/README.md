# 596: Church Encoding in Rust

**Difficulty:** 4  **Level:** Advanced

Build booleans, numbers, and pairs from pure functions — using Rust's function pointers and closures where lambda calculus uses only lambdas.

## The Problem This Solves

When you learn a programming language, you're handed its primitives: integers, booleans, strings. You take them for granted. But what if you couldn't? What if `true`, `false`, and `0` didn't exist, and you had to build them?

That's the Church encoding challenge. Alonzo Church proved in 1936 that you only need *functions* to build everything else. This isn't just a thought experiment — it's the mathematical bedrock that all of functional programming stands on. Every time you use a closure in Rust, you're using the building block Church said was sufficient.

For Rust developers specifically, Church encoding reveals something subtle: Rust's type system makes some things that are trivial in lambda calculus surprisingly complex. The friction you feel while working through this example is precisely the friction between "general lambda calculus" and "Rust's ownership + type system." Understanding where they differ makes you a sharper Rust programmer.

This example focuses on the most concrete, Rust-idiomatic version of Church encoding — using function pointers (`fn` types) instead of heap-allocated closures, which gets you close to the lambda calculus ideal without the `Box<dyn Fn>` overhead.

## The Intuition

**Church booleans**: True and false are functions that choose between two options.

```
true  = take two arguments, return the FIRST
false = take two arguments, return the SECOND
```

In Rust:
```rust
fn church_true (t: i32, _f: i32) -> i32 { t }   // pick first
fn church_false(_t: i32,  f: i32) -> i32 { f }   // pick second
```

`if` is now just function call:
```rust
fn church_if(condition: fn(i32, i32) -> i32, then: i32, else_: i32) -> i32 {
    condition(then, else_)  // the CONDITION decides which to return
}

church_if(church_true,  42, 0)  // → 42  (true picks first)
church_if(church_false, 42, 0)  // → 0   (false picks second)
```

**Church numerals**: The number N is a function that applies its argument exactly N times.

```rust
fn zero (f: fn(usize) -> usize, x: usize) -> usize { x }           // 0 applications
fn one  (f: fn(usize) -> usize, x: usize) -> usize { f(x) }        // 1 application
fn two  (f: fn(usize) -> usize, x: usize) -> usize { f(f(x)) }     // 2 applications
fn three(f: fn(usize) -> usize, x: usize) -> usize { f(f(f(x))) }  // 3 applications
```

Reading the number: apply "add 1" starting from 0.
```rust
fn to_int(n: impl Fn(fn(usize) -> usize, usize) -> usize) -> usize {
    n(|x| x + 1, 0)  // count how many times n applies the function
}

to_int(three)  // → 3
```

## How It Works in Rust

This example deliberately uses Rust `fn` pointers (not `Box<dyn Fn>`) to stay close to lambda calculus. Function pointers are zero-overhead — they're just addresses, like C function pointers.

**Booleans and logic:**
```rust
fn church_true (t: i32, _f: i32) -> i32 { t }
fn church_false(_t: i32,  f: i32) -> i32 { f }

fn church_if(c: fn(i32, i32) -> i32, t: i32, f: i32) -> i32 { c(t, f) }

fn church_not(b: fn(i32, i32) -> i32) -> fn(i32, i32) -> i32 {
    // not true = false; not false = true
    // We observe b with (1, 0) to decide, then return the opposite function
    if b(1, 0) == 1 { church_false } else { church_true }
}

fn church_and(p: fn(i32, i32) -> i32, q: fn(i32, i32) -> i32) -> bool {
    p(1, 0) == 1 && q(1, 0) == 1
}
```

**Arithmetic — addition and multiplication:**
```rust
fn church_add(
    m: fn(fn(usize) -> usize, usize) -> usize,
    n: fn(fn(usize) -> usize, usize) -> usize,
) -> impl Fn(fn(usize) -> usize, usize) -> usize {
    // m+n: apply f m times to (n applied f times to x)
    move |f, x| m(f, n(f, x))
}

fn church_mul(
    m: fn(fn(usize) -> usize, usize) -> usize,
    n: fn(fn(usize) -> usize, usize) -> usize,
) -> impl Fn(fn(usize) -> usize, usize) -> usize {
    // m×n: apply f, but each "application" is actually n applications
    move |f, x| m(|y| n(f, y), x)
}

assert_eq!(to_int(church_add(two, three)), 5);
assert_eq!(to_int(church_mul(two, three)), 6);
```

**Why `impl Fn` instead of function pointers for add/mul?**  
`church_add` returns a *closure* (it captures `m` and `n`), not a plain function pointer. Closures capture state; function pointers can't. This is one place where Rust's type system is more fine-grained than lambda calculus.

**Church pairs:**
```rust
// A pair IS a function: given a selector, it returns the selected element
fn church_pair<A: Copy, B: Copy>(a: A, b: B) -> impl Fn(fn(A, B) -> A) -> A + Copy {
    move |f| f(a, b)
}

// Note: only works for projecting the first element with this type signature
// Full pairs need more type-level machinery in Rust
```

## What This Unlocks

- **Lambda calculus in action**: The gap between `fn church_true(t: i32, f: i32) -> i32 { t }` and `λt.λf.t` is zero — you're writing lambda calculus directly in Rust syntax.
- **Zero-overhead abstraction**: Unlike the `Rc<dyn Fn>` version in example 198, this version uses `fn` pointers and `impl Fn` — potentially optimized to inline by the compiler. Church encoding doesn't have to be slow.
- **Type-level insights**: The places where this example gets awkward (pairs, polymorphism) reveal exactly where Rust's type system diverges from pure lambda calculus. These are the same places where Rust's ownership model adds precision that lambda calculus lacks.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Church true | `let ctrue t _ = t` | `fn church_true(t: i32, _f: i32) -> i32 { t }` |
| Church false | `let cfalse _ f = f` | `fn church_false(_t: i32, f: i32) -> i32 { f }` |
| `if` | `let cif b t f = b t f` | `fn church_if(c: fn(i32,i32)->i32, t: i32, f: i32) -> i32` |
| Numeral type | `('a->'a)->'a->'a` (polymorphic) | `fn(fn(usize)->usize, usize)->usize` (concrete) |
| Add/Mul return type | Same function type | `impl Fn(...)` (closure, not fn pointer) |
| Polymorphism | Automatic | Must monomorphize or use `dyn` |
| Pairs | Fully general | Type signature limits generality |
