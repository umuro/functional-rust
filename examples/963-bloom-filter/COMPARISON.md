# Bloom Filter — Comparison

## Core Insight
A Bloom filter is a bit array + k hash functions. `add(x)` sets k bits; `might_contain(x)` checks all k bits are set. Both languages use the same math. The difference: OCaml uses a `bool array` (simple, 1 byte per bit) while Rust uses `Vec<u64>` (8x more space-efficient via bitwise operations). Rust also requires explicit `wrapping_*` arithmetic to avoid overflow panics.

## OCaml Approach
- `bool array` — one bool per bit slot (simple but memory-wasteful)
- `String.fold_left` for hash accumulation
- `abs h mod bf.size` for index calculation
- `bf.bits.(i) <- true` for bit setting
- Integer arrays for compact bit representation (manual bit ops)
- `lsl`, `lor`, `lxor`, `lsr` — OCaml bitwise operators

## Rust Approach
- `Vec<u64>` — 64 bits per word, 8x more compact
- `wrapping_mul`, `wrapping_add`, `wrapping_shl`, `wrapping_sub` — explicit overflow handling
- `bits[word] |= 1u64 << bit` — set a bit in a u64 word
- `(bits[word] >> bit) & 1 == 1` — test a bit
- `count_ones()` on u64 — popcount (hardware-accelerated)
- FP rate formula: `(1 - e^(-k*n/m))^k`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Bit storage | `bool array` (1 byte/bit) | `Vec<u64>` (1 bit/bit) |
| Hash fold | `String.fold_left (fun h c -> h*31 + code c)` | `s.bytes().fold(h, \|h, b\| h.wrapping_mul(31).wrapping_add(b))` |
| Overflow | Silent (OCaml int wraps) | Explicit `wrapping_*` methods |
| Bit set | `arr.(i) <- true` | `arr[word] \|= 1u64 << bit` |
| Bit test | `arr.(i)` | `(arr[word] >> bit) & 1 == 1` |
| Popcount | Manual count | `u64::count_ones()` (hardware) |
| Bitwise ops | `lsl`, `lor`, `lxor` | `<<`, `\|`, `^`, `>>` |
