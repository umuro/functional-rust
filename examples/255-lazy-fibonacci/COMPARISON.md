# OCaml vs Rust: Lazy Fibonacci Stream

## Side-by-Side Code

### OCaml

```ocaml
type 'a stream = Cons of 'a * (unit -> 'a stream)

let rec fibs a b = Cons (a, fun () -> fibs b (a + b))

let rec take n (Cons (x, rest)) =
  if n = 0 then []
  else x :: take (n - 1) (rest ())

let () =
  let fib_stream = fibs 0 1 in
  List.iter (Printf.printf "%d ") (take 10 fib_stream)
```

### Rust (idiomatic — Iterator)

```rust
pub struct FibIter { a: u64, b: u64 }

impl Iterator for FibIter {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let value = self.a;
        let next_b = self.a + self.b;
        self.a = self.b;
        self.b = next_b;
        Some(value)
    }
}

// Usage: FibIter { a: 0, b: 1 }.take(10).collect::<Vec<_>>()
```

### Rust (functional — thunk-based, mirrors OCaml)

```rust
pub struct Stream<T> {
    pub head: T,
    tail: Box<dyn Fn() -> Stream<T>>,  // the thunk
}

pub fn fibs_stream(a: u64, b: u64) -> Stream<u64> {
    Stream {
        head: a,
        tail: Box::new(move || fibs_stream(b, a + b)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust (Iterator) | Rust (Stream) |
|---------|-------|-----------------|---------------|
| Stream type | `'a stream` | `impl Iterator<Item=u64>` | `Stream<u64>` |
| Infinite generator | `val fibs : int -> int -> int stream` | `FibIter::new(u64, u64) -> FibIter` | `fn fibs_stream(u64, u64) -> Stream<u64>` |
| Take prefix | `val take : int -> 'a stream -> 'a list` | `.take(n).collect::<Vec<_>>()` | `stream.take(n) -> Vec<u64>` |
| Thunk type | `unit -> 'a stream` | N/A (implicit in `next`) | `Box<dyn Fn() -> Stream<T>>` |

## Key Insights

1. **`Iterator` is Rust's stream abstraction.** OCaml's `stream` type is a
   library-level concept; Rust bakes lazy sequences into the language via the
   `Iterator` trait.  The entire `std` ecosystem (`.map`, `.filter`, `.take`,
   `.zip`, ...) composes freely with any `Iterator`, including infinite ones.

2. **Recursive types require `Box`.** OCaml's GC tracks object sizes at runtime,
   so `type 'a stream = Cons of 'a * (unit -> 'a stream)` compiles fine.  In
   Rust every type must have a statically-known size.  `Stream<T>` contains a
   `Box<dyn Fn()...>` (pointer — fixed 16 bytes) rather than `Stream<T>` itself
   (infinite size), making the type representable.

3. **`move` closures replace GC-managed capture.** OCaml closures automatically
   capture variables from the enclosing scope via the GC.  Rust's `move` keyword
   transfers ownership of `a` and `b` into the closure, making it `'static` and
   independent of any stack frame — a necessary trade-off for memory safety
   without GC.

4. **No-allocation iteration is possible.** The `FibIter` approach stores only
   two `u64` values on the stack.  OCaml's `stream` allocates a `Cons` cell on
   the heap for every element produced.  Rust lets you choose the trade-off:
   use the Iterator for zero allocation, or the `Stream` type to mirror OCaml's
   structure exactly.

5. **Composability.** Because `FibIter` implements `Iterator`, you can write
   `FibIter::new(0,1).filter(|x| x % 2 == 0).take(5).sum::<u64>()` with no
   extra code.  The thunk-based `Stream` would need manual implementations of
   each combinator.

## When to Use Each Style

**Use idiomatic Rust (Iterator) when:** you want zero-allocation lazy sequences
that compose naturally with the rest of `std`, which is almost always.

**Use thunk-based Rust (Stream) when:** you are teaching the OCaml→Rust
translation, need an explicit coinductive structure (e.g. for tree streams), or
want to experiment with custom stream combinators that diverge from `Iterator`.
