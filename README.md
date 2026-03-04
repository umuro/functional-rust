# Functional Rust

854 OCaml → Rust functional programming examples, from basic list operations to category theory.

🌐 **Live site:** https://hightechmind.io/rust/

## Structure

Each example in `examples/NNN-name/` contains:
- `README.md` — explanation, difficulty, learning outcomes
- `example.ml` — OCaml implementation
- `example.rs` — Rust implementation with `fn main()` and `#[test]` blocks
- `COMPARISON.md` — ⚙️ Design Notes: side-by-side analysis, trade-offs, performance notes

## Levels

| Level | Name | Focus |
|-------|------|-------|
| 1 | Foundations | Lists, recursion, pattern matching, Option/Result |
| 2 | Intermediate | Monads, traits, iterators, error handling |
| 3 | Advanced | Free monads, effects, optics, recursion schemes |
| 4 | Expert | Category theory, Kan extensions, profunctors |
| 5 | Master | Tagless final, effect systems, proof-carrying types |

## Site Generation

```bash
cd ~/.openclaw/workspace/skills/typescript-builder/functional-rust-site
npx tsx src/generate-site.ts
```

## Deployment

```bash
bash ~/.openclaw/workspace/skills/functional-rust/scripts/deploy.sh
```
