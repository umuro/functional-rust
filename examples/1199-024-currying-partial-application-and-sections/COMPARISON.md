# OCaml vs Rust: Currying, Partial Application, and Operator Sections

## Side-by-Side Code

### OCaml

```ocaml
(* Every OCaml function is automatically curried *)
let add x y = x + y
let add5 = add 5             (* partial application — zero syntax *)

(* Tupled style requires explicit destructuring *)
let add_tup (x, y) = x + y

(* curry / uncurry bridge the two styles *)
let curry   f x y = f (x, y)
let uncurry f (x, y) = f x y

(* Operator sections fix one operand of an infix operator *)
let double    = ( * ) 2
let increment = ( + ) 1
let halve     = Fun.flip ( / ) 2   (* flip swaps args: halve x = x / 2 *)

(* Labeled args allow partial application in any order *)
let scale_and_shift ~scale ~shift x = x * scale + shift
let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)
```

### Rust (idiomatic)

```rust
// Rust functions are NOT curried — return a closure explicitly
fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
let add5 = add(5);   // add5: impl Fn(i32) -> i32

// Tupled add — identical concept, just tuple destructuring in the arg
fn add_tupled((x, y): (i32, i32)) -> i32 { x + y }

// Operator section factories
fn multiply(n: i32) -> impl Fn(i32) -> i32 { move |x| x * n }
fn divide_by(n: i32) -> impl Fn(i32) -> i32 { move |x| x / n }

let double    = multiply(2);
let increment = add(1);
let halve     = divide_by(2);   // Fun.flip ( / ) 2 — arg order explicit

// Labeled-arg partial application: fixed order, same result
fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}
let celsius_of_fahrenheit = scale_and_shift(5, -160);
```

### Rust (generic curry / uncurry)

```rust
// curry: tupled fn → curried fn
// Box<dyn Fn> needed because stable Rust can't write `impl Fn(A) -> impl Fn(B) -> C`
fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    F: Fn((A, B)) -> C + Copy + 'static,
    A: Copy + 'static,
    B: 'static,
    C: 'static,
{
    move |x: A| Box::new(move |y: B| f((x, y)))
}

// uncurry: curried fn → tupled fn
fn uncurry<A, B, C>(
    f: impl Fn(A) -> Box<dyn Fn(B) -> C> + 'static,
) -> impl Fn((A, B)) -> C {
    move |(x, y)| f(x)(y)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Curried add | `val add : int -> int -> int` | `fn add(x: i32) -> impl Fn(i32) -> i32` |
| Partial application | `let add5 = add 5` (type: `int -> int`) | `let add5 = add(5)` (type: `impl Fn(i32) -> i32`) |
| Tupled add | `val add_tup : int * int -> int` | `fn add_tupled((x,y): (i32,i32)) -> i32` |
| Operator section | `( * ) 2 : int -> int` | `multiply(2)` returns `impl Fn(i32) -> i32` |
| Flip | `Fun.flip ( / ) 2 : int -> int` | `divide_by(2)` — closure order explicit |
| Generic curry | `val curry : ('a * 'b -> 'c) -> 'a -> 'b -> 'c` | `fn curry<A,B,C,F>(f:F) -> impl Fn(A) -> Box<dyn Fn(B)->C>` |
| Generic uncurry | `val uncurry : ('a -> 'b -> 'c) -> 'a * 'b -> 'c` | `fn uncurry<A,B,C>(f: impl Fn(A)->Box<dyn Fn(B)->C>+'static) -> impl Fn((A,B))->C` |

## Key Insights

1. **Automatic vs explicit currying:** OCaml makes every multi-arg function a closure chain for free. In Rust, currying is opt-in: you write `fn f(x: T) -> impl Fn(U) -> V` and return a `move` closure. The intent is just as clear, but the annotation cost is higher.

2. **`impl Fn` vs `Box<dyn Fn>`:** `impl Fn(T) -> U` works perfectly when the compiler can infer the concrete closure type at the call site. Once the type must cross a boundary — stored in a struct, returned through a generic function, or used as a trait object — `Box<dyn Fn(T) -> U>` is the idiomatic choice. OCaml never needs this distinction.

3. **Nested RPIT is not stable:** Rust cannot yet write `fn curry<A,B,C,F>(f:F) -> impl Fn(A) -> impl Fn(B) -> C` on stable (issue #99697). The inner `impl Fn` inside a `Fn` bound is not yet supported. `Box<dyn Fn>` is the stable workaround; `F: Copy` lets us avoid `Arc`/`Rc` for function items.

4. **`Fun.flip` vs closure argument reorder:** OCaml has a higher-order `flip` combinator. In Rust, we simply capture the desired operand directly: `divide_by(n)` captures `n` as the divisor and accepts the dividend as the closure arg. No `flip` is needed — the closure naturally expresses the argument order.

5. **Labeled arguments:** OCaml's `~label` syntax lets you apply any subset of arguments in any order. Rust has no equivalent. The idiom is to curry in a fixed order, or use a builder struct when argument naming is critical.

## When to Use Each Style

**Use `impl Fn` return:** When writing a simple factory or partial application that stays within a single function's scope and the concrete type doesn't need to be stored.

**Use `Box<dyn Fn>` return:** When the closure must be stored in a `Vec`, returned from a generic function, or composed through multiple layers where the concrete type is unknown.

**Use a struct with `Fn` bound:** When partial application involves many parameters, named fields improve clarity (analogous to OCaml labeled args).
