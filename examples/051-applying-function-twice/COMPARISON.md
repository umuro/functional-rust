# Side-by-Side: Applying a Function Twice

## OCaml

```ocaml
let twice f x = f (f x)

let double x = 2 * x
let square x = x * x

let quad   = twice double   (* partial application — no argument given *)
let fourth = twice square

let () =
  Printf.printf "quad 3   = %d\n" (quad 3);    (* 12 *)
  Printf.printf "fourth 2 = %d\n" (fourth 2)   (* 16 *)
```

## Rust — Implementation 1: Generic (direct form)

```rust
pub fn twice<T, F>(f: F, x: T) -> T
where
    F: Fn(T) -> T,
{
    f(f(x))
}

// Usage (both arguments supplied):
twice(double, 3)   // 12
twice(square, 2)   // 16
```

## Rust — Implementation 2: Partial application (curried style)

```rust
pub fn twice_partial<T, F>(f: F) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
{
    move |x| f(f(x))  // f captured by move into the returned closure
}

// Usage — mirrors OCaml exactly:
let quad   = twice_partial(double);   // quad: impl Fn(i32) -> i32
let fourth = twice_partial(square);

quad(3)     // 12
fourth(2)   // 16
```

## Rust — Implementation 3: Function pointer variant

```rust
pub fn twice_fp(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// fn(i32) -> i32 — bare function pointer, no captured environment.
twice_fp(double, 3)   // 12
twice_fp(square, 2)   // 16
```

---

## Concept Map

| OCaml concept | Rust equivalent |
|---------------|----------------|
| Curried function `('a -> 'a) -> 'a -> 'a` | Two-argument generic fn or `impl Fn` return |
| `let quad = twice double` | `let quad = twice_partial(double)` |
| Polymorphic `'a` | Generic type parameter `T` |
| All functions implicitly curried | Closures capture by move; explicit `impl Fn(T) -> T` |
| Function value (no distinction) | `Fn` trait (closure) vs `fn` pointer |

## Why `Fn` and not `FnOnce`?

`FnOnce` means the closure can only be called once (it consumes captured values).
Here `f` is called twice inside the body, so it must be `Fn` (callable by shared
reference, any number of times).  `FnMut` would also work but is unnecessarily
broad when no mutation is needed.

## Ownership note on `twice_partial`

```rust
// f is moved into the returned closure.
// The closure itself is Fn because F: Fn — it can be called many times.
// Each call to the returned closure borrows f immutably to invoke f(f(x)).
move |x| f(f(x))
```

The intermediate value `f(x): T` is owned by the stack frame and then moved
into the second call `f(…)`.  No heap allocation is required.
