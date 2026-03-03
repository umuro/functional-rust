# Currying, Partial Application, and Sections: OCaml vs Rust

## The Core Insight
Currying is OCaml's bread and butter — every multi-argument function is actually a chain of single-argument functions. Rust made a deliberate design choice NOT to curry by default, favoring explicit closures instead. This reveals a philosophical difference: OCaml optimizes for functional composition; Rust optimizes for clarity and zero-cost abstraction.

## OCaml Approach
In OCaml, `let add x y = x + y` is syntactic sugar for `let add = fun x -> fun y -> x + y`. This means `add 5` naturally returns a function `fun y -> 5 + y`. Operator sections like `( * ) 2` partially apply multiplication. `Fun.flip` swaps argument order for operators like division. Labeled arguments (`~scale ~shift`) enable partial application in any order. This all composes beautifully for pipeline-style programming.

## Rust Approach
Rust functions take all arguments at once: `fn add(x: i32, y: i32) -> i32`. Partial application requires explicitly returning a closure: `fn add5() -> impl Fn(i32) -> i32 { |y| add(5, y) }`. The `move` keyword captures variables by value. Generic `curry`/`uncurry` converters are possible but require `Box<dyn Fn>` for the intermediate closure (due to Rust's requirement that function return types have known size). The tradeoff is more verbosity for complete control over capture and allocation.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Default | All functions curried | All functions take full args |
| Partial application | `add 5` (free) | `\|y\| add(5, y)` (closure) |
| Operator section | `( * ) 2` | `\|x\| x * 2` |
| Flip | `Fun.flip ( / ) 2` | No built-in (write closure) |
| Labeled args | `~scale ~shift` | Not available (use structs) |
| Pipeline | `List.fold_left` | `iter().fold()` or explicit |
| Return function | Natural (currying) | `impl Fn(...)` or `Box<dyn Fn(...)>` |

## What Rust Learners Should Notice
- OCaml's currying is zero-cost because the compiler knows the full type; Rust closures may allocate when boxed (`Box<dyn Fn>`) but `impl Fn` closures are monomorphized and zero-cost
- `move |y| ...` captures variables by value — essential when the closure outlives its creation scope
- Rust's lack of currying is intentional: explicit closures are clearer about what's captured and how
- `impl Fn(i32) -> i32` as a return type is Rust's way of saying "returns some closure" without boxing — it's a zero-cost abstraction
- For labeled/named arguments, Rust uses the builder pattern or struct arguments — different idiom, same result

## Further Reading
- [The Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [OCaml Higher-Order Functions](https://cs3110.github.io/textbook/chapters/hop/higher_order.html)
