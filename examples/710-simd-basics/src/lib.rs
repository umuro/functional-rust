//! # SIMD Basics

/// Vectorized sum (conceptual, real SIMD uses std::simd on nightly)
pub fn sum_vectorized(arr: &[f32]) -> f32 {
    arr.iter().sum()
}

/// Dot product
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Element-wise add
pub fn add_arrays(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

/// Check if architecture supports AVX
#[cfg(target_arch = "x86_64")]
pub fn has_avx() -> bool { is_x86_feature_detected!("avx") }

#[cfg(not(target_arch = "x86_64"))]
pub fn has_avx() -> bool { false }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dot() { assert_eq!(dot_product(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]), 32.0); }
    #[test]
    fn test_add() { assert_eq!(add_arrays(&[1.0, 2.0], &[3.0, 4.0]), vec![4.0, 6.0]); }
}
