// 090: Infinite Iterators — cycle, repeat, from_fn

#[cfg(test)]
mod tests {
    #[test]
    fn test_cycle() {
        let v: Vec<i32> = [1, 2, 3].iter().copied().cycle().take(7).collect();
        assert_eq!(v, vec![1, 2, 3, 1, 2, 3, 1]);
    }

    #[test]
    fn test_repeat() {
        let v: Vec<i32> = std::iter::repeat(42).take(4).collect();
        assert_eq!(v, vec![42, 42, 42, 42]);
    }

    #[test]
    fn test_from_fn() {
        let mut n = 0i32;
        let v: Vec<i32> = std::iter::from_fn(move || {
            let v = n;
            n += 1;
            Some(v)
        })
        .take(5)
        .collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_repeat_with() {
        let mut c = 0;
        let v: Vec<i32> = std::iter::repeat_with(move || {
            c += 1;
            c * c
        })
        .take(4)
        .collect();
        assert_eq!(v, vec![1, 4, 9, 16]);
    }
}
