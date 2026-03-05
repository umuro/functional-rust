# Clone-on-Write Collections — Comparison

## Core Insight
Clone-on-write defers allocation until mutation occurs. In Rust, this is explicit via `Cow<'a, T>`. In OCaml, immutable data structures with structural sharing provide this behavior implicitly — you never clone because you never mutate.

## OCaml Approach
- Immutable data = natural CoW via structural sharing
- `0 :: list` shares the tail — zero-copy extension
- `List.map` creates new structure only where changed
- No explicit Cow type needed
- GC handles shared references transparently
- Explicit CoW possible with `ref` + `Array.copy` but rarely needed

## Rust Approach
- `Cow<'a, [T]>`: enum of `Borrowed(&'a [T])` and `Owned(Vec<T>)`
- `Cow<'a, str>`: `Borrowed(&'a str)` or `Owned(String)`
- `to_mut()`: clones on first mutation, returns `&mut`
- `into_owned()`: consumes Cow, returns owned value
- Functions can return `Cow` to avoid cloning on happy path
- Deref coercion makes reading transparent

## Comparison Table

| Feature | OCaml | Rust (`Cow`) |
|---|---|---|
| CoW mechanism | Implicit (immutability) | Explicit (`Cow<'a, T>`) |
| Read cost | Zero (shared) | Zero (Deref) |
| Write cost | New allocation (structural sharing) | Clone on first write |
| API complexity | None (just use values) | `Cow::Borrowed`/`Owned`, `to_mut()` |
| Return from function | Value (shared by GC) | `Cow` (avoids clone if unmodified) |
| Typical use | Default behavior | Optimization for read-heavy paths |
