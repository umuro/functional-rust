// 002: List Operations
// Core list operations: head, tail, length, append, reverse

// Approach 1: Vec methods
fn head(v: &[i32]) -> Option<&i32> {
    v.first()
}

fn tail(v: &[i32]) -> Option<&[i32]> {
    if v.is_empty() { None } else { Some(&v[1..]) }
}

fn length(v: &[i32]) -> usize {
    v.len()
}

fn append(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = a.to_vec();
    result.extend_from_slice(b);
    result
}

fn reverse(v: &[i32]) -> Vec<i32> {
    v.iter().rev().copied().collect()
}

// Approach 2: Recursive (functional style)
fn rec_length(v: &[i32]) -> usize {
    if v.is_empty() { 0 } else { 1 + rec_length(&v[1..]) }
}

fn rec_reverse(v: &[i32]) -> Vec<i32> {
    if v.is_empty() {
        vec![]
    } else {
        let mut rest = rec_reverse(&v[1..]);
        rest.push(v[0]);
        rest
    }
}

// Approach 3: Tail-recursive with accumulator
fn rev_acc(v: &[i32]) -> Vec<i32> {
    fn aux(slice: &[i32], acc: Vec<i32>) -> Vec<i32> {
        if slice.is_empty() {
            acc
        } else {
            let mut new_acc = vec![slice[0]];
            new_acc.extend(acc);
            aux(&slice[1..], new_acc)
        }
    }
    aux(v, vec![])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head() {
        assert_eq!(head(&[1, 2, 3]), Some(&1));
        assert_eq!(head(&[]), None);
    }

    #[test]
    fn test_tail() {
        assert_eq!(tail(&[1, 2, 3]), Some([2, 3].as_slice()));
        assert_eq!(tail(&[]), None);
    }

    #[test]
    fn test_length() {
        assert_eq!(length(&[1, 2, 3]), 3);
        assert_eq!(length(&[]), 0);
    }

    #[test]
    fn test_append() {
        assert_eq!(append(&[1, 2], &[3, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(&[1, 2, 3]), vec![3, 2, 1]);
    }

    #[test]
    fn test_rec_length() {
        assert_eq!(rec_length(&[1, 2, 3, 4]), 4);
    }

    #[test]
    fn test_rec_reverse() {
        assert_eq!(rec_reverse(&[1, 2, 3]), vec![3, 2, 1]);
    }

    #[test]
    fn test_rev_acc() {
        assert_eq!(rev_acc(&[1, 2, 3]), vec![3, 2, 1]);
    }
}
