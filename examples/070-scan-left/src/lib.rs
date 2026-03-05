// 070: Scan Left — running accumulation

// Approach 1: Using .scan() iterator
fn running_sum(v: &[i32]) -> Vec<i32> {
    let mut result = vec![0];
    result.extend(v.iter().scan(0, |state, &x| {
        *state += x;
        Some(*state)
    }));
    result
}

// Approach 2: Manual scan function
fn scan_left<T: Clone, F>(init: T, v: &[T], f: F) -> Vec<T>
where
    F: Fn(&T, &T) -> T,
{
    let mut result = vec![init.clone()];
    let mut acc = init;
    for x in v {
        acc = f(&acc, x);
        result.push(acc.clone());
    }
    result
}

fn running_product(v: &[i32]) -> Vec<i32> {
    scan_left(1, v, |a, b| a * b)
}

// Approach 3: Running max
fn running_max(v: &[i32]) -> Vec<i32> {
    if v.is_empty() { return vec![]; }
    let mut result = vec![v[0]];
    let mut current_max = v[0];
    for &x in &v[1..] {
        current_max = current_max.max(x);
        result.push(current_max);
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(&[1, 2, 3, 4]), vec![0, 1, 3, 6, 10]);
        assert_eq!(running_sum(&[]), vec![0]);
    }

    #[test]
    fn test_running_product() {
        assert_eq!(running_product(&[1, 2, 3, 4]), vec![1, 1, 2, 6, 24]);
    }

    #[test]
    fn test_scan_left() {
        assert_eq!(scan_left(0, &vec![1, 2, 3], |a, b| a + b), vec![0, 1, 3, 6]);
    }

    #[test]
    fn test_running_max() {
        assert_eq!(running_max(&[3, 1, 4, 1, 5, 9]), vec![3, 3, 4, 4, 5, 9]);
        assert_eq!(running_max(&[]), Vec::<i32>::new());
    }
}
