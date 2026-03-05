# OCaml vs Rust: Lens Modify — Transform a Field With a Function

## Side-by-Side Code

### OCaml
```ocaml
type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

let modify (l : ('s, 'a) lens) (f : 'a -> 'a) (s : 's) : 's =
  l.set (f (l.get s)) s

type counter = { count : int; label : string }

let count_lens = {
  get = (fun c -> c.count);
  set = (fun n c -> { c with count = n });
}

let increment = modify count_lens (( + ) 1)
let double    = modify count_lens (( * ) 2)
let reset     = modify count_lens (fun _ -> 0)
```

### Rust (idiomatic — method on struct)
```rust
impl<S: 'static, A: 'static> Lens<S, A> {
    pub fn modify(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        (self.set)(f((self.get)(s)), s)
    }
}

let lens = counter_count_lens();
let incremented = lens.modify(|n| n + 1, &counter);
let doubled     = lens.modify(|n| n * 2, &counter);
let reset       = lens.modify(|_| 0,     &counter);
```

### Rust (composed — modify through two levels)
```rust
// cart_item_lens: Lens<Cart, Item>
// item_price_lens: Lens<Item, f64>
// composed:       Lens<Cart, f64>
let cart_price = cart_item_lens().compose(item_price_lens());
let discounted = cart_price.modify(|p| p * 0.9, &cart);
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Lens type | `('s, 'a) lens` | `Lens<S, A>` |
| `modify` signature | `('s,'a) lens -> ('a -> 'a) -> 's -> 's` | `fn modify(&self, f: impl FnOnce(A)->A, s: &S) -> S` |
| Transformation fn | `'a -> 'a` (auto-curried) | `impl FnOnce(A) -> A` (explicit trait) |
| Partial application | `let increment = modify count_lens ((+) 1)` | requires a closure or helper fn |
| Struct update | `{ c with count = n }` | `Counter { count: n, ..c.clone() }` |

## Key Insights

1. **`modify` = `set ∘ f ∘ get`**: Both languages implement the same three-step pipeline — fetch, transform, replace. The implementations are structurally identical; only the syntax differs.

2. **Partial application gap**: OCaml's currying makes `let increment = modify count_lens ((+) 1)` natural — it returns a function `counter -> counter`. Rust requires an explicit closure or a helper struct to achieve the same partially-applied combinator, because Rust functions are not auto-curried.

3. **Ownership and immutability**: OCaml's record update syntax (`{ c with count = n }`) performs an implicit shallow copy. Rust spells this out with `..c.clone()`, making the allocation visible. `modify` takes `&S` (borrow) and returns a new owned `S`, matching OCaml's persistent/immutable data style.

4. **`FnOnce` vs `Fn`**: The transformation `f` is consumed once per call (`FnOnce`), which is the most general Rust closure bound. If you need to call `modify` repeatedly with the same closure stored somewhere, you'd use `Fn` instead — a trade-off OCaml never surfaces because closures are always copyable values there.

5. **Composition pays off at `modify` time**: A composed `Lens<Cart, f64>` lets you call `modify` on the nested price field with exactly the same API as a flat lens. No extra boilerplate, no repeated navigation code — the composition glues the path together once and `modify` works uniformly at any depth.

## When to Use Each Style

**Use `modify` when:** you need to transform a field rather than replace it with a literal — incrementing counters, scaling prices, uppercasing strings, appending to lists. Any time the new value depends on the old one, `modify` is cleaner than `set(lens, f(get(lens, s)), s)`.

**Use composed `modify` when:** the field to transform is nested two or more levels deep. Compose the path lenses once, then call `modify` on the result — the navigation logic lives in one place and the transformation logic lives at the call site.
