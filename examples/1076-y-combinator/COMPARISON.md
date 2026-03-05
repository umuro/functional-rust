# OCaml vs Rust: Y Combinator — Anonymous Recursion

## Side-by-Side Code

### OCaml
```ocaml
type 'a fix = Fix of ('a fix -> 'a)

let y f =
  let g (Fix x as w) = f (fun a -> x w a) in
  g (Fix g)

let factorial = y (fun self n ->
  if n = 0 then 1 else n * self (n - 1))
```

### Rust (idiomatic — Rc<RefCell>)
```rust
pub fn y<A: Copy + 'static, R: 'static>(
    f: impl Fn(&dyn Fn(A) -> R, A) -> R + 'static,
) -> Box<dyn Fn(A) -> R> {
    let holder: Rc<RefCell<Option<Rc<dyn Fn(A) -> R>>>> = Rc::new(RefCell::new(None));
    let f = Rc::new(f);
    let holder_clone = Rc::clone(&holder);
    let closure: Rc<dyn Fn(A) -> R> = Rc::new(move |a: A| -> R {
        let self_fn = Rc::clone(holder_clone.borrow().as_ref().unwrap());
        drop(holder_clone.borrow()); // release before recursing
        f(&*self_fn, a)
    });
    *holder.borrow_mut() = Some(Rc::clone(&closure));
    Box::new(move |a: A| closure(a))
}

let factorial = y(|self_fn: &dyn Fn(u64) -> u64, n: u64| {
    if n == 0 { 1 } else { n * self_fn(n - 1) }
});
```

### Rust (trait-based — zero heap allocation)
```rust
pub trait Recursive<A, R> {
    fn call(&self, arg: A) -> R;
}

pub struct RecFn<F>(pub F);

impl<A: Copy, R, F> Recursive<A, R> for RecFn<F>
where F: Fn(&dyn Fn(A) -> R, A) -> R {
    fn call(&self, arg: A) -> R {
        (self.0)(&|a| self.call(a), arg)
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Y combinator | `val y : (('a -> 'b) -> 'a -> 'b) -> 'a -> 'b` | `fn y<A, R>(f: impl Fn(&dyn Fn(A)->R, A)->R) -> Box<dyn Fn(A)->R>` |
| Recursive wrapper | `type 'a fix = Fix of ('a fix -> 'a)` | `struct Fix { f: Box<dyn Fn(&Fix, A) -> R> }` |
| Self-reference | Pattern match on `Fix x` | `Rc<RefCell<Option<Rc<dyn Fn>>>>` |
| Function type | `'a -> 'b` (first-class) | `dyn Fn(A) -> R` (trait object) |

## Key Insights

1. **OCaml's algebraic types make recursive types trivial** — `type 'a fix = Fix of ('a fix -> 'a)` is a one-liner. Rust needs pointer indirection (`Box`/`Rc`) to break the infinite size.
2. **Rust's ownership prevents natural cyclic references** — the `Rc<RefCell<Option<...>>>` pattern is the idiomatic workaround for creating self-referencing closures.
3. **The trait-based approach is uniquely Rust** — by using `&dyn Fn` in the trait method, we get stack-based self-reference without any heap allocation.
4. **OCaml's GC vs Rust's Rc** — OCaml's garbage collector handles the cyclic reference between `Fix` and the closure transparently. Rust's `Rc` requires explicit setup.
5. **Both languages need type-level tricks** — OCaml needs the `Fix` wrapper to satisfy the type checker; Rust needs trait objects to erase the recursive type.

## When to Use Each Style

**Use idiomatic Rust (Rc<RefCell>) when:** You need a heap-allocated recursive closure that can be stored, passed around, and called multiple times from different contexts.
**Use trait-based Rust when:** You want zero-cost abstraction and the recursive function is used locally — the `RecFn` approach avoids all heap allocation.
