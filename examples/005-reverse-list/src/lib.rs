// 005: Reverse a List

// Approach 1: Built-in (in-place)
fn reverse_inplace(v: &mut Vec<i32>) {
    v.reverse();
}

// Approach 2: Iterator-based (new Vec)
fn reverse_iter(v: &[i32]) -> Vec<i32> {
    v.iter().rev().copied().collect()
}

// Approach 3: Fold-based (accumulator pattern)
fn reverse_fold(v: &[i32]) -> Vec<i32> {
    v.iter().fold(vec![], |mut acc, &x| {
        acc.insert(0, x);
        acc
    })
}

// Approach 3b: Recursive
fn reverse_recursive(v: &[i32]) -> Vec<i32> {
    if v.is_empty() {
        vec![]
    } else {
        let mut rest = reverse_recursive(&v[1..]);
        rest.push(v[0]);
        rest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_inplace() {
        let mut v = vec![1, 2, 3, 4, 5];
        reverse_inplace(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_iter() {
        assert_eq!(reverse_iter(&[1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
        assert_eq!(reverse_iter(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_reverse_fold() {
        assert_eq!(reverse_fold(&[1, 2, 3]), vec![3, 2, 1]);
    }

    #[test]
    fn test_reverse_recursive() {
        assert_eq!(reverse_recursive(&[1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
        assert_eq!(reverse_recursive(&[42]), vec![42]);
        assert_eq!(reverse_recursive(&[]), Vec::<i32>::new());
    }
}
