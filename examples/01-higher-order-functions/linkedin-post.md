# LinkedIn Post: Functional Rust #1 - Higher-Order Functions

---

🦀 **Functional Rust #1: Higher-Order Functions**

Coming from OCaml? Here's how `map`, `filter`, and `fold` translate to Rust.

**OCaml:**
```ocaml
let rec map f = function
  | [] -> []
  | x :: xs -> f x :: map f xs
```

**Rust:**
```rust
fn map<T, U, F>(f: F, list: &[T]) -> Vec<U>
where F: Fn(&T) -> U
{
    match list {
        [] => vec![],
        [x, xs @ ..] => {
            let mut result = vec![f(x)];
            result.extend(map(f, xs));
            result
        }
    }
}
```

**Key insight:** The recursion pattern stays identical. Rust adds explicit generics and ownership, but the FP logic is preserved.

**Why this matters:** Higher-order functions are the foundation of functional programming. Once you master map/filter/fold, you unlock composable, reusable abstractions that transform how you think about data processing.

Full example + explanation: [website link]

#rust #functionalprogramming #ocaml #programming #rustlang #fp

---

## Posting Instructions

1. **Copy text above** (excluding "LinkedIn Post:" header and instructions)
2. **Add code formatting** (LinkedIn supports code blocks with triple backticks)
3. **Post to:**
   - Your LinkedIn profile
   - Rust Programming group (28.9K members)
4. **Add website link** once deployed
5. **Monitor engagement:**
   - Reply to comments within 1 hour
   - Answer questions thoughtfully
   - Thank people for likes/shares
6. **Track metrics:**
   - Likes, comments, shares
   - Profile views increase
   - Connection requests from Rust developers

## Timing
- **Best time:** Thursday 8:30 AM EST (validated for r/rust, likely similar for LinkedIn)
- **Frequency:** 1 example/week initially
- **Duration:** Active engagement for first 24 hours

## Hashtag Strategy
- `#rust` - Primary (broad reach)
- `#functionalprogramming` - Secondary (target audience)
- `#ocaml` - Tertiary (catch OCaml devs learning Rust)
- `#programming` - General reach
- `#rustlang` - Rust-specific
- `#fp` - FP community

---

*Part of Functional Rust series - translating OCaml FP concepts to idiomatic Rust.*
