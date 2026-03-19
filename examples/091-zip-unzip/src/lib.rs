// 091: Zip and Unzip

#[cfg(test)]
mod tests {
    #[test]
    fn test_zip() {
        let v: Vec<_> = [1, 2, 3].iter().zip(["a", "b", "c"].iter()).collect();
        assert_eq!(v, vec![(&1, &"a"), (&2, &"b"), (&3, &"c")]);
    }

    #[test]
    fn test_zip_unequal() {
        let v: Vec<_> = [1, 2].iter().zip([10, 20, 30].iter()).collect();
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_unzip() {
        let (a, b): (Vec<i32>, Vec<&str>) = vec![(1, "a"), (2, "b")].into_iter().unzip();
        assert_eq!(a, vec![1, 2]);
        assert_eq!(b, vec!["a", "b"]);
    }

    #[test]
    fn test_zip_with() {
        let v: Vec<i32> = [1, 2, 3]
            .iter()
            .zip([10, 20, 30].iter())
            .map(|(a, b)| a + b)
            .collect();
        assert_eq!(v, vec![11, 22, 33]);
    }
}
