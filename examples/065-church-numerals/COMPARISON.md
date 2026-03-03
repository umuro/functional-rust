# Church Numerals — OCaml vs Rust Comparison

## Core Insight

Church numerals are the purest test of first-class function support. OCaml's uniform value representation makes them trivially expressible — `let zero _f x = x` is all you need. Rust's ownership model creates significant friction: closures capture by move/borrow, have unique types, and can't be easily composed without `Box<dyn Fn>` and `Rc`.

## OCaml Approach

Elegantly minimal. Functions are first-class values with uniform representation — all the same size on the heap. Composition (`mul m n f = m (n f)`) is just partial application. Type inference handles everything automatically. This is where ML languages truly shine.

## Rust Approach

Two alternatives: (1) `Box<dyn Fn>` closures with `Rc` for shared ownership — works but verbose, allocates heavily, and fights the borrow checker. (2) An ADT-based `ChurchNum` enum (Zero | Succ) — more idiomatic Rust, explicit, and easier to reason about ownership. The ADT approach is essentially Peano numbers.

## Comparison Table

| Aspect        | OCaml                          | Rust                                    |
|---------------|--------------------------------|-----------------------------------------|
| **Memory**    | GC'd closures (uniform)       | `Box<dyn Fn>` + `Rc` (heap alloc each) |
| **Null safety** | N/A                         | N/A                                     |
| **Errors**    | N/A                           | Ownership errors at compile time        |
| **Iteration** | Partial application            | Must clone/Rc-share closures            |
| **Ergonomics**| 1-line definitions             | 10+ lines with type annotations         |

## Things Rust Learners Should Notice

1. **Closure types are unique** — you can't name them, must use `dyn Fn` trait objects
2. **`Rc` for shared ownership** — when multiple closures need the same captured value
3. **ADT alternative is more Rusty** — Peano `Zero | Succ(Box<N>)` avoids closure complexity
4. **This is where GC shines** — OCaml's garbage collector makes higher-order functions effortless
5. **Trade-off clarity** — Rust makes the cost of abstraction visible; OCaml hides it

## Further Reading

- [Closures in Rust](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Box<dyn Fn>](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [Church encoding (Wikipedia)](https://en.wikipedia.org/wiki/Church_encoding)
