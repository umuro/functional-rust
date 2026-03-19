# OCaml vs Rust: Currying, Partial Application, and Operator Sections

## Side-by-Side Code

### OCaml

```ocaml
(* All OCaml functions are curried by default *)
let add x y = x + y
let add5 = add 5             (* partial application, no extra syntax *)

let add_tup (x, y) = x + y  (* tupled — NOT the default *)

let curry   f x y = f (x, y)
let uncurry f (x, y) = f x y

(* Operator sections *)
let double    = ( * ) 2
let increment = ( + ) 1
let halve     = Fun.flip ( / ) 2

(* Labeled args: partial application in any order *)
let scale_and_shift ~scale ~shift x = x * scale + shift
let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)
```

### Rust (idiomatic)

```rust
// Rust functions are NOT curried — partial application via closures
pub fn add(x: i32, y: i32) -> i32 { x + y }

pub fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y  // closure captures x
}

pub fn double(x: i32) -> i32 { x * 2 }
pub fn increment(x: i32) -> i32 { x + 1 }
pub fn halve(x: i32) -> i32 { x / 2 }

pub fn scale_and_shift(scale: i32, shift: i32, x: i32) -> i32 {
    x * scale + shift
}
pub fn celsius_of_fahrenheit(f: i32) -> i32 {
    scale_and_shift(5, -160, f)  // partial application by wrapping
}

// Pipeline: fold a slice of fn pointers
pub fn apply_pipeline(fns: &[fn(i32) -> i32], start: i32) -> i32 {
    fns.iter().fold(start, |acc, f| f(acc))
}
```

### Rust (functional — curry/uncurry/flip with Box<dyn Fn>)

```rust
// curry: tupled function → sequential (one arg at a time)
pub fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    F: Fn((A, B)) -> C + Clone + 'static,
    A: Clone + 'static,
    B: 'static,
    C: 'static,
{
    move |x: A| {
        let f = f.clone();
        let x = x.clone();
        Box::new(move |y: B| f((x.clone(), y)))
    }
}

// uncurry: closure-returning function → tupled
pub fn uncurry<A, B, C, G, F>(f: F) -> impl Fn((A, B)) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    move |(x, y)| f(x)(y)
}

// flip: swap first two arguments (like OCaml's Fun.flip)
pub fn flip<A, B, C, F>(f: F) -> impl Fn(B) -> Box<dyn Fn(A) -> C>
where
    F: Fn(A, B) -> C + Clone + 'static,
    A: 'static,
    B: Clone + 'static,
    C: 'static,
{
    move |b: B| {
        let f = f.clone();
        let b = b.clone();
        Box::new(move |a: A| f(a, b.clone()))
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Binary add | `val add : int -> int -> int` | `fn add(x: i32, y: i32) -> i32` |
| Partial add | `val add5 : int -> int` | `let add5: impl Fn(i32) -> i32 = make_adder(5)` |
| Tupled add | `val add_tup : int * int -> int` | `fn add_tup((x,y): (i32,i32)) -> i32` |
| curry | `val curry : ('a*'b->'c) -> 'a -> 'b -> 'c` | `fn curry<A,B,C,F>(f:F) -> impl Fn(A) -> Box<dyn Fn(B)->C>` |
| uncurry | `val uncurry : ('a->'b->'c) -> 'a*'b -> 'c` | `fn uncurry<A,B,C,G,F>(f:F) -> impl Fn((A,B))->C` |
| flip | `val flip : ('a->'b->'c) -> 'b -> 'a -> 'c` | `fn flip<A,B,C,F>(f:F) -> impl Fn(B) -> Box<dyn Fn(A)->C>` |
| Operator section | `( * ) 2 : int -> int` | `\|x\| x * 2` or `fn double(x:i32)->i32` |

## Key Insights

1. **Currying is automatic in OCaml, explicit in Rust.** Every OCaml function
   can be partially applied with no extra syntax. In Rust you must explicitly
   return a closure (`move |y| x + y`) that captures the fixed argument.

2. **`Box<dyn Fn>` for higher-order generics.** When writing generic `curry` or
   `flip`, the inner closure has an un-nameable concrete type. Returning
   `Box<dyn Fn(B) -> C>` heap-allocates it and erases the type — the cost of
   Rust's zero-overhead abstractions at this boundary. (A nightly feature
   `impl_trait_in_fn_trait_return` will eventually remove this need.)

3. **Lifetime and Clone bounds flow from ownership.** `curry` requires
   `A: Clone + 'static` and `F: Clone + 'static` because the inner `Box` must
   own its captures and those captures may be used repeatedly. OCaml's GC
   handles this transparently.

4. **Labeled arguments vs positional parameters.** OCaml's `~scale` and
   `~shift` allow calling `scale_and_shift ~shift:(-160) ~scale:5` in any
   order. Rust requires a wrapper closure to fix specific arguments of a
   positional function.

5. **Function pointers for pipelines.** OCaml's `[double; increment; halve]`
   stores closures of the same type naturally. In Rust, a `&[fn(i32)->i32]`
   slice of function pointers works when all functions are named (not closures
   with captured state); mixed cases require `Vec<Box<dyn Fn(i32)->i32>>`.

## When to Use Each Style

**Use `impl Fn` return (make_adder style) when:** creating a simple partially-
applied function where the captured type is known and the return doesn't need
to be stored generically.

**Use `Box<dyn Fn>` when:** writing generic higher-order combinators (`curry`,
`flip`) where the inner closure type cannot be named, or storing mixed closures
in a collection.
