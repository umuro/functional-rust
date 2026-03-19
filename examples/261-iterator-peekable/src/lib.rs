#![allow(clippy::all)]
//! 261. Lookahead with Peekable
//!
//! `Peekable` adds `peek()` to inspect the next element without consuming it.

#[cfg(test)]
mod tests {
    #[test]
    fn test_peek_no_consume() {
        let mut iter = [1i32, 2, 3].iter().peekable();
        let p = iter.peek().copied().copied();
        let n = iter.next().copied();
        assert_eq!(p, Some(1));
        assert_eq!(n, Some(1));
        assert_eq!(iter.next().copied(), Some(2));
    }

    #[test]
    fn test_peek_groups() {
        let data = [1i32, 1, 2, 3, 3];
        let mut iter = data.iter().peekable();
        let mut groups: Vec<Vec<i32>> = Vec::new();
        while let Some(&val) = iter.peek() {
            let mut g = Vec::new();
            while iter.peek() == Some(&val) {
                g.push(*iter.next().unwrap());
            }
            groups.push(g);
        }
        assert_eq!(groups, vec![vec![1, 1], vec![2], vec![3, 3]]);
    }

    #[test]
    fn test_peek_empty() {
        let data: Vec<i32> = vec![];
        let mut p = data.iter().peekable();
        assert_eq!(p.peek(), None);
    }
}
