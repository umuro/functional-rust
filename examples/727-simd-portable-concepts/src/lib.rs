#![allow(clippy::all)]
// 727. SIMD concepts with std::simd (portable_simd)
//
// This file demonstrates the portable_simd API using stable-compatible
// scalar implementations that mirror the SIMD mental model exactly.
// On nightly Rust, replace the scalar impls with std::simd types.
//
// To enable on nightly: add `#![feature(portable_simd)]` and use
// `use std::simd::*;`
//
// The scalar versions below are structurally identical to the SIMD versions —
// just swap `[f32; LANES]` for `f32xN` and loops for SIMD ops.

// ── LANES constant — matches f32x8 ───────────────────────────────────────────

const LANES: usize = 8;

// ── Portable scalar "SIMD" types (mirrors std::simd API) ─────────────────────

/// Simulates `std::simd::f32x8` — 8 f32 lanes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct F32x8([f32; LANES]);

impl F32x8 {
    pub fn splat(v: f32) -> Self {
        Self([v; LANES])
    }
    pub fn from_array(a: [f32; LANES]) -> Self {
        Self(a)
    }
    pub fn to_array(self) -> [f32; LANES] {
        self.0
    }

    /// Lane-wise addition — compiles to VADDPS ymm on AVX2.
    pub fn add(self, rhs: Self) -> Self {
        let mut r = [0.0f32; LANES];
        for i in 0..LANES {
            r[i] = self.0[i] + rhs.0[i];
        }
        Self(r)
    }

    /// Lane-wise multiplication — compiles to VMULPS ymm on AVX2.
    pub fn mul(self, rhs: Self) -> Self {
        let mut r = [0.0f32; LANES];
        for i in 0..LANES {
            r[i] = self.0[i] * rhs.0[i];
        }
        Self(r)
    }

    /// Fused multiply-add: self * a + b — compiles to VFMADD213PS on AVX2+FMA.
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        let mut r = [0.0f32; LANES];
        for i in 0..LANES {
            r[i] = self.0[i].mul_add(a.0[i], b.0[i]);
        }
        Self(r)
    }

    /// Horizontal sum reduction — compiles to `vhaddps` or tree reduction.
    pub fn reduce_sum(self) -> f32 {
        self.0.iter().copied().sum()
    }

    /// Lane-wise max — compiles to VMAXPS ymm.
    pub fn max(self, rhs: Self) -> Self {
        let mut r = [0.0f32; LANES];
        for i in 0..LANES {
            r[i] = self.0[i].max(rhs.0[i]);
        }
        Self(r)
    }

    /// Lane-wise min — compiles to VMINPS ymm.
    pub fn min(self, rhs: Self) -> Self {
        let mut r = [0.0f32; LANES];
        for i in 0..LANES {
            r[i] = self.0[i].min(rhs.0[i]);
        }
        Self(r)
    }

    /// Mask select: choose `on_true[i]` where `mask[i] > 0`, else `on_false[i]`.
    /// Maps to `VBLENDVPS` or `VPBLENDMD` on AVX512.
    pub fn select(mask: &MaskF32x8, on_true: Self, on_false: Self) -> Self {
        let mut r = [0.0f32; LANES];
        for i in 0..LANES {
            r[i] = if mask.0[i] {
                on_true.0[i]
            } else {
                on_false.0[i]
            };
        }
        Self(r)
    }

    /// Compare greater-than, producing a mask.
    pub fn gt(self, rhs: Self) -> MaskF32x8 {
        let mut m = [false; LANES];
        for i in 0..LANES {
            m[i] = self.0[i] > rhs.0[i];
        }
        MaskF32x8(m)
    }
}

/// A boolean mask over 8 f32 lanes. Maps to `std::simd::Mask<i32, 8>`.
#[derive(Clone, Copy, Debug)]
pub struct MaskF32x8([bool; LANES]);

impl MaskF32x8 {
    pub fn any(self) -> bool {
        self.0.iter().any(|&b| b)
    }
    pub fn all(self) -> bool {
        self.0.iter().all(|&b| b)
    }
}

// ── Vectorised algorithms ─────────────────────────────────────────────────────

/// Dot product using 8-wide SIMD accumulation.
/// Processes 8 elements per loop iteration.
pub fn dot_product_simd(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let n = a.len();
    let full_chunks = n / LANES;

    let mut acc = F32x8::splat(0.0);
    for i in 0..full_chunks {
        let off = i * LANES;
        let va = F32x8::from_array(a[off..off + LANES].try_into().unwrap());
        let vb = F32x8::from_array(b[off..off + LANES].try_into().unwrap());
        // acc = va * vb + acc  (FMA)
        acc = va.mul_add(vb, acc);
    }

    // Handle remaining elements (scalar tail)
    let mut result = acc.reduce_sum();
    for i in (full_chunks * LANES)..n {
        result += a[i] * b[i];
    }
    result
}

/// Element-wise clamp using SIMD min/max.
pub fn clamp_simd(data: &mut [f32], lo: f32, hi: f32) {
    let vlo = F32x8::splat(lo);
    let vhi = F32x8::splat(hi);
    let n = data.len();
    let full = n / LANES;

    for i in 0..full {
        let off = i * LANES;
        let v = F32x8::from_array(data[off..off + LANES].try_into().unwrap());
        let clamped = v.max(vlo).min(vhi);
        data[off..off + LANES].copy_from_slice(&clamped.to_array());
    }
    // Scalar tail
    for v in &mut data[full * LANES..] {
        *v = v.clamp(lo, hi);
    }
}

/// ReLU activation: max(x, 0) — common in neural networks.
pub fn relu_simd(data: &mut [f32]) {
    clamp_simd(data, 0.0, f32::INFINITY);
}

/// Horizontal sum of a large slice using 8-wide vectors.
pub fn sum_simd(data: &[f32]) -> f32 {
    let full = data.len() / LANES;
    let mut acc = F32x8::splat(0.0);
    for i in 0..full {
        let off = i * LANES;
        let v = F32x8::from_array(data[off..off + LANES].try_into().unwrap());
        acc = acc.add(v);
    }
    let mut result = acc.reduce_sum();
    for &v in &data[full * LANES..] {
        result += v;
    }
    result
}

// ── Scalar reference implementations ─────────────────────────────────────────

pub fn dot_scalar(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(&x, &y)| x * y).sum()
}

// ── main ──────────────────────────────────────────────────────────────────────

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_add() {
        let a = F32x8::splat(2.0);
        let b = F32x8::splat(3.0);
        assert_eq!(a.add(b).to_array(), [5.0; 8]);
    }

    #[test]
    fn reduce_sum() {
        let a = F32x8::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(a.reduce_sum(), 36.0);
    }

    #[test]
    fn dot_product_matches_scalar() {
        let a: Vec<f32> = (0..64).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..64).map(|i| (64 - i) as f32).collect();
        let d_simd = dot_product_simd(&a, &b);
        let d_scalar = dot_scalar(&a, &b);
        assert!((d_simd - d_scalar).abs() < 0.01);
    }

    #[test]
    fn relu_zeroes_negatives() {
        let mut v = vec![-2.0f32, -1.0, 0.0, 1.0, 2.0, -3.0, 4.0, -0.5];
        relu_simd(&mut v);
        assert_eq!(v, [0.0, 0.0, 0.0, 1.0, 2.0, 0.0, 4.0, 0.0]);
    }

    #[test]
    fn clamp_bounds() {
        let mut v = vec![-5.0f32, 0.0, 5.0, 10.0, 15.0, 3.0, -1.0, 8.0];
        clamp_simd(&mut v, 0.0, 10.0);
        for &x in &v {
            assert!(x >= 0.0 && x <= 10.0);
        }
    }

    #[test]
    fn sum_simd_correct() {
        let data: Vec<f32> = (1..=16).map(|i| i as f32).collect();
        let s = sum_simd(&data);
        assert_eq!(s, 136.0); // 1+2+…+16
    }

    #[test]
    fn mask_select() {
        let a = F32x8::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let threshold = F32x8::splat(4.5);
        let mask = a.gt(threshold);
        let selected = F32x8::select(&mask, F32x8::splat(1.0), F32x8::splat(0.0));
        assert_eq!(&selected.to_array()[..4], &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&selected.to_array()[4..], &[1.0, 1.0, 1.0, 1.0]);
    }
}
