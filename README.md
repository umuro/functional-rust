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

## Site Generation & Deployment

All scripts are **inside this repo** — nothing lives outside it.

```bash
# Generate + deploy HTML (no videos)
cd /home/node/.openclaw/workspace/functional-rust
bash site/deploy-rust-site.sh --no-videos

# Generate HTML only
python3 site/generate-rust-site.py

# Render one video
node video/render.js 001

# Render all videos in parallel
bash video/batch-render.sh
```

⚠️ **Wrong paths (stale, do not use):**
- `skills/typescript-builder/functional-rust-site/src/generate-site.ts` — does not exist
- `skills/functional-rust/scripts/deploy.sh` — stale, replaced by `site/deploy-rust-site.sh`
- `workspace/examples/` — empty, examples are at `functional-rust/examples/`
