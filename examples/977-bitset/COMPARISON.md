# Bitset — Comparison

## Core Insight
A bitset stores a set of integers compactly: 1 bit per element. Word-level operations (`|`, `&`, `^`, `~`) apply to 64 bits simultaneously. OCaml's `int` is 63-bit (the runtime uses bit 0 as a GC tag for boxing), so each word holds 63 bits. Rust uses `u64` (all 64 bits). Rust also has hardware-accelerated `count_ones()` and `trailing_zeros()` as methods.

## OCaml Approach
- `int array` — 63 bits per word (OCaml int is 63-bit on 64-bit systems)
- `lsl`, `lor`, `land`, `lxor`, `lnot` — OCaml bitwise operators
- `words_for_bits n = (n + 62) / 63` — accounts for 63-bit words
- Manual popcount via `while w <> 0 do w := w land (w-1); incr count done`
- `Array.fold_left` for counting across all words
- Set/clear/toggle return unit (mutate in place)

## Rust Approach
- `Vec<u64>` — 64 bits per word (full u64, no GC tag bit)
- `|=`, `&=`, `^=`, `!` — Rust bitwise operators (same semantics)
- `(size + 63) / 64` words needed
- `word.count_ones()` — hardware POPCNT instruction
- `word.trailing_zeros()` — hardware BSF/CTZ for finding lowest set bit
- `word &= word - 1` — clear lowest set bit trick (same as OCaml)
- `.iter().zip(&other.bits).map(|(a,b)| a | b).collect()` for set operations

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Word type | `int` (63-bit) | `u64` (64-bit) |
| Words per N bits | `(N+62)/63` | `(N+63)/64` |
| Bitwise AND | `land` | `&` |
| Bitwise OR | `lor` | `\|` |
| Bitwise XOR | `lxor` | `^` |
| Bitwise NOT | `lnot` | `!` |
| Left shift | `lsl` | `<<` |
| Right shift | `lsr` | `>>` |
| Popcount | Manual Brian Kernighan loop | `u64::count_ones()` (hardware) |
| Lowest set bit | `w land (w - 1)` clear | `w.trailing_zeros()` + `w &= w-1` |
| Set operations | `for i = 0 to n-1 do result.(i) <- a.(i) lor b.(i) done` | `a.iter().zip(b.iter()).map(\|(a,b)\| a\|b).collect()` |
