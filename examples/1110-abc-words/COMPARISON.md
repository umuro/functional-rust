# OCaml vs Rust: ABC Words

## Side-by-Side Code

### OCaml

```ocaml
let is_abc_word (word : string) : bool =
  try
    String.index word 'a'
    |> fun i -> String.index_from word i 'b'
    |> fun i -> String.index_from word i 'c'
    |> ignore; true
  with Not_found -> false
```

### Rust (idiomatic — stateful iterator)

```rust
pub fn is_abc_word(word: &str) -> bool {
    let mut chars = word.chars();
    chars.any(|c| c == 'a') && chars.any(|c| c == 'b') && chars.any(|c| c == 'c')
}
```

### Rust (fold — accumulator replaces mutable index)

```rust
pub fn is_abc_word_fold(word: &str) -> bool {
    const TARGET: [char; 3] = ['a', 'b', 'c'];
    word.chars().fold(0usize, |idx, ch| {
        if idx < TARGET.len() && ch == TARGET[idx] { idx + 1 } else { idx }
    }) == TARGET.len()
}
```

### Rust (recursive — closest to OCaml pattern matching)

```rust
pub fn is_abc_word_recursive(word: &str) -> bool {
    fn find_seq(chars: &[char], seq: &[char]) -> bool {
        match (chars, seq) {
            (_, [])    => true,
            ([], _)    => false,
            ([ch, rest @ ..], [target, remaining @ ..]) if ch == target =>
                find_seq(rest, remaining),
            ([_, rest @ ..], _) => find_seq(rest, seq),
        }
    }
    let chars: Vec<char> = word.chars().collect();
    find_seq(&chars, &['a', 'b', 'c'])
}
```

## Type Signatures

| Concept             | OCaml                            | Rust                              |
|---------------------|----------------------------------|-----------------------------------|
| Function signature  | `val is_abc_word : string -> bool` | `fn is_abc_word(word: &str) -> bool` |
| String type         | `string` (mutable byte sequence) | `&str` (immutable UTF-8 slice)    |
| Optional index      | `int` (or `Not_found` exception) | `bool` from `.any()`              |
| Exception handling  | `try ... with Not_found -> false` | Short-circuit `&&` — no exceptions |
| List filtering      | `List.filter is_abc_word`        | `.iter().copied().filter(is_abc_word).collect()` |

## Key Insights

1. **Iterator statefulness is the key insight.** OCaml threads the found index
   explicitly through the pipeline (`|> fun i -> String.index_from word i 'b'`).
   Rust's `chars()` iterator maintains its own cursor — after `.any()` returns
   `true`, the cursor sits immediately after the matched character. The next
   `.any()` call automatically searches only the remaining suffix. No explicit
   position variable is needed.

2. **Exception-free control flow.** OCaml's `String.index` raises `Not_found` when
   a letter is absent; the `try/with` wrapper converts this into a `bool`. Rust
   avoids exceptions entirely: `Iterator::any()` returns `false` when exhausted,
   and `&&` short-circuits — cleanly expressing "failed to find letter" without
   a separate error type or exception handler.

3. **Zero-copy vs. index arithmetic.** OCaml's `String.index_from word i 'c'`
   searches the original string from index `i`, doing byte arithmetic on a shared
   buffer. Rust's `chars()` iterator is also zero-copy (no substring allocation),
   but models position as iterator state rather than a raw integer — safer and
   more composable.

4. **Fold as frozen state machine.** The fold implementation models the search as
   a tiny state machine: the accumulator `idx` tracks "how many target letters have
   been matched so far." This is idiomatic functional style — pure accumulation
   rather than mutation — and maps naturally to a `scan` + `last` or `try_fold`.

5. **Slice patterns mirror algebraic types.** The recursive implementation uses
   Rust's slice patterns (`[ch, rest @ ..]`) to decompose a `&[char]` exactly as
   OCaml would decompose a `char list`. The `@ ..` rest binding is the direct
   counterpart to OCaml's `_ :: rest`.

## When to Use Each Style

**Use idiomatic Rust (stateful iterator)** when searching for a subsequence in a
stream: it is the most concise, allocates nothing, and composes well with other
iterator adapters. Prefer this in production code.

**Use the fold version** when you need the search to be expressible as a pure
function of accumulated state — for example, inside a `rayon` parallel fold, or
when you want to count matches rather than just detect them.

**Use the recursive version** when translating OCaml or Haskell code directly and
want the structural correspondence to be visually obvious — useful for teaching or
for verifying that a port is correct before optimizing.
