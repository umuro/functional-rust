// 094: Peekable Iterator

fn dedup(v: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut iter = v.iter().peekable();
    while let Some(&val) = iter.next() {
        result.push(val);
        while iter.peek() == Some(&&val) { iter.next(); }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup() {
        assert_eq!(dedup(&[1, 1, 2, 2, 2, 3, 3, 1]), vec![1, 2, 3, 1]);
        assert_eq!(dedup(&[]), Vec::<i32>::new());
        assert_eq!(dedup(&[5]), vec![5]);
    }

    #[test]
    fn test_peekable() {
        let mut iter = [1, 2, 3].iter().peekable();
        assert_eq!(iter.peek(), Some(&&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.peek(), Some(&&2));
    }
}
