# OCaml vs Rust: Currying, Partial Application, and Sections

## Side-by-Side Code

### OCaml — partial application (automatic)
```ocaml
let add x y = x + y
let add5 = add 5        (* partial application: add5 : int -> int *)

let double    = ( * ) 2
let increment = ( + ) 1
let halve     = Fun.flip ( / ) 2

let scale_and_shift ~scale ~shift x = x * scale + shift
let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)
```

### Rust (idiomatic) — partial application via closures
```rust
fn partial_add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
let add5 = partial_add(5);   // specialised adder

fn double(x: i32) -> i32    { x * 2 }
fn increment(x: i32) -> i32 { x + 1 }
fn halve(x: i32) -> i32     { x / 2 }

fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}
let celsius_of_fahrenheit = scale_and_shift(5, -160);
```

### OCaml — generic curry/uncurry
```ocaml
let curry   f x y = f (x, y)
let uncurry f (x, y) = f x y
```

### Rust — generic curry/uncurry
```rust
// curry: (A,B)->C  →  A -> Box<dyn Fn(B)->C>
fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    A: Clone + 'static, B: 'static, C: 'static,
    F: Fn((A, B)) -> C + Clone + 'static,
{
    move |a: A| {
        let f = f.clone();
        Box::new(move |b: B| f((a.clone(), b)))
    }
}

// uncurry: (A -> B -> C) → (A,B) -> C
fn uncurry<A, B, C, F, G>(f: F) -> impl Fn((A, B)) -> C
where
    F: Fn(A) -> G,
    G: Fn(B) -> C,
{
    move |(a, b)| f(a)(b)
}
```

### Pipeline comparison
```ocaml
(* OCaml *)
let pipeline = [double; increment; halve]
let result = List.fold_left (fun acc f -> f acc) 6 pipeline
(* 6 → 12 → 13 → 6 *)
```
```rust
// Rust
let result = apply_pipeline(6, &[double, increment, halve]);
// 6 → 12 → 13 → 6
fn apply_pipeline(init: i32, pipeline: &[fn(i32) -> i32]) -> i32 {
    pipeline.iter().fold(init, |acc, f| f(acc))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Curried add | `val add : int -> int -> int` | `fn partial_add(x: i32) -> impl Fn(i32) -> i32` |
| Partial application | `let add5 = add 5` | `let add5 = partial_add(5);` |
| Tupled add | `val add_tup : int * int -> int` | `fn add_tup((x,y): (i32,i32)) -> i32` |
| curry | `('a*'b->'c) -> 'a -> 'b -> 'c` | `fn curry<A,B,C,F>(f:F) -> impl Fn(A)->Box<dyn Fn(B)->C>` |
| uncurry | `('a->'b->'c) -> 'a*'b -> 'c` | `fn uncurry<A,B,C,F,G>(f:F) -> impl Fn((A,B))->C` |
| Operator section | `( * ) 2 : int -> int` | `fn double(x: i32) -> i32 { x * 2 }` |
| Flip section | `Fun.flip ( / ) 2` | `fn halve(x: i32) -> i32 { x / 2 }` |
| Labeled partial | `scale_and_shift ~scale:5 ~shift:(-160)` | `scale_and_shift(5, -160)` |

## Key Insights

1. **Auto-curry vs explicit closure:** OCaml curries every function for free;
   Rust requires writing the closure explicitly. The intent is the same but
   Rust makes the allocation and capture visible.

2. **`Box<dyn Fn>` for generic curry:** Returning `impl Fn` from inside a
   `move` closure is not stable in Rust without boxing, because the inner
   closure has an unnameable type. `Box<dyn Fn(B) -> C>` is the idiomatic
   workaround and makes the heap allocation explicit.

3. **`Fun.flip` vs argument order:** OCaml's `Fun.flip ( / ) 2` fixes 2 as
   the *divisor* (right argument). Rust achieves this by just writing
   `|x| x / 2` — no combinator needed because the argument order is already
   explicit in the closure body.

4. **Labeled arguments vs positional:** OCaml's `~scale ~shift` labels let
   callers supply arguments in any order. Rust has no labeled arguments;
   the same effect comes from positional parameters plus a returned closure.

5. **`fn` pointers vs closures in slices:** OCaml's `[double; increment; halve]`
   is a list of first-class functions. Rust's `&[fn(i32) -> i32]` holds
   function *pointers* (not closures), which is why `double`, `increment`,
   and `halve` are declared as `fn` items rather than `let` bindings with
   `impl Fn` types — function items coerce to `fn` pointers, closures do not.

## When to Use Each Style

**Use idiomatic Rust (`move` closure returning `impl Fn`)** when you need
partial application within a single codebase and performance matters — zero
heap allocation, monomorphised types, no virtual dispatch.

**Use `Box<dyn Fn>` (as in `curry`)** when you need a generic adapter that
works with arbitrary function types at runtime, or when you must store
heterogeneous closures in a collection.
