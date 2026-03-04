# 622: Classy Optics (Typeclass-Style)

**Difficulty:** 5  **Level:** Master

Express optics as traits so functions can declare "I work on any type with a `name` field" without knowing the concrete type.

## The Problem This Solves

Concrete optics (`Lens<User, String>`) bind you to a specific struct. But a logging function that needs a `user_id` field shouldn't care whether the struct is `HttpRequest`, `GrpcRequest`, or `WebSocketEvent`. In Haskell, this is the `HasX` typeclass pattern: `HasUserId s => s -> String`. Any struct that implements `HasUserId` works, and the compiler resolves the lens at the call site.

Without classy optics, you either: (1) pass concrete lenses as parameters — boilerplate, hard to compose; (2) use a shared trait that returns the field directly — works but is just normal traits, loses the get/set/modify composability; or (3) duplicate the function for each struct — obviously wrong.

Classy optics combine traits and optics: define a trait with a `lens()` method that returns the optic. Any struct implements the trait. Generic code bounds on `HasField` and uses `.lens()` to get/set/modify. The result is Haskell-style polymorphic access with Rust's zero-cost trait dispatch.

## The Intuition

Classy optics are traits where the associated method returns an optic — so any function can say "I need a type that has a `name` lens" via a trait bound, and the concrete struct provides the lens implementation, giving you polymorphic field access with full `get`/`set`/`modify` composability. The trade-off: more setup than concrete lenses, but functions become reusable across unrelated struct types.

## How It Works in Rust

```rust
// A concrete lens type
struct Lens<S, A> {
    get: fn(&S) -> A,
    set: fn(S, A) -> S,
}

impl<S, A: Clone> Lens<S, A> {
    fn view<'a>(&self, s: &'a S) -> A where A: 'a { (self.get)(s) }
    fn over(&self, s: S, f: impl FnOnce(A) -> A) -> S {
        let a = (self.get)(&s);
        (self.set)(s, f(a))
    }
}

// "Classy" trait: any type that has a `name` field
trait HasName {
    fn name_lens() -> Lens<Self, String> where Self: Sized;
    fn get_name(&self) -> String { (Self::name_lens().get)(self) }
}

// Two unrelated structs both implement HasName
struct User  { name: String, email: String }
struct Group { name: String, members: usize }

impl HasName for User {
    fn name_lens() -> Lens<Self, String> {
        Lens {
            get: |u| u.name.clone(),
            set: |mut u, n| { u.name = n; u },
        }
    }
}

impl HasName for Group {
    fn name_lens() -> Lens<Self, String> {
        Lens {
            get: |g| g.name.clone(),
            set: |mut g, n| { g.name = n; g },
        }
    }
}

// Generic function — works on ANY HasName type
fn uppercase_name<T: HasName>(thing: T) -> T {
    let lens = T::name_lens();
    lens.over(thing, |n| n.to_uppercase())
}

// Polymorphic call site — compiler resolves the right lens
let user  = User  { name: "alice".into(), email: "a@x.com".into() };
let group = Group { name: "admins".into(), members: 5 };

let u2 = uppercase_name(user);   // User::name_lens() used
let g2 = uppercase_name(group);  // Group::name_lens() used
```

## What This Unlocks

- **`Has*` trait polymorphism**: write `fn log_request<R: HasRequestId + HasUserId>(r: &R)` — works on any request type in your system.
- **Optics libraries**: `lens-rs` and similar crates use this pattern — derive `Lens` on structs, then use trait bounds to write generic transformers.
- **Cross-cutting concerns**: authentication, logging, metrics — all need user IDs or correlation IDs from various types. Classy optics let you write the concern once and apply it everywhere.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `HasX` typeclass | Module type / functor with accessor | Trait with `lens()` method |
| Polymorphic lens | `module type HasName = sig val name : (t, string) Lens.t end` | `trait HasName { fn name_lens() -> Lens<Self, String> }` |
| Generic function | `module type HasName => S -> S` | `fn f<T: HasName>(t: T) -> T` |
| Type inference | Good in OCaml 5 | Excellent — monomorphized at call site |
| Composition | Functor / module application | Trait + blanket `impl` for composed bounds |
| Zero-cost | GC overhead | Monomorphized — no runtime dispatch needed |
