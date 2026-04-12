/// Idiomatic Rust: use slice::copy_from_slice with subslice indexing.
/// Copies `len` elements from `src[src_pos..]` into `dst[dst_pos..]`.
///
/// # Panics
/// Panics if the source or destination slices are out of bounds.
pub fn array_blit<T: Copy>(src: &[T], src_pos: usize, dst: &mut [T], dst_pos: usize, len: usize) {
    dst[dst_pos..dst_pos + len].copy_from_slice(&src[src_pos..src_pos + len]);
}

/// Functional style: build the result without mutating in place.
/// Returns a new Vec with the blitted region applied.
pub fn array_blit_functional<T: Copy>(
    src: &[T],
    src_pos: usize,
    dst: &[T],
    dst_pos: usize,
    len: usize,
) -> Vec<T> {
    let mut result = dst.to_vec();
    result[dst_pos..dst_pos + len].copy_from_slice(&src[src_pos..src_pos + len]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blit_middle() {
        let src = [10, 20, 30, 40, 50];
        let mut dst = [0i32; 8];
        array_blit(&src, 1, &mut dst, 2, 3);
        assert_eq!(dst, [0, 0, 20, 30, 40, 0, 0, 0]);
    }

    #[test]
    fn test_blit_start_to_start() {
        let src = [1, 2, 3];
        let mut dst = [0i32; 3];
        array_blit(&src, 0, &mut dst, 0, 3);
        assert_eq!(dst, [1, 2, 3]);
    }

    #[test]
    fn test_blit_single_element() {
        let src = [99];
        let mut dst = [0i32; 5];
        array_blit(&src, 0, &mut dst, 3, 1);
        assert_eq!(dst, [0, 0, 0, 99, 0]);
    }

    #[test]
    fn test_blit_zero_length() {
        let src = [1, 2, 3];
        let mut dst = [7i32; 4];
        array_blit(&src, 0, &mut dst, 0, 0);
        assert_eq!(dst, [7, 7, 7, 7]);
    }

    #[test]
    fn test_functional_blit() {
        let src = [10, 20, 30, 40, 50];
        let dst = [0i32; 8];
        let result = array_blit_functional(&src, 1, &dst, 2, 3);
        assert_eq!(result, vec![0, 0, 20, 30, 40, 0, 0, 0]);
    }

    #[test]
    fn test_functional_blit_does_not_mutate_original() {
        let src = [10, 20, 30];
        let dst = [0i32; 5];
        let result = array_blit_functional(&src, 0, &dst, 1, 3);
        assert_eq!(result, vec![0, 10, 20, 30, 0]);
        assert_eq!(dst, [0, 0, 0, 0, 0]);
    }
}
