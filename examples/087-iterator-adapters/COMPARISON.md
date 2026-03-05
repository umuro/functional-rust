## Core Insight

An adapter takes an iterator and returns a new iterator that transforms each element. They compose lazily — no intermediate collections.

## OCaml Approach
- `Seq.map`, `Seq.filter` — return new Seq
- Can build custom: `let my_map f s () = match s () with ...`

## Rust Approach
- Built-in: `.map()`, `.filter()`, `.take()`, `.skip()`
- Custom: struct wrapping `I: Iterator` + impl Iterator

## Comparison Table

| Adapter | OCaml | Rust |
|---------|-------|------|
| Map | `Seq.map f s` | `.map(f)` |
| Filter | `Seq.filter p s` | `.filter(p)` |
| Take | manual | `.take(n)` |
| Skip | manual | `.skip(n)` |
| Custom | Closure-based | Struct wrapping iterator |
