# OCaml vs Rust: Currying, Partial Application, and Sections

## Side-by-Side Code

### OCaml
```ocaml
let add x y = x + y
let add5 = add 5

let add_tup (x, y) = x + y

let curry f x y = f (x, y)
let uncurry f (x, y) = f x y

let double = ( * ) 2
let increment = ( + ) 1
let halve = Fun.flip ( / ) 2

let scale_and_shift ~scale ~shift x = x * scale + shift
let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)
```

### Rust (idiomatic)
```rust
/// Adds two integers (idiomatic Rust closure).
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Partially applies `add` to `5`.
pub fn add5() -> impl Fn(i32) -> i32 {
    |y| add(5, y)
}

/// Tupled version of add (takes a tuple).
pub fn add_tup(pair: (i32, i32)) -> i32 {
    pair.0 + pair.1
}

/// Operator section: multiply by 2.
pub fn double() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

/// Operator section: increment by 1.
pub fn increment() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

/// Operator section: halve (integer division by 2).
pub fn halve() -> impl Fn(i32) -> i32 {
    |x| x / 2
}

/// Labeled‑style function with named parameters.
pub fn scale_and_shift(scale: i32, shift: i32) -> impl Fn(i32) -> i32 {
    move |x| x * scale + shift
}

/// Celsius to Fahrenheit conversion using partial application.
pub fn celsius_of_fahrenheit() -> impl Fn(i32) -> i32 {
    scale_and_shift(5, -160)
}
```

### Rust (functional/curried)
```rust
/// Curried addition (returns a closure).
pub fn add_curried(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// Curried multiplication (operator section).
pub fn double_curried() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

/// Curried increment.
pub fn increment_curried() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

/// Curried halve with flipped division.
pub fn halve_curried() -> impl Fn(i32) -> i32 {
    |x| x / 2
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val add : int -> int -> int` | `fn add(i32, i32) -> i32` |
| Partial application | `val add5 : int -> int` | `fn add5() -> impl Fn(i32) -> i32` |
| Tupled function | `val add_tup : int * int -> int` | `fn add_tup((i32, i32)) -> i32` |
| Operator section | `val double : int -> int` | `fn double() -> impl Fn(i32) -> i32` |
| Labeled arguments | `scale_and_shift : scale:int -> shift:int -> int -> int` | `fn scale_and_shift(i32, i32) -> impl Fn(i32) -> i32` |

## Key Insights

1. **Currying is explicit in Rust:** OCaml's automatic currying is a language feature; Rust requires explicit closure chains. This makes Rust's function signatures simpler (multi‑argument) but reduces the convenience of partial application.

2. **Ownership and borrowing:** Rust's closures capture their environment by reference or by value (using `move`). This gives fine‑grained control over memory but adds complexity compared to OCaml's garbage‑collected closures.

3. **Type system difference:** OCaml's type inference can handle curried functions elegantly; Rust's trait system allows returning `impl Fn` to hide the exact closure type, providing similar abstraction.

4. **Idiom difference:** Idiomatic Rust prefers multi‑argument functions and explicit partial application when needed, while OCaml leverages currying for composition and pipeline styles. Both are expressive but reflect different language philosophies.