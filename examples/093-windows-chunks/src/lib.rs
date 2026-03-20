#![allow(clippy::all)]
// 093: Windows and Chunks

#[cfg(test)]
mod tests {
    #[test]
    fn test_windows() {
        let v = vec![1, 2, 3, 4, 5];
        let w: Vec<&[i32]> = v.windows(3).collect();
        assert_eq!(w, vec![&[1, 2, 3][..], &[2, 3, 4][..], &[3, 4, 5][..]]);
    }

    #[test]
    fn test_windows_2() {
        let v = vec![1, 2, 3];
        let w: Vec<&[i32]> = v.windows(2).collect();
        assert_eq!(w, vec![&[1, 2][..], &[2, 3][..]]);
    }

    #[test]
    fn test_chunks() {
        let v = vec![1, 2, 3, 4, 5];
        let c: Vec<&[i32]> = v.chunks(2).collect();
        assert_eq!(c, vec![&[1, 2][..], &[3, 4][..], &[5][..]]);
    }

    #[test]
    fn test_chunks_exact() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let c: Vec<&[i32]> = v.chunks(3).collect();
        assert_eq!(c, vec![&[1, 2, 3][..], &[4, 5, 6][..]]);
    }
}
