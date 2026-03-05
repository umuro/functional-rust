# OCaml vs Rust: Affine Traversal — At Most One Focus

## Side-by-Side Code

### OCaml
```ocaml
type ('s, 'a) affine = {
  preview : 's -> 'a option;
  set     : 'a -> 's -> 's;
}

type user = { name: string; email: string option; phone: string option }

let email_affine : (user, string) affine = {
  preview = (fun u -> u.email);
  set = (fun e u -> { u with email = Some e });
}

(* over = apply f when present, no-op when absent *)
let over aff f s =
  match aff.preview s with
  | Some v -> aff.set (f v) s
  | None   -> s
```

### Rust (idiomatic — boxed closures)
```rust
pub struct Affine<S, A> {
    preview: Box<dyn Fn(&S) -> Option<A>>,
    set: Box<dyn Fn(A, &S) -> S>,
}

impl<S: Clone + 'static, A: 'static> Affine<S, A> {
    pub fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        match (self.preview)(s) {
            Some(v) => (self.set)(f(v), s),
            None    => s.clone(),
        }
    }
}

pub fn email_affine() -> Affine<User, String> {
    Affine::new(
        |u: &User| u.email.clone(),
        |e, u: &User| User { email: Some(e), ..u.clone() },
    )
}
```

### Rust (zero-cost — trait dispatch)
```rust
pub trait AffineTraversal {
    type Source: Clone;
    type Focus;

    fn preview(s: &Self::Source) -> Option<Self::Focus>;
    fn set(a: Self::Focus, s: &Self::Source) -> Self::Source;

    fn over(f: impl FnOnce(Self::Focus) -> Self::Focus, s: &Self::Source) -> Self::Source {
        match Self::preview(s) {
            Some(v) => Self::set(f(v), s),
            None    => s.clone(),
        }
    }
}

pub struct UserEmailAffine;

impl AffineTraversal for UserEmailAffine {
    type Source = User;
    type Focus  = String;

    fn preview(u: &User) -> Option<String> { u.email.clone() }
    fn set(e: String, u: &User) -> User    { User { email: Some(e), ..u.clone() } }
}
```

## Type Signatures

| Concept              | OCaml                                        | Rust (boxed)                                    |
|----------------------|----------------------------------------------|-------------------------------------------------|
| Affine type          | `('s, 'a) affine`                            | `Affine<S, A>`                                  |
| Preview              | `'s -> 'a option`                            | `Box<dyn Fn(&S) -> Option<A>>`                  |
| Set                  | `'a -> 's -> 's`                             | `Box<dyn Fn(A, &S) -> S>`                       |
| Over                 | `('a -> 'a) -> 's -> 's`                     | `fn over(&self, f: impl FnOnce(A)->A, s:&S)->S` |
| Optional value       | `'a option`                                  | `Option<A>`                                     |
| Record update syntax | `{ u with email = Some e }`                  | `User { email: Some(e), ..u.clone() }`          |

## Key Insights

1. **Record update vs struct update**: OCaml's `{ u with field = v }` compiles to a structural copy; Rust's `..u.clone()` is explicit — the `.clone()` call reminds the programmer that a heap allocation occurs. The clone is unavoidable here because the `set` closure must own its output.

2. **Two dispatch strategies**: Rust offers *boxed closures* (`Box<dyn Fn(...)>`) for runtime flexibility (the affine is a first-class value you can pass around) and *trait objects* for zero-cost compile-time dispatch. OCaml only has the record-of-closures style, which is closest to the boxed approach.

3. **Lifetime tracking**: OCaml's GC hides ownership. Rust's `preview` takes `&S` (borrow, no copy) while `set` takes an owned `A` and `&S` (borrow the whole), returning an owned `S`. The signature makes data flow explicit.

4. **`'static` bounds**: Because closures are stored inside the struct (`Box<dyn Fn>`), Rust requires `'static` lifetime on `S` and `A`. In OCaml this constraint is invisible — the GC keeps everything alive.

5. **HashMap affines**: OCaml's `Map.Make(String)` functor produces a module; Rust uses `HashMap<String, String>` directly. Both express "focus on a key" with identical logic — a closure that captures the key and either reads or replaces the entry.

## When to Use Each Style

**Use boxed-closure `Affine<S, A>` when:** you need the affine to be a runtime value — stored in a `Vec`, chosen at runtime, or returned from a factory function. The heap allocation is a one-time cost at construction.

**Use the `AffineTraversal` trait when:** the affine is known at compile time and you want zero overhead. The compiler monomorphises the `over` default method into a direct function call with no indirection.
