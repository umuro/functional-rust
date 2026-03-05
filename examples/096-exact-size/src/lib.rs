// 096: Exact Size Iterator


#[cfg(test)]
mod tests {
    #[test]
    fn test_exact_size() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.iter().len(), 5);
        assert_eq!((0..10).len(), 10);
    }

    #[test]
    fn test_enumerate_len() {
        let v = vec!["a", "b", "c"];
        let e: Vec<_> = v.iter().enumerate().collect();
        assert_eq!(e, vec![(0, &"a"), (1, &"b"), (2, &"c")]);
    }

    #[test]
    fn test_chunks_exact() {
        let v = vec![1, 2, 3, 4, 5];
        let c: Vec<&[i32]> = v.chunks_exact(2).collect();
        assert_eq!(c, vec![&[1,2][..], &[3,4][..]]);
        assert_eq!(v.chunks_exact(2).remainder(), &[5]);
    }
}
