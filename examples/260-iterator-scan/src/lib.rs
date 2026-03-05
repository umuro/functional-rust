//! 260. Stateful accumulation with scan()
//!
//! `scan()` is like `fold` but emits each intermediate state as an iterator element.

#[cfg(test)]
mod tests {
    #[test]
    fn test_scan_running_sum() {
        let result: Vec<i32> = [1, 2, 3, 4, 5].iter()
            .scan(0i32, |s, &x| { *s += x; Some(*s) })
            .collect();
        assert_eq!(result, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_scan_early_stop() {
        let result: Vec<i32> = [1, 2, 3, 4, 5].iter()
            .scan(0i32, |s, &x| { *s += x; if *s > 6 { None } else { Some(*s) } })
            .collect();
        assert_eq!(result, vec![1, 3, 6]);
    }

    #[test]
    fn test_scan_product() {
        let result: Vec<i32> = [1, 2, 3, 4].iter()
            .scan(1i32, |s, &x| { *s *= x; Some(*s) })
            .collect();
        assert_eq!(result, vec![1, 2, 6, 24]);
    }
}
