// 092: Scan with Accumulator

fn running_sum(v: &[i32]) -> Vec<i32> {
    let mut result = vec![0];
    result.extend(v.iter().scan(0, |acc, &x| {
        *acc += x;
        Some(*acc)
    }));
    result
}

fn running_max(v: &[i32]) -> Vec<i32> {
    if v.is_empty() {
        return vec![];
    }
    let mut max_val = v[0];
    let mut result = vec![max_val];
    for &x in &v[1..] {
        max_val = max_val.max(x);
        result.push(max_val);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(&[1, 2, 3, 4]), vec![0, 1, 3, 6, 10]);
    }

    #[test]
    fn test_running_max() {
        assert_eq!(running_max(&[3, 1, 4, 1, 5]), vec![3, 3, 4, 4, 5]);
    }
}
