// 097: Flatten and Flat Map


#[cfg(test)]
mod tests {
    #[test]
    fn test_flatten() {
        let v: Vec<i32> = vec![vec![1,2], vec![3,4], vec![5]].into_iter().flatten().collect();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_flat_map() {
        let v: Vec<i32> = [1, 2, 3].iter().flat_map(|&x| vec![x, x * 10]).collect();
        assert_eq!(v, vec![1, 10, 2, 20, 3, 30]);
    }

    #[test]
    fn test_flatten_empty() {
        let v: Vec<i32> = vec![vec![], vec![1], vec![], vec![2,3]].into_iter().flatten().collect();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_flatten_options() {
        let v: Vec<i32> = [Some(1), None, Some(3)].iter().flatten().copied().collect();
        assert_eq!(v, vec![1, 3]);
    }
}
