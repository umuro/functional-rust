// 089: Lazy Sequences


#[cfg(test)]
mod tests {
    #[test]
    fn test_naturals() {
        let mut n = 0i32;
        let v: Vec<i32> = std::iter::from_fn(move || { let v = n; n += 1; Some(v) }).take(5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_fibs() {
        let v: Vec<u64> = std::iter::successors(Some((0u64, 1u64)), |&(a, b)| Some((b, a + b)))
            .map(|(a, _)| a).take(8).collect();
        assert_eq!(v, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_powers() {
        let mut exp = 0u32;
        let v: Vec<u64> = std::iter::from_fn(move || {
            if exp >= 4 { None } else { let v = 1u64 << exp; exp += 1; Some(v) }
        }).collect();
        assert_eq!(v, vec![1, 2, 4, 8]);
    }
}
