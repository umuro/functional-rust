# Persistent List — Comparison

## Core Insight
A persistent data structure preserves old versions when modified. OCaml lists are inherently persistent: `x :: list` creates a new cons cell pointing to the existing list — zero copying, GC manages memory. Rust requires `Rc<T>` (reference-counted shared ownership) to allow multiple bindings to own the same tail, since Rust's ownership model forbids multiple owners without `Rc`/`Arc`.

## OCaml Approach
- `type 'a pstack = Empty | Cons of 'a * 'a pstack` — recursive type
- `0 :: list` — O(1) cons, GC handles sharing automatically
- Old versions remain accessible because GC won't collect shared nodes
- `let list2 = x :: list1` creates new head, shares list1's storage
- Pattern matching for pop: `Cons (x, rest) -> Some (x, rest)`
- Built-in `list` type IS a persistent list — no extra work

## Rust Approach
- `enum PList<T> { Nil, Cons(T, Rc<PList<T>>) }` — needs `Rc` for sharing
- `Rc::new(PList::Cons(x, Rc::clone(tail)))` — clone the Rc (increments count, O(1))
- `Rc::clone` does NOT copy the list node — it copies the pointer + bumps ref count
- When last `Rc` pointing to a node is dropped, the node is freed
- Thread safety: use `Arc<T>` instead of `Rc<T>` for shared across threads
- Pattern matching same as OCaml, but requires `list.as_ref()` to match

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Sharing mechanism | GC (automatic) | `Rc<T>` (reference counting) |
| Cons operation | `x :: list` | `Rc::new(Cons(x, Rc::clone(tail)))` |
| Old versions | Kept alive by GC | Kept alive by Rc count > 0 |
| Memory reclaim | GC cycle | Drop when last Rc gone |
| Deref to match | `match list with Cons ...` | `match list.as_ref() { Cons ...` |
| Thread safety | GC handles | Need `Arc<T>` instead of `Rc<T>` |
| Clone pointer | n/a (GC) | `Rc::clone()` — O(1) pointer copy |
