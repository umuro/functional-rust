# CLAUDE.md — Functional Smelt: OCaml → Rust Conversion

> You are a Rust expert extracting idiomatic Rust from OCaml ore.
> Every example you produce must compile, pass clippy, pass tests — no exceptions.

---

## Project Layout

```
functional-rust/
├── Cargo.toml          ← workspace root (auto-discovers examples/[0-9]*/Cargo.toml)
├── QUEUE.md            ← OCaml snippets waiting for conversion
└── examples/
    └── NNN-name/       ← one crate per example
        ├── Cargo.toml
        ├── src/
        │   └── lib.rs  ← Rust implementations + #[cfg(test)]
        ├── example.rs  ← standalone Rust (copy of lib.rs public API — for HTML display)
        ├── example.ml  ← OCaml original (idiomatic, verified)
        ├── README.md   ← what this example teaches
        └── COMPARISON.md ← the functional parallel insight (must be > 500 bytes)
```

---

## Per-Example Cargo.toml

```toml
[package]
name = "example-NNN-name"
version = "0.1.0"
edition = "2021"
```

No external dependencies unless the problem specifically requires one.
Use only `std`. If you need randomness: `use std::collections::HashMap` etc.

---

## Rust Quality Gates (mandatory, in order)

Run these after writing `src/lib.rs`. Fix every issue before moving on.

```bash
# 1. Format
cargo fmt -p example-NNN-name

# 2. Clippy — ALL warnings are errors, no exceptions
cargo clippy -p example-NNN-name -- -D warnings

# 3. Tests — ALL must pass
cargo test -p example-NNN-name

# 4. Confirm clean (run again after fixes)
cargo clippy -p example-NNN-name -- -D warnings && cargo test -p example-NNN-name
```

**If clippy or tests fail: fix the code, not the quality gates.**
Do not add `#[allow(clippy::...)]` to silence warnings — fix the root cause.

---

## Rust Code Standards

### Idiomatic first, recursive second

Write 2-3 implementations per example:

```rust
// Solution 1: Idiomatic Rust — how a Rust developer writes it
// Uses std library, iterators, slices
fn last_idiomatic<T>(list: &[T]) -> Option<&T> {
    list.last()
}

// Solution 2: Functional/recursive — closer to OCaml style
// Shows the OCaml parallel explicitly
fn last_recursive<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,
        [x] => Some(x),
        [_, rest @ ..] => last_recursive(rest),
    }
}

// Solution 3: Iterator chain (when meaningfully different)
fn last_iter<T>(list: &[T]) -> Option<&T> {
    list.iter().last()
}
```

### Ownership and borrowing comments

When ownership decisions are non-obvious, explain them inline:

```rust
// Takes &[T] not Vec<T> — borrows the slice, no allocation needed
fn process<T>(items: &[T]) -> usize {
    items.len()
}
```

### Tests — minimum 4, covering:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() { /* empty input */ }

    #[test]
    fn test_single() { /* single element */ }

    #[test]
    fn test_multiple() { /* typical case with several elements */ }

    #[test]
    fn test_edge_case() { /* boundary condition specific to the problem */ }
}
```

Tests must use `assert_eq!` or `assert!`. No `println!` in tests.
All test functions must have a clear name describing what they test.

### No stubs

Never produce:
- Functions that just `todo!()` or `unimplemented!()`
- Tests that just `assert!(true)`
- Empty `impl` blocks
- Placeholder comments like `// TODO: implement`

---

## example.rs (for HTML display)

After `src/lib.rs` passes all quality gates, create `example.rs`:
- Copy the public functions and the `#[cfg(test)]` block
- Add a `main()` function that demonstrates the usage with `println!`
- Include a comment block at the bottom showing the expected output:

```rust
fn main() {
    println!("last([1,2,3,4]) = {:?}", last(&[1, 2, 3, 4]));
    println!("last([]) = {:?}", last::<i32>(&[]));
}

/* Output:
   last([1,2,3,4]) = Some(4)
   last([]) = None
*/
```

---

## example.ml (OCaml original)

Write clean, idiomatic OCaml with:
- 1-2 implementations (at least one idiomatic, one recursive if different)
- Inline test assertions using `assert (expr = expected_value)`
- A `let () = print_endline "ok"` at the end to confirm execution

```ocaml
(* Idiomatic OCaml *)
let last lst = List.nth_opt lst (List.length lst - 1)

(* Recursive OCaml — shows the explicit recursion *)
let rec last_rec = function
  | [] -> None
  | [x] -> Some x
  | _ :: rest -> last_rec rest

let () =
  assert (last [1;2;3;4] = Some 4);
  assert (last [] = None);
  print_endline "ok"
```

---

## README.md

```markdown
# Example NNN: [Title]

**Difficulty:** ⭐ / ⭐⭐ / ⭐⭐⭐  
**Category:** [Lists & HOF | Pattern Matching | Error Handling | Trees | etc.]  
**OCaml Source:** [source reference — book/chapter or URL]

## Problem Statement

[1-2 sentences: what does this example compute?]

## Learning Outcomes

- [What Rust concept this teaches]
- [What ownership pattern appears]
- [What OCaml→Rust translation insight this shows]
- [One more concrete takeaway]

## OCaml Approach

[2-3 sentences: how OCaml solves it, what makes it idiomatic OCaml]

## Rust Approach

[2-3 sentences: how Rust solves it, what makes it idiomatic Rust]

## Key Differences

1. **[Concept]:** [OCaml does X, Rust does Y]
2. **[Concept]:** [OCaml does X, Rust does Y]
3. **[Concept]:** [OCaml does X, Rust does Y]
4. **[Concept]:** [OCaml does X, Rust does Y]
```

---

## COMPARISON.md (minimum 500 bytes — required for site publish)

Must include all sections below. Short = stub = excluded from website.

```markdown
# OCaml vs Rust: [Title]

## Side-by-Side Code

### OCaml
```ocaml
[idiomatic OCaml solution]
```

### Rust (idiomatic)
```rust
[idiomatic Rust solution]
```

### Rust (functional/recursive)
```rust
[recursive Rust solution]
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val last : 'a list -> 'a option` | `fn last<T>(list: &[T]) -> Option<&T>` |
| List type | `'a list` | `&[T]` (slice) |
| Optional value | `'a option` | `Option<T>` |

## Key Insights

1. [Most important translation insight]
2. [Ownership/borrowing insight]
3. [Type system difference]
4. [Idiom difference]
5. [Performance or safety difference]

## When to Use Each Style

**Use idiomatic Rust when:** [concrete scenario]  
**Use recursive Rust when:** [concrete scenario]
```

---

## Self-Verification (run before reporting done)

```bash
# All examples in this batch must pass:
for dir in examples/NNN-*; do
  echo "=== $dir ==="
  cargo clippy -p "example-$(basename $dir)" -- -D warnings 2>&1 | tail -3
  cargo test -p "example-$(basename $dir)" 2>&1 | tail -3
done
```

Checklist before reporting done:
- [ ] `cargo fmt` ran — code is formatted
- [ ] `cargo clippy -D warnings` → zero warnings
- [ ] `cargo test` → all tests pass
- [ ] `example.rs` exists and has `main()` with output comment
- [ ] `example.ml` exists and has `assert` tests + `print_endline "ok"`
- [ ] `README.md` exists with all sections filled
- [ ] `COMPARISON.md` exists and is > 500 bytes
- [ ] No stubs (`todo!()`, `unimplemented!()`, `assert!(true)`)

---

## Reporting Format

When done with a batch, report:

```
DONE — Batch [A/B/C]
Converted: [list of example names]
cargo fmt ✓ | clippy -D warnings ✓ | cargo test ✓

001-last-element    — 4 tests ✓ | clippy clean ✓
002-last-two        — 4 tests ✓ | clippy clean ✓
...

Queue remaining: [N] items
```

---

## Known Failure Patterns (avoid these)

| Pattern | What breaks | Fix |
|---------|------------|-----|
| `impl<T: Clone>` on slice operations | Clippy warns: unnecessary Clone bound | Use `&[T]` without Clone unless you actually clone |
| `for i in 0..list.len()` | Clippy: use `.iter().enumerate()` | Use iterators |
| `list.iter().map(...).collect::<Vec<_>>()` | Clippy: use `.map().collect()` directly | It's fine, clippy won't warn on this |
| `.clone()` on `&str` or `Copy` types | Clippy: redundant clone | Remove `.clone()` |
| Unused variables `let x = ...` without `_` | Clippy: unused variable | Prefix with `_` or use it |
| `match x { Some(v) => Some(f(v)), None => None }` | Clippy: use `.map()` | Use `x.map(f)` |
| Returning `Option<T>` from function that returns `T` when calling a function that returns `T` directly | Clippy: match can be simplified | Use `?` or `.ok()` |
