📖 **[View on hightechmind.io →](https://hightechmind.io/rust/428-macro-hygiene)**

---

# 428: Macro Hygiene Rules

**Difficulty:** 3  **Level:** Advanced

Variables introduced inside a macro body don't escape into the caller's scope — preventing silent name collisions that would otherwise make macros dangerous to use.

## The Problem This Solves

Imagine writing a `swap!` macro that needs a temporary variable. The naive implementation does `let tmp = $a; $a = $b; $b = tmp;`. If the caller happens to have a variable named `tmp`, you've just silently shadowed it. The caller's code now reads a different value, and the bug is nearly impossible to track down because the source of the shadow is hidden inside the macro.

This is the classic C preprocessor nightmare. A macro like `#define MIN(a,b) ({int tmp=a; tmp<b?tmp:b;})` will wreck any code that uses `tmp` nearby. C developers have worked around this for decades by using obscure names like `_swap_tmp_` — a hack, not a solution.

Rust `macro_rules!` macros are **hygienic by default**. The compiler tracks which syntactic context each identifier comes from. A `let tmp` inside a macro body belongs to the macro's context, not the caller's. The caller's `tmp` is in a different context and is completely unaffected.

## The Intuition

Hygiene means every identifier has a birthplace stamp. When the macro expands, the compiler sees two distinct `tmp`s: one born in the macro definition, one born in the caller's code. They are different variables even though they share a name. The macro's `tmp` is invisible to the caller, and the caller's `tmp` is invisible to the macro's internals.

The one exception: identifiers introduced via `$name:ident` fragments come from the *caller's* context (that's the whole point — the caller is naming something). So `make_counter!(hits)` really does introduce `hits` into the caller's scope. Hygiene lets macros create invisible temporaries while still letting callers choose the names they expose.

## How It Works in Rust

```rust
// Hygienic: the internal 'tmp' cannot clash with caller's 'tmp'
macro_rules! swap {
    ($a:expr, $b:expr) => {
        {
            let tmp = $a;  // this 'tmp' is in MACRO context — invisible to caller
            $a = $b;
            $b = tmp;
        }
    };
}

// Caller has their own 'tmp' — swap! doesn't touch it
let tmp = "I am the caller's tmp";
let mut x = 1;
let mut y = 2;
swap!(x, y);
assert_eq!(tmp, "I am the caller's tmp"); // untouched!
assert_eq!((x, y), (2, 1));              // correctly swapped

// $name:ident comes from CALLER context — it appears in caller's scope
macro_rules! make_counter {
    ($name:ident) => { let mut $name = 0u32; };
}
make_counter!(hits); // introduces 'hits' in the caller's scope
hits += 1;           // valid — 'hits' is in caller scope

// Internal names in other macros don't collide with caller's 'result'
macro_rules! log_and_double {
    ($x:expr) => {{
        let result = $x * 2;  // hygienic — not the caller's 'result'
        println!("{} → {}", stringify!($x), result);
        result
    }};
}
let result = 42;               // caller's 'result'
let doubled = log_and_double!(21); // macro's internal 'result' is separate
assert_eq!(result, 42);        // caller's 'result' unchanged
```

## What This Unlocks

- **Safe utility macros** — write `min!`, `max!`, `swap!`, `dbg_val!` without worrying about name collisions in user code.
- **Internal temporaries** — macros can use descriptive internal names (`result`, `count`, `temp`) without the C convention of ugly prefixes.
- **Proc macro hygiene** — when writing procedural macros with `quote!`, use `Span::call_site()` vs `Span::def_site()` to deliberately choose hygiene level for generated identifiers.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Macro hygiene | Not applicable — OCaml macros (ppx) operate at AST level with explicit name generation | `macro_rules!` hygienic by default; identifiers stamped with definition context |
| Name collision in macros | C-style macros (cpp) in OCaml-C interop are unhygienic | Internal `let` bindings in `macro_rules!` never leak |
| Caller-injected names | `ppx` can splice caller-provided names | `$name:ident` fragment — in caller's context by design |
| Proc macros | N/A | Choose hygiene via `Span::call_site()` / `Span::def_site()` in `proc_macro2` |
