//! # Affine Traversal
//! Focus on at most one element.

pub fn affine_get<T>(v: &[T]) -> Option<&T> { v.first() }
pub fn affine_set<T: Clone>(v: &[T], t: T) -> Vec<T> {
    if v.is_empty() { vec![] } else { let mut r = v.to_vec(); r[0] = t; r }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_affine() {
        let v = vec![1, 2, 3];
        assert_eq!(affine_get(&v), Some(&1));
        assert_eq!(affine_set(&v, 10), vec![10, 2, 3]);
    }
}
