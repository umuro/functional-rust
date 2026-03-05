# 617: Prism Pattern — Fallible Access for Enum Variants

**Difficulty:** 5  **Level:** Master

A Prism is the enum counterpart to a Lens. Where a Lens always succeeds (every struct has the field), a Prism may fail (not every enum is the right variant).

## The Problem This Solves

You have an enum — say, a JSON type with six variants: `Null`, `Bool`, `Num`, `Str`, `Arr`, `Obj`. You want to write generic code that processes "the boolean value, if any" from a list of JSON values. Without Prisms, you write `match` at every call site:

```rust
let bools: Vec<bool> = jsons.iter().filter_map(|j| {
    match j {
        Json::Bool(b) => Some(*b),
        _ => None,
    }
}).collect();
```

That's fine. But now imagine the same pattern for every variant, in every function, across a codebase with ten enum types. The `match` arms are repeated, the "extract if this variant" logic is scattered, and adding a new variant forces you to review every `match`.

A Prism makes variant access first-class. You define the extraction logic once (in the Prism's `preview` function) and the construction logic once (in `review`). Then pass the Prism as a value — to filtering functions, mapping functions, or any generic code that works with "some variant of some type." The Prism is your typed, named, reusable accessor for that one variant. This example exists to solve exactly that pain.

## The Intuition

A Lens has two operations: `get` (always returns `A`) and `set` (always succeeds).

A Prism has two operations:
- **`preview`**: `(&S) -> Option<A>` — "if this is the right variant, give me the payload"
- **`review`**: `(A) -> S` — "construct this variant from a payload" — always succeeds

`preview` is like a type-safe checked downcast. Given a `Json`, is it a `Bool`? If yes, give me the `bool`. If not, give me `None`.

`review` is the variant constructor. Given a `bool`, make a `Json::Bool(b)`.

The two operations are inverses when the variant matches:
- `preview(review(a)) == Some(a)` — build and then inspect gives back what you started with
- If `preview(s) == Some(a)`, then `review(a) == s` — inspect and then build gives back the original

These are the Prism laws (see example 207). They ensure the Prism makes sense — that `preview` and `review` are talking about the same variant in the same way.

**`over` — conditional transformation:**

Unlike a Lens (where `over` always applies), a Prism's `over` only transforms when `preview` succeeds:

```rust
fn over(&self, f: impl Fn(A) -> A, s: S) -> S {
    match self.preview(&s) {
        Some(a) => self.review(f(a)),  // matching variant: transform it
        None    => s,                  // wrong variant: return unchanged
    }
}
```

## How It Works in Rust

**Define Prisms for your enum:**

```rust
let bool_prism: Prism<Json, bool> = Prism::new(
    |j| match j { Json::Bool(b) => Some(*b), _ => None },
    |b| Json::Bool(b),
);

let num_prism: Prism<Json, f64> = Prism::new(
    |j| match j { Json::Num(n) => Some(*n), _ => None },
    |n| Json::Num(n),
);
```

**Verify the laws built into the Prism:**

```rust
// Law 1: preview(review(a)) == Some(a)
bool_prism.law_preview_review(true)   // true — it's lawful

// Law 2: if preview(s) == Some(a) then review(a) == s
bool_prism.law_review_preview(&Json::Num(1.0))  // true — Num doesn't match, vacuously true
```

**`over` — transform only matching variants:**

```rust
let j1 = Json::Bool(false);
let j2 = bool_prism.over(|b| !b, j1);   // Json::Bool(true)

let j3 = Json::Null;
let j4 = bool_prism.over(|b| !b, j3);   // Json::Null — unchanged, Null isn't a Bool
```

**Filter a collection using `preview`:**

```rust
let jsons = vec![Json::Bool(true), Json::Num(3.14), Json::Str("hi".into()), Json::Null];

let bools: Vec<bool> = jsons.iter()
    .filter_map(|j| bool_prism.preview(j))
    .collect();
// [true] — only the Bool variant matched
```

The Prism replaces the inline `match` with a named, reusable accessor. Pass `bool_prism` as a value to any function that needs to extract booleans from JSON.

## What This Unlocks

- **Named, reusable variant accessors**: define once, use in `filter_map`, `map`, generic functions — no repeated `match` arms at call sites.
- **Algebraic guarantees**: the two Prism laws (built as methods here) ensure `preview` and `review` are consistent — your Prism isn't just two unrelated functions wearing a struct.
- **Composable with Lenses**: a Lens into a struct field that is itself an enum, followed by a Prism into one of its variants, gives you a Traversal — a single accessor for "the payload of this variant in this field, if it exists."

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `preview` signature | `'s -> 'a option` | `fn(&S) -> Option<A>` — identical concept |
| `review` signature | `'a -> 's` | `fn(A) -> S` — identical concept |
| Law: ReviewPreview | `preview(review(a)) = Some(a)` | `self.preview(&self.review(a.clone())) == Some(a)` |
| Law: PreviewReview | If `preview(s)=Some(a)` then `review(a)=s` | Same — `match self.preview(s)` with clone |
| `over` for non-match | Returns `s` unchanged | Returns `s` unchanged — requires `S: Clone` if by reference |
| Composition | `prism1.compose(prism2)` | Manual — chain `preview` calls with `and_then` |
