📖 **[View on hightechmind.io →](https://hightechmind.io/rust/539-lifetime-nll)**

---

# 539: Non-Lexical Lifetimes (NLL)

**Difficulty:** 3  **Level:** Intermediate

Since Rust 2018, borrows end when they're last *used* — not at the closing brace of their scope. This eliminates a large class of false-positive borrow errors.

## The Problem This Solves

The pre-NLL borrow checker was lexical — it extended borrows to the end of the enclosing block, even if the borrow was never used again after the first line. This produced infuriating false errors:

```rust
// Pre-2018 Rust — this failed:
let mut v = vec![1, 2, 3];
let first = &v[0];      // shared borrow of v — starts here
// ... never use `first` again ...
v.push(4);              // ERROR: cannot mutate while borrowed
                        // (even though `first` is never used after this!)
println!("{}", first);  // borrow was "kept alive" until here by old rules
```

The error was technically sound under lexical lifetimes, but practically wrong — the code is safe because `first` and `push` don't actually overlap. NLL made the compiler smart enough to see that.

## The Intuition

NLL changes the lifetime model from "the borrow lives until the end of the block" to "the borrow lives until the last use of the reference." It's a more precise analysis.

Think of a borrow like a library checkout. Lexical lifetimes meant you kept the book checked out until you left the building, even if you finished reading on page 10. NLL means you return it as soon as you're done — opening the shelf for others immediately.

This is not a language change in terms of safety. All the same rules apply — no dangling references, no aliasing. It's a precision improvement: the compiler accepts more valid programs without accepting any invalid ones.

## How It Works in Rust

**Basic NLL — borrow ends at last use:**

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = v[0];  // Copy type — no borrow actually held here
// With NLL, even a shared borrow ends here if not used again:
v.push(6);         // OK — borrow already ended
println!("first: {}, v: {:?}", first, v);
```

**NLL in loops — read then mutate:**

```rust
let mut map = std::collections::HashMap::new();
map.insert("key", 0i32);

for _ in 0..5 {
    let current = *map.get("key").unwrap(); // shared borrow — ends here
    // borrow ended (last use was `*` dereference)
    map.insert("key", current + 1);         // mutable borrow — OK
}
```

**NLL with conditional returns:**

```rust
fn process(s: &mut String) -> String {
    let r = s.as_str(); // borrow starts
    if r.len() > 3 {
        return r.to_uppercase(); // r used here — borrow ends via this return
    }
    // borrow ended (the `if` branch returned or was skipped)
    s.push_str(" world"); // mutation OK — borrow has ended
    s.clone()
}
```

**The case NLL *doesn't* fix — actual overlap:**

```rust
let mut v = vec![1, 2, 3];
let r = &v[0];    // borrow starts
v.push(4);        // ERROR even with NLL — r is still live!
println!("{}", r);// r used here — this is the actual last use
// Solution: move println! before push, or drop r before push
```

## What This Unlocks

- **Read-then-mutate patterns** — lookup a value, compute something with it, then modify the collection. NLL makes the common "get or insert" pattern safe without workarounds.
- **Loops that read and write the same structure** — per-iteration borrows end at the end of each iteration, not at the end of the loop block.
- **Cleaner code without artificial scoping** — no more wrapping reads in extra `{}` blocks just to end a borrow before a mutation on the same line.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reference validity | GC manages all references — no analysis needed | NLL: borrow ends at last use, not end of scope |
| Borrow scope | N/A (GC) | Lexical (pre-2018): end of block. NLL (post-2018): end of last use |
| Mutation after read | Always allowed (with care) | Allowed once the last shared borrow has been used — NLL detects this |
| False positives | N/A | NLL eliminated a large class of valid programs that old borrow checker rejected |
| Analysis model | No borrow checker | Control-flow analysis: tracks borrows through branches, loops, early returns |
