# OCaml vs Rust: Profunctor

## Type Definition

### OCaml
```ocaml
module type PROFUNCTOR = sig
  type ('a, 'b) t
  val dimap : ('c -> 'a) -> ('b -> 'd) -> ('a, 'b) t -> ('c, 'd) t
end
```

### Rust
```rust
pub trait Profunctor<A, B> {
    type Output<C, D>;
    fn dimap<C, D, F, G>(self, f: F, g: G) -> Self::Output<C, D>
    where F: FnOnce(C) -> A, G: FnOnce(B) -> D;
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Variance | Implicit | Explicit PhantomData |
| Function composition | Direct with `.` | Closures |
| Boxing | Automatic | Required for dyn |
