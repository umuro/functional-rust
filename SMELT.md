# Smelt — The functional-rust Content Pipeline

> Smelt: to extract metal from ore by heating. Here: OCaml source → idiomatic Rust.

## The Four Steps

```
Hunt → Queue → Convert → Publish
```

### 1. Hunt 🔍
Collect OCaml examples from multiple sources:
- OCaml 99 Problems (examples 001–056 — current base)
- Real World OCaml (Minsky, Madhavapeddy, Hickey)
- Cornell CS3110 — Functional Programming in OCaml course
- OCaml.org official tutorials
- Exercism OCaml track
- Functional Programming in OCaml MOOC (France Université Numérique)
- OCaml Standard Library patterns (List, Map, Set, Hashtbl, Option, Result)
- Jane Street Core library patterns
- Lwt / Async concurrency patterns
- ppx metaprogramming examples

Hunt results go into **QUEUE.md**.

### 2. Queue 📋
`QUEUE.md` is the ore queue — OCaml snippets waiting to be converted.

Each entry has:
- Source URL or book/chapter reference
- Topic description
- OCaml code
- Status: `[ ]` pending / `[x]` done

### 3. Convert ⚒️
Claude Code (dev-tools Docker) takes each queued item:
1. Writes idiomatic Rust in `examples/NNN-name/example.rs`
2. Writes OCaml version in `examples/NNN-name/example.ml`
3. Runs `cargo test` and `ocaml` to verify both compile and pass
4. Writes `COMPARISON.md` — the key insight explaining the functional parallel
5. Updates `README.md` for the example

Quality gate: both files must compile and all tests pass. No stubs allowed through.

### 4. Publish 🌐
Generator script reads `examples/` → produces HTML → deploys to `hightechmind.io/rust/`

```bash
python3 ~/.openclaw/workspace/scripts/generate-rust-site.py
scp -P 65002 -i ~/.ssh/id_ed25519 /tmp/rust-site/rust/*.html \
    u508071997@185.224.137.204:~/domains/hightechmind.io/public_html/rust/
```

## File Structure

```
functional-rust/
├── SMELT.md        ← this file (pipeline docs)
├── QUEUE.md        ← ore queue (OCaml snippets pending conversion)
├── CLAUDE.md       ← instructions for Claude Code workers
├── Cargo.toml      ← workspace root
└── examples/
    └── NNN-name/
        ├── README.md       ← what this example teaches
        ├── example.rs      ← idiomatic Rust (verified)
        ├── example.ml      ← OCaml original (verified)
        └── COMPARISON.md   ← the functional parallel insight
```

## Stub Detection

`COMPARISON.md` missing OR < 500 bytes = stub. Stubs are skipped at publish time
(they generate a placeholder HTML page instead of full content).

## Cron Schedule

Three Smelt cycles per day (Amsterdam time):
- 02:00 — Batch A (5 examples)
- 12:00 — Batch B (5 examples)
- 18:00 — Batch C (5 examples)

After all stubs cleared: reduce to 1 cycle/day for steady growth beyond example 056.
