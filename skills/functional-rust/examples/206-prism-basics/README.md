# 206: Prism Basics — Optics for Enum Variants

**Difficulty:** ⭐⭐⭐  **Level:** Intermediate

A Prism is like a Lens for enum variants — it focuses on a variant that *might* exist, returning `Option` when the variant doesn't match.

## The Problem This Solves

You have an enum with several variants. You want to work with one variant in particular — extract its payload, modify it if it's there, or construct a value of that variant. In Rust you write `match` statements everywhere:

```rust
fn double_radius(s: &Shape) -> Shape {
    match s {
        Shape::Circle(r) => Shape::Circle(r * 2.0),
        other => other.clone(),
    }
}
```

That's fine for one transformation. But now you have five variants and seven operations — you're writing 35 `match` arms. When you add a new variant, every `match` potentially needs updating. And these `match` expressions are scattered through your codebase, each repeating the same "extract if it's this variant, do nothing otherwise" logic.

A Prism packages that logic once. It says: "I know how to extract the payload of `Shape::Circle` (if the shape is a circle), and I know how to construct a `Shape::Circle` from a radius." Everything else — the "do nothing if it's the wrong variant" logic — is handled by the Prism machinery. This example exists to solve exactly that pain.

## The Intuition

A Lens always succeeds: every `Person` has a `name`, so `name_lens.get(&person)` always returns a `String`.

A Prism might fail: not every `Shape` is a `Circle`, so `circle_prism.preview(&shape)` returns `Option<f64>`. If the shape is a circle, you get `Some(radius)`. If it's anything else, you get `None`.

A Prism has two operations (not three like a Lens):
- **`preview`**: `(&S) -> Option<A>` — try to extract the payload; returns `None` if wrong variant
- **`review`**: `(A) -> S` — construct the enum from a payload; always succeeds

```rust
struct Prism<S, A> {
    preview: Box<dyn Fn(&S) -> Option<A>>,  // extraction (may fail)
    review:  Box<dyn Fn(A) -> S>,           // construction (always works)
}
```

Think of `preview` as a type-safe downcast: "is this a `Circle`? If yes, give me the radius." Think of `review` as the variant constructor: "give me a `Circle` with this radius."

`over` — applying a function to the payload — is now conditional: it only transforms if `preview` succeeds.

## How It Works in Rust

**Define a Prism for a specific enum variant:**

```rust
fn circle_prism() -> Prism<Shape, f64> {
    Prism::new(
        |s| match s {
            Shape::Circle(r) => Some(*r),  // preview: extract if Circle
            _ => None,                      // miss: return None
        },
        |r| Shape::Circle(r),              // review: construct Circle
    )
}
```

**Use preview and review:**

```rust
let circle = Shape::Circle(5.0);
let rect   = Shape::Rectangle(3.0, 4.0);
let cp     = circle_prism();

(cp.preview)(&circle)   // Some(5.0)  — it's a circle
(cp.preview)(&rect)     // None       — it's not

(cp.review)(10.0)       // Shape::Circle(10.0)
```

**`over` — modify only if the variant matches:**

```rust
impl<S: Clone, A> Prism<S, A> {
    fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        match (self.preview)(s) {
            Some(a) => (self.review)(f(a)),  // transform and reconstruct
            None    => s.clone(),            // wrong variant — return unchanged
        }
    }
}

let c2 = cp.over(|r| r * 2.0, &circle);  // Shape::Circle(10.0)
let r2 = cp.over(|r| r * 2.0, &rect);   // Shape::Rectangle(3.0, 4.0) — unchanged
```

**Prisms for standard types (`Option`, `Result`):**

```rust
fn some_prism<T: Clone + 'static>() -> Prism<Option<T>, T> {
    Prism::new(|o| o.clone(), |a| Some(a))
}

fn ok_prism<T: Clone + 'static, E: Clone + 'static>() -> Prism<Result<T, E>, T> {
    Prism::new(|r| r.as_ref().ok().cloned(), |a| Ok(a))
}
```

These let you treat `Option` and `Result` as sum types and apply Prism operations uniformly.

## What This Unlocks

- **Variant operations without exhaustive matching**: `over` applies a function to the payload of one variant, leaving everything else unchanged — no `match` required at the call site.
- **First-class variant access**: pass a Prism as a value, use it in generic code that processes "whatever variant this is focusing on."
- **Uniform API for sum types**: `Option`, `Result`, and your own enums all have the same `preview`/`review`/`over` interface — write generic algorithms over any Prism.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sum type access | Pattern matching with `function` syntax | `match` on `&self` with `*` dereference |
| `preview` return type | `'a option` — native variant | `Option<A>` — same semantics |
| `review` | Direct variant constructor as a function | Closure `\|r\| Shape::Circle(r)` |
| `over` for non-match | Returns original `s` unchanged | Returns `s.clone()` — requires `Clone` |
| Generic prisms | Implicit polymorphism | Requires explicit `Clone + 'static` bounds |
