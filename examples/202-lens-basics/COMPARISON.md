# Comparison: Example 202 — Lens Basics

## Lens Type Definition

### OCaml
```ocaml
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

let view l s = l.get s
let set l a s = l.set a s
let over l f s = l.set (f (l.get s)) s
```

### Rust
```rust
struct Lens<S, A> {
    get: Box<dyn Fn(&S) -> A>,
    set: Box<dyn Fn(A, &S) -> S>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    fn view(&self, s: &S) -> A { (self.get)(s) }
    fn set(&self, a: A, s: &S) -> S { (self.set)(a, s) }
    fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        (self.set)(f((self.get)(s)), s)
    }
}
```

## Creating a Lens

### OCaml
```ocaml
let name_lens = {
  get = (fun p -> p.name);
  set = (fun n p -> { p with name = n });
}
```

### Rust (Closure)
```rust
fn name_lens() -> Lens<Person, String> {
    Lens::new(|p| p.name.clone(), |n, p| Person { name: n, ..p.clone() })
}
```

### Rust (Trait — zero-cost)
```rust
struct PersonName;
impl LensLike<Person, String> for PersonName {
    fn get(s: &Person) -> String { s.name.clone() }
    fn set(a: String, s: &Person) -> Person {
        Person { name: a, ..s.clone() }
    }
}
```
