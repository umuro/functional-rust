//! Region Inference
//!
//! How the compiler infers lifetime regions.

/// Compiler infers minimal region for borrow.
pub fn inferred_region() {
    let mut x = 5;
    let r = &x;     // region starts
    let _ = *r;     // region ends (last use)
    x = 10;         // OK: region ended
    assert_eq!(x, 10);
}

/// Region spans usage.
pub fn region_span(data: &[i32]) -> i32 {
    // 'a region covers entire function
    data.iter().sum()
}

/// Nested regions.
pub fn nested_regions(v: &mut Vec<i32>) {
    {
        let r = &v[0..2];  // inner region
        let _ = r.len();
    }
    v.push(10);  // outer region continues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inferred() {
        inferred_region();
    }

    #[test]
    fn test_span() {
        let data = [1, 2, 3];
        assert_eq!(region_span(&data), 6);
    }

    #[test]
    fn test_nested() {
        let mut v = vec![1, 2, 3];
        nested_regions(&mut v);
        assert_eq!(v.len(), 4);
    }
}
