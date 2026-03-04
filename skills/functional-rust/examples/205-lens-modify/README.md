# 205: Lens Modify — Transform a Field With a Function

**Difficulty:** ⭐⭐⭐  **Level:** Intermediate

`modify` applies a transformation function through a Lens — the most practical Lens operation in real code.

## The Problem This Solves

`set` replaces a field with a literal value. That covers some cases, but real code usually transforms: "increment the counter", "double the price", "append to the list", "uppercase the label". For each transformation you'd write a separate function — `increment_count`, `double_price`, `append_tag` — each one repeating the same struct-update boilerplate.

You need a way to say: "apply this function to this field and give me back a new struct." That's `modify`. Without it, you'd write `set(lens, f(view(lens, s)), s)` everywhere — mixing the "how to navigate" concern with the "what to do" concern at every call site.

`modify` separates these concerns cleanly: the Lens knows the path, the function knows the transformation. You can pass any function; the Lens handles the structural surgery. Pipelines become a sequence of `modify` calls, each readable and independent. This example exists to solve exactly that pain.

## The Intuition

`modify` is defined in terms of `get` and `set`:

```
modify lens f s  =  set lens (f (get lens s)) s
```

In English: "look at the value, run it through `f`, put the result back." The struct's other fields are untouched.

But it's easier to think of `modify` as the **primary** operation and `get`/`set` as special cases:
- `get lens s` = `modify lens (save the value) s` (the Identity functor trick, if you want the theory)
- `set lens a s` = `modify lens (|_| a) s` — replace with a constant function

In practice, you call `modify` or `over` (same thing, different name in different libraries) far more often than raw `get`/`set`.

## How It Works in Rust

**Adding `modify` to the Lens:**

```rust
impl<S: 'static, A: 'static> Lens<S, A> {
    fn modify(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        // get the current value, transform it, set it back
        (self.set)(f((self.get)(s)), s)
    }
}
```

**Basic usage:**

```rust
let c = Counter { count: 5, label: "clicks".into() };
let l = count_lens();

l.modify(|n| n + 1,  &c).count  // 6
l.modify(|n| n * 2,  &c).count  // 10
l.modify(|_| 0,      &c).count  // 0  (constant function = set)
```

**Modify through a composed Lens:**

```rust
let o = Outer { inner: Inner { value: 10 }, tag: "test".into() };
let deep = inner_lens().compose(value_lens());

// Transform the deeply nested value — one line
let o2 = deep.modify(|v| v + 1, &o);
assert_eq!(o2.inner.value, 11);
assert_eq!(o2.tag, "test");  // other fields untouched
```

**Pipeline of modifications:**

When you need several sequential transformations, chain `modify` calls:

```rust
fn pipeline(c: &Counter) -> Counter {
    let c2 = count_lens().modify(|n| n + 10, c);   // step 1: add 10
    count_lens().modify(|n| n * 2, &c2)              // step 2: double
}

let result = pipeline(&Counter { count: 5, label: "x".into() });
// (5 + 10) * 2 = 30
```

Each step is its own line. Each step is independently readable. There's no nested expression syntax to parse.

## What This Unlocks

- **One operation for all transformations**: `modify(lens, f)` replaces the entire family of `transform_field_*` helper functions — just pass a different `f`.
- **Composable pipelines**: sequence multiple `modify` calls to build multi-step update pipelines where each step is independently clear.
- **Deep updates, simple functions**: compose Lenses for depth, use `modify` for the transformation — keep path logic and business logic completely separate.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Function name | `modify` or `over`, infix `%~` operator | `modify` or `over` method on `Lens<S,A>` |
| Infix pipeline | `s \|> (count_l %~ ((+) 1)) \|> (count_l %~ ( *2))` | Sequential `let` bindings — no infix |
| Partial application | `modify count_l ((+) 1)` returns `'s -> 's` | Closures required: `\|c\| l.modify(\|n\| n+1, c)` |
| Currying | Natural — `modify lens` is a function | Not idiomatic — wrap in a closure |
| Pipelines | `\|>` operator chains transformations cleanly | `let c2 = …; let c3 = …;` sequential bindings |
