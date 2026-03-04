# 719: Struct of Arrays vs Array of Structs

**Difficulty:** 3  **Level:** Expert

Lay out data for the CPU's cache, not for object-oriented convenience.

## The Problem This Solves

The natural object-oriented layout — one struct per entity, all fields together — is called Array of Structures (AoS): `Vec<Particle>` where each `Particle` has `{x, y, z, mass, vx, vy, vz}`. This is easy to read and reason about, but it punishes the CPU when you process a single field across many entities.

When you iterate over the `x` field of 10,000 particles, the CPU fetches a 64-byte cache line for each particle — but only 4 bytes of it (`x`) are useful. The rest (`y`, `z`, `mass`, `vx`, `vy`, `vz`) are loaded into cache and immediately ignored. Effective cache utilisation: 4/32 = 12.5%. Every cache miss stalls the CPU pipeline and costs 100–300 ns.

Structure of Arrays (SoA) inverts the layout: `{xs: Vec<f32>, ys: Vec<f32>, zs: Vec<f32>, ...}`. When you iterate over `xs`, every byte in every cache line is the field you want. Effective cache utilisation: 100%. This single layout change routinely delivers 2–10× throughput improvements on field-processing workloads.

## The Intuition

Think of the memory layout as a spreadsheet. AoS is row-per-entity: each row is one particle, columns are fields. SoA is column-per-field: each column is one field for all particles.

When you process one column (one field across all particles), the SoA layout is a sequential scan of contiguous memory. The CPU prefetcher predicts the pattern and loads the next cache line before you ask for it. AoS forces you to skip 28 bytes after every 4-byte read — the prefetcher can't keep up, and you pay the cache-miss penalty on every element.

The trade-off: SoA makes single-field processing faster, but accessing all fields for one entity (particle.x, particle.y, particle.z together) becomes a scattered access across three separate arrays. Know your hot loop before choosing a layout.

## How It Works in Rust

```rust
// ── Array of Structs (AoS) — natural, but cache-hostile for field loops ─
#[derive(Clone, Default)]
pub struct ParticleAoS {
    pub x: f32, pub y: f32, pub z: f32,
    pub mass: f32, pub vx: f32, pub vy: f32, pub vz: f32,
}

// 87.5% of each cache line wasted when iterating only `x`:
pub fn sum_x_aos(particles: &[ParticleAoS]) -> f32 {
    particles.iter().map(|p| p.x).sum()
}

// ── Structure of Arrays (SoA) — cache-optimal for field processing ───────
pub struct ParticlesSoA {
    pub x:    Vec<f32>,  // all x-coordinates, contiguous
    pub y:    Vec<f32>,  // all y-coordinates, contiguous
    pub z:    Vec<f32>,
    pub mass: Vec<f32>,
    pub vx:   Vec<f32>,
    pub vy:   Vec<f32>,
    pub vz:   Vec<f32>,
}

// 100% cache utilisation — xs is a dense f32 array:
pub fn sum_x_soa(p: &ParticlesSoA) -> f32 {
    p.x.iter().sum()
}

// SIMD-vectorisable — LLVM sees a tight loop over f32 slices:
pub fn apply_gravity_soa(p: &mut ParticlesSoA, dt: f32) {
    for (vy, &mass) in p.vy.iter_mut().zip(p.mass.iter()) {
        *vy -= 9.81 * mass * dt;  // no stride, no wasted bandwidth
    }
}
```

## What This Unlocks

- **Game engines and physics simulations** — Unity's ECS, Bevy's ECS, and most AAA game engines default to SoA internally for particle systems, rigid bodies, and character controllers.
- **Scientific computing** — NumPy's column-wise operations, pandas DataFrames, and Apache Arrow all use SoA (columnar) layout for the same reason.
- **SIMD and auto-vectorisation** — LLVM can auto-vectorise tight loops over `&[f32]` slices into SSE/AVX instructions, processing 4–16 floats per instruction cycle.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default layout | Records (AoS equivalent, GC-managed) | Struct-per-element (AoS) unless redesigned |
| SoA equivalent | Arrays of separate float arrays | `struct { xs: Vec<f32>, ys: Vec<f32>, ... }` |
| Cache control | No control (GC moves objects) | Full control: Vec<T> is always contiguous, predictable |
| SIMD opportunity | Limited (Owl library, C bindings) | Direct via `std::arch` or auto-vectorisation |
| Hot loop optimisation | Rare due to GC overhead | Standard practice in performance-critical Rust |
