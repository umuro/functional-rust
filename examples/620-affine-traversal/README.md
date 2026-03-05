📖 **[View on hightechmind.io →](https://hightechmind.io/rust/620-affine-traversal)**

---

# 620: Affine Traversal (Optional Lens)

**Difficulty:** 5  **Level:** Master

Focus on zero or one element in a structure — the composition of a `Lens` and a `Prism` for optional nested access.

## The Problem This Solves

You have nested data where some layers might not exist: `user.address?.city`. A `Lens` always succeeds (the field is always there). A `Prism` matches one enum variant (zero or one match). But when you compose them — "get the city from the address, but only if the address exists" — you need an optic that can focus on zero or one target. Neither Lens nor Prism alone expresses this.

In Haskell, this is the `AffineTraversal` or `Optional` optic. It has a `preview :: S → Option<A>` (get the target if it exists) and a `set :: S → A → S` (update the target if it exists, do nothing if it doesn't). It's strictly between Prism and Traversal in the optics hierarchy.

The practical value: deeply nested optional access is ubiquitous in real data. Without affine traversals, you either chain `.map()` operations on `Option<T>` (verbose, breaks composition) or use the `?` operator (breaks at function boundaries). An affine traversal gives you a single composable handle on "this path through the data, which may or may not exist."

## The Intuition

An affine traversal is an optic that targets zero or one element — `preview` returns `Option<A>` (might not be there), and `set`/`modify` are no-ops when the target is absent — making it the right tool for optional nested access like `user?.address?.city`. The trade-off: more expressive than Option chaining but requires understanding the optics hierarchy; start with Option chaining and introduce affine traversals when composition gets unwieldy.

## How It Works in Rust

```rust
// Affine traversal: focus on zero or one element
struct Affine<S, A> {
    preview: Box<dyn Fn(&S) -> Option<A>>,
    set:     Box<dyn Fn(S, A) -> S>,
}

impl<S: Clone + 'static, A: Clone + 'static> Affine<S, A> {
    fn new(
        preview: impl Fn(&S) -> Option<A> + 'static,
        set:     impl Fn(S, A) -> S + 'static,
    ) -> Self {
        Affine { preview: Box::new(preview), set: Box::new(set) }
    }

    fn modify(&self, s: S, f: impl Fn(A) -> A) -> S {
        if let Some(a) = (self.preview)(&s) {
            (self.set)(s, f(a))
        } else {
            s  // no-op when target absent
        }
    }

    // Compose two affine traversals: focus deeper
    fn compose<B: Clone + 'static>(self, other: Affine<A, B>) -> Affine<S, B> {
        let preview1 = self.preview;
        let preview2 = other.preview;
        let set1     = self.set;
        let set2     = other.set;
        Affine::new(
            move |s| preview1(s).and_then(|a| preview2(&a)),
            move |s, b| {
                if let Some(a) = preview1(&s) {
                    let new_a = (set2)(a, b);
                    (set1)(s, new_a)
                } else { s }
            },
        )
    }
}

// Example: optional address city
#[derive(Clone)]
struct User { address: Option<Address> }
#[derive(Clone)]
struct Address { city: String }

// Prism-like: User → Option<Address>
let user_address: Affine<User, Address> = Affine::new(
    |u| u.address.clone(),
    |mut u, a| { u.address = Some(a); u },
);

// Lens-like: Address → city (always present)
let address_city: Affine<Address, String> = Affine::new(
    |a| Some(a.city.clone()),
    |mut a, c| { a.city = c; a },
);

// Compose: User → Option<String> (city might not exist)
let user_city = user_address.compose(address_city);
```

## What This Unlocks

- **Deeply nested optional access**: `user.org?.team?.lead?.email` as a single composable optic — `preview` returns `Option`, `set` is safe no-op.
- **Enum field update**: update a field inside a specific variant — if the value is a different variant, `set` does nothing.
- **Library-level nullable navigation**: build a fluent API for navigating optional nested structures without `if let` chains.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Affine traversal | `preview` + `set` where `set` may be no-op | `struct Affine<S, A>` with `preview` + `set` |
| `preview` | `s -> 'a option` | `Fn(&S) -> Option<A>` |
| Lens vs Affine | Lens always succeeds | Affine may return `None` |
| Prism vs Affine | Prism: variant match | Affine: may include field access after match |
| Composition | `Lens ∘ Prism` or `Prism ∘ Lens` | `.compose()` on `Affine<S,A>` + `Affine<A,B>` |
| In optics hierarchy | Prism ⊆ Affine ⊆ Traversal | Same hierarchy |
