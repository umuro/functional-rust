# OCaml vs Rust: Applicative Functor Laws

## Type Definition

### OCaml
```ocaml
module type APPLICATIVE = sig
  type 'a t
  val pure : 'a -> 'a t
  val ap : ('a -> 'b) t -> 'a t -> 'b t
end
```

### Rust
```rust
pub struct Applicative<T>(pub T);

impl<T> Applicative<T> {
    pub fn pure(value: T) -> Self {
        Applicative(value)
    }
}

impl<T: Clone> Applicative<T> {
    pub fn ap<U, F>(self, f: Applicative<F>) -> Applicative<U>
    where
        F: FnOnce(T) -> U,
    {
        Applicative(f.0(self.0))
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Higher-kinded types | Native support via module system | Workaround with traits/generics |
| Type classes | Module functors | Traits (no HKT) |
| Partial application | Built-in currying | Closures required |
| Law verification | Expressive types | Runtime tests |

## Identity Law

### OCaml
```ocaml
let id_law v = ap (pure Fun.id) v = v
```

### Rust
```rust
fn verify_identity<T: Clone + PartialEq>(v: Applicative<T>) -> bool {
    let id_fn = Applicative::pure(|x: T| x);
    v.clone().ap(id_fn) == v
}
```

## Notes

- OCaml's module system provides true abstraction over type constructors
- Rust requires concrete types or trait workarounds for HKT patterns
- Both can express the applicative pattern, but OCaml is more natural
