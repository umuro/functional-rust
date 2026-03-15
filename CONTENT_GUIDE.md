# Content Guide — Functional Rust

This document describes the file conventions each example directory must follow so the site generator picks them up correctly.

## Example Directory Layout

```
examples/001-higher-order-functions/
  Cargo.toml              ← required (marks this as a real example)
  README.md               ← required (missing = example is skipped entirely)
  src/lib.rs              ← Rust source (primary code)
  example.ml              ← OCaml source (optional; fallback: OCaml blocks in COMPARISON.md)
  COMPARISON.md           ← side-by-side comparison with OCaml and Rust code blocks
  video.mp4               ← Remotion-generated tutorial video (optional; triggers video section)
  video-description.txt   ← accessibility text for the video (optional; shown as collapsible)
```

## README.md Required Fields

The site generator extracts metadata from structured fields in README.md:

```markdown
# Example 001: Higher-Order Functions

**Difficulty:** Fundamental

**Category:** Functional Programming

## Problem Statement
...

## Learning Outcomes
...

## Rust Approach
...

## OCaml Approach
...

## Key Differences
...
```

- `**Difficulty:**` — one of `Fundamental`, `Intermediate`, `Advanced`, `Expert`
- `# Example NNN: Title` — used for the page title and deduplication (examples with identical titles are skipped)

## OCaml Code

Every example should have OCaml code. The site falls back in this order:

1. `example.ml` — preferred; shown as the OCaml source file
2. First ` ```ocaml ` block in `COMPARISON.md` — used when `example.ml` is absent
3. "(no OCaml source for this example)" placeholder — shown when neither exists

**312 examples currently lack OCaml code.** These show the placeholder text.

## Remotion Tutorial Videos

When a Remotion-generated `video.mp4` is committed to the example directory, the site automatically shows:

- An HTML5 `<video controls>` player (pause, rewind, seek built-in)
- A collapsible "Text description (accessibility)" section if `video-description.txt` exists

No site regeneration is needed when videos are added — they appear automatically on the next build.

### Video file naming

| File | Purpose |
|------|---------|
| `video.mp4` | Tutorial video rendered by Remotion |
| `video-description.txt` | Accessibility transcript / description (plain text) |

### Example workflow

```bash
# Generate video with Remotion (run from skills/remotion or similar)
npx remotion render --props '{"exampleId":"001-higher-order-functions"}' out/video.mp4

# Copy output to example directory
cp out/video.mp4 examples/001-higher-order-functions/video.mp4

# Add accessibility text
cat > examples/001-higher-order-functions/video-description.txt <<'EOF'
This video demonstrates higher-order functions in Rust. It shows how map, filter, and fold
work on iterators, with a side-by-side comparison to the equivalent OCaml code.
EOF

# Commit both files
git add examples/001-higher-order-functions/
git commit -m "add Remotion video and accessibility text for example 001"
```

## Skipped Examples

The site generator skips an example directory if:

1. **No README.md** — no metadata, no content to display
2. **Duplicate title** — same title as an earlier example (keep the lower-numbered one)

Placeholder directories (named like `{1115..1118}-placeholder`) are automatically skipped.

## Generating the Site

```bash
python3 ~/.openclaw/workspace/scripts/generate-rust-site.py
```

Output: `/tmp/rust-site/rust/`

Preview:
```bash
python3 -m http.server 8080 --directory /tmp/rust-site/rust
open http://localhost:8080/index.html
```
