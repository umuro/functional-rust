## Core Insight

The Iterator trait is Rust's most powerful abstraction. Implement `next() -> Option<Item>` and get 70+ methods for free. OCaml uses `Seq` (lazy sequences) for similar functionality.

## OCaml Approach
- `Seq.t` type: `unit -> Seq.node` where `node = Nil | Cons of 'a * 'a Seq.t`
- Lazy by construction
- Manual implementation via closures

## Rust Approach
- `trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }`
- All adapter methods provided by default
- Lazy — nothing computed until consumed

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Core | `unit -> node` | `fn next() -> Option<Item>` |
| Lazy | Yes (thunk) | Yes (pull-based) |
| Free methods | Few | 70+ (map, filter, fold...) |
| State | Closure captures | `&mut self` |
