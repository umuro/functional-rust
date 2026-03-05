# OCaml vs Rust: Prism Basics — Optics for Enum Variants

## Side-by-Side Code

### OCaml

```ocaml
type ('s, 'a) prism = {
  preview : 's -> 'a option;
  review  : 'a -> 's;
}

type shape =
  | Circle of float
  | Rectangle of float * float
  | Triangle of float * float * float

let circle_prism : (shape, float) prism = {
  preview = (function Circle r -> Some r | _ -> None);
  review  = (fun r -> Circle r);
}

(* Modify a shape through the prism *)
let over prism f s =
  match prism.preview s with
  | Some a -> prism.review (f a)
  | None   -> s
```

### Rust (closure-based)

```rust
pub struct Prism<S, A> {
    preview: Box<dyn Fn(&S) -> Option<A>>,
    review:  Box<dyn Fn(A) -> S>,
}

pub fn circle_prism() -> Prism<Shape, f64> {
    Prism::new(
        |s| match s { Shape::Circle(r) => Some(*r), _ => None },
        Shape::Circle,
    )
}

// over: apply f to payload when the variant matches
pub fn over(&self, s: &S, f: impl FnOnce(A) -> A) -> S where S: Clone {
    match self.preview(s) {
        Some(a) => self.review(f(a)),
        None    => s.clone(),
    }
}
```

### Rust (trait-based, zero allocation)

```rust
pub trait PrismTrait {
    type Source: Clone;
    type Focus;

    fn preview(s: &Self::Source) -> Option<Self::Focus>;
    fn review(a: Self::Focus) -> Self::Source;

    fn over(s: &Self::Source, f: impl FnOnce(Self::Focus) -> Self::Focus) -> Self::Source {
        match Self::preview(s) {
            Some(a) => Self::review(f(a)),
            None    => s.clone(),
        }
    }
}

pub struct CirclePrism;

impl PrismTrait for CirclePrism {
    type Source = Shape;
    type Focus  = f64;
    fn preview(s: &Shape) -> Option<f64>  { match s { Shape::Circle(r) => Some(*r), _ => None } }
    fn review(r: f64) -> Shape             { Shape::Circle(r) }
}
```

## Type Signatures

| Concept              | OCaml                                    | Rust                                         |
|----------------------|------------------------------------------|----------------------------------------------|
| Prism type           | `('s, 'a) prism`                         | `Prism<S, A>` / `impl PrismTrait`           |
| preview              | `'s -> 'a option`                        | `fn(&S) -> Option<A>`                        |
| review               | `'a -> 's`                               | `fn(A) -> S`                                 |
| over                 | `prism -> ('a -> 'a) -> 's -> 's`        | `fn over(&self, s: &S, f: FnOnce(A)->A)->S` |
| Partial focus        | structural `option`                      | `Option<A>` with explicit `clone` on miss   |
| Dispatch             | record of closures (runtime)             | boxed closures *or* monomorphised trait impls|

## Key Insights

1. **Same algebra, different shapes.** OCaml records `{ preview; review }` map directly onto Rust structs with boxed `Fn` fields. The logic — "unwrap, transform, re-wrap or fall through" — is identical in both languages; only the syntax differs.

2. **Ownership forces an explicit clone on miss.** In OCaml `over` can return the original `s` unchanged because values are immutable and GC-managed. In Rust, returning the untouched value requires `S: Clone` (or working with owned values), making the cost of the "miss" path visible at the type level.

3. **Two Rust encoding strategies trade heap for flexibility.** The boxed-closure `Prism<S, A>` is a runtime object: prisms can be stored in `Vec`s, passed as arguments, and composed generically — at the cost of one heap allocation and dynamic dispatch. The `PrismTrait` encoding is monomorphised at compile time (zero allocation, inlined calls) but cannot be stored in a heterogeneous collection without type-erasing it first.

4. **`review` can be a constructor.** Both OCaml and Rust let you pass a variant constructor directly as the `review` function (`Shape::Circle` is already a function `f64 -> Shape` in both languages), keeping the prism definition minimal.

5. **Prisms compose with lenses.** A Lens that always succeeds (`get`, `set`) and a Prism that might fail (`preview`, `review`) are dual: their composition yields an *optional* — a Prism whose focus is itself lensed. This layering of optics is exactly how libraries like `lens` (Haskell), `optics` (Haskell/OCaml) and Rust's `lens-rs` are structured, showing that the tiny `Prism` abstraction here is the foundational building block of a full optics hierarchy.

## When to Use Each Style

**Use closure-based `Prism<S,A>` when:** you need to store prisms in data structures, build prism combinators at runtime, or compose heterogeneous prisms — flexibility matters more than allocation cost.

**Use trait-based `PrismTrait` when:** you are writing hot-path code where you want the compiler to inline the preview/review logic entirely, or when you want to express prisms as zero-sized types that carry no data of their own.
