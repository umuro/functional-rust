# Comparison: Functor Laws

## Identity Law

**OCaml:**
```ocaml
let id x = x

(* map id x = x *)
assert (map id (Just 42) = Just 42);
assert (map id Nothing = Nothing);
```

**Rust:**
```rust
fn identity<T>(x: T) -> T { x }

// map(id, x) == x
assert_eq!(Maybe::Just(42).map(identity), Maybe::Just(42));
assert_eq!(Maybe::Nothing.map(identity), Maybe::Nothing);
```

## Composition Law

**OCaml:**
```ocaml
let compose f g x = f (g x)
let f x = x * 2
let g x = x + 3

(* map (f . g) x = map f (map g x) *)
assert (map (compose f g) (Just 5) = map f (map g (Just 5)));
```

**Rust:**
```rust
let f = |x: i32| x * 2;
let g = |x: i32| x + 3;

// map(f∘g, x) == map(f, map(g, x))
let x = Maybe::Just(5);
assert_eq!(x.clone().map(|v| f(g(v))), x.map(g).map(f));
```

## Bad Functor (Law Violation)

**OCaml:**
```ocaml
module BadFunctor = struct
  type 'a t = Bad of 'a * int
  let map f (Bad (x, count)) = Bad (f x, count + 1)
  (* map id (Bad(x, 0)) = Bad(x, 1) ≠ Bad(x, 0) *)
end
```

**Rust:**
```rust
struct BadFunctor<T> { value: T, map_count: usize }

impl<T> BadFunctor<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> BadFunctor<U> {
        BadFunctor { value: f(self.value), map_count: self.map_count + 1 }
        // Identity law violated: map_count changes!
    }
}
```
