# 720: Cache-Friendly Iteration and Data Access Patterns

**Difficulty:** 3  **Level:** Expert

Write iteration patterns the CPU's prefetcher can follow — sequential, predictable, and branch-minimal.

## The Problem This Solves

Modern CPUs execute instructions in ~1 ns, but fetching from main memory takes 60–200 ns. The L1/L2/L3 caches bridge this gap — but only if your access pattern gives the hardware prefetcher a chance to load data before you need it. Unpredictable access patterns (random indices, pointer chasing through linked structures, column-major access on row-major matrices) defeat the prefetcher and cause the CPU to stall waiting for memory.

The problem is invisible in code review. A 1024×1024 matrix sum looks identical whether you iterate row-major or column-major, but the performance difference is 10–50× on real hardware. Rust's zero-cost iterator model doesn't fix cache behaviour — sequential iterators are fast because they're sequential, not because they're idiomatic. Understanding *why* row-major access is fast (and column-major is slow) is essential for any performance-critical Rust code.

## The Intuition

A CPU cache line is 64 bytes. When you load one element, the hardware fetches the surrounding 64 bytes speculatively, betting you'll want the neighbours next. This bet pays off when you iterate sequentially — the next 15 `f32`s are already in cache. It fails catastrophically when you jump by `cols * sizeof(f32)` between accesses — each jump lands in a cold cache line, costing a full memory round-trip.

Rule of thumb: access memory in the order it was laid out. For a row-major matrix (`data[r * cols + c]`), iterate row by row, not column by column. For a struct, process the same field across all instances (SoA layout) rather than all fields of one instance at a time (AoS layout).

## How It Works in Rust

```rust
pub struct Matrix {
    data: Vec<f32>,  // row-major: [row0col0, row0col1, ..., row1col0, ...]
    rows: usize, cols: usize,
}

// ── Fast: sequential access — prefetcher loads ahead ─────────────────────
pub fn sum_row_major(m: &Matrix) -> f64 {
    // Single linear scan — cache line used 100%
    m.data.iter().map(|&v| v as f64).sum()
}

// ── Slow: stride of `cols` between accesses — cold cache lines ───────────
pub fn sum_col_major(m: &Matrix) -> f64 {
    let mut acc = 0.0f64;
    for c in 0..m.cols {
        for r in 0..m.rows {
            // Each access jumps m.cols * 4 bytes — cache miss per element
            acc += m.get(r, c) as f64;
        }
    }
    acc
}

// ── Tiled transpose: access both matrices in cache-friendly blocks ────────
pub fn transpose_tiled(src: &Matrix, dst: &mut Matrix, tile: usize) {
    for row_tile in (0..src.rows).step_by(tile) {
        for col_tile in (0..src.cols).step_by(tile) {
            for r in row_tile..(row_tile + tile).min(src.rows) {
                for c in col_tile..(col_tile + tile).min(src.cols) {
                    // Both reads and writes stay within a tile that fits in L1 cache
                    dst.set(c, r, src.get(r, c));
                }
            }
        }
    }
}
```

Tiling works by choosing a block size (e.g., 64 elements) that fits in L1 cache. You complete all work on that block before moving to the next, so every cache line loaded is used fully before eviction.

## What This Unlocks

- **Matrix operations** — matrix multiply (GEMM) in BLAS uses tiling to achieve near-peak FLOPS; naive row×column multiply is 10–100× slower on large matrices.
- **Image processing** — apply filters in tile-sized chunks (matching cache size) rather than scanning the full image one scanline at a time.
- **Database engines** — columnar databases (DuckDB, Apache Arrow) store data column-by-column explicitly to enable cache-friendly aggregate queries.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Memory layout control | GC controls object placement | `Vec<T>` is always contiguous; layout fully predictable |
| Row-major iteration | `Array` row-by-row (default) | Iterator over flat slice — optimal by default |
| Column-major penalty | Same (GC can't fix it) | Same — it's a hardware limitation, not a language one |
| Tiled algorithms | Implement manually | Implement manually; `std::slice::chunks` helps |
| SIMD opportunity | Limited | Auto-vectorised by LLVM when loop body is simple |
