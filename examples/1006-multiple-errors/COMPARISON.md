# Multiple Error Types — Comparison

## Core Insight
When functions call code with different error types, you need a unification strategy. Rust offers two: type-erased (`Box<dyn Error>`) and typed (enum with `From` impls).

## OCaml Approach
- Unified variant types manually wrap each sub-error
- Polymorphic variants auto-unify but lose exhaustiveness guarantees
- No standard trait object equivalent to `Box<dyn Error>`

## Rust Approach
- `Box<dyn Error>`: any error type auto-converts, but you lose pattern matching
- Typed enum + `From` impls: more boilerplate, full pattern matching retained
- The `?` operator works with both approaches

## Comparison Table

| Aspect | OCaml Variant | OCaml Poly Variant | Rust `Box<dyn>` | Rust Enum |
|--------|--------------|-------------------|-----------------|-----------|
| Setup cost | Medium | Low | Low | Medium |
| Pattern matching | Yes | Partial | No (need downcast) | Yes, exhaustive |
| Extensibility | Closed | Open | Open | Closed |
| Performance | Zero-cost | Zero-cost | Heap allocation | Zero-cost |
| Best for | Libraries | Prototyping | Scripts/prototypes | Libraries/apps |
