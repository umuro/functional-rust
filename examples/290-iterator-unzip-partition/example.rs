//! 290. Advanced splitting patterns
//!
//! Advanced unzip and partition patterns for multi-group splitting.

fn main() {
    // partition_map pattern: split by transformation type
    let data = vec!["1", "two", "3", "four", "5"];
    let (nums, words): (Vec<i32>, Vec<&&str>) = data.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut ns, mut ws), s| {
            match s.parse::<i32>() {
                Ok(n) => ns.push(n),
                Err(_) => ws.push(s),
            }
            (ns, ws)
        }
    );
    println!("Numbers: {:?}", nums);
    println!("Words: {:?}", words);

    // Trisect: negative, zero, positive
    let nums = [-3i32, 0, 1, -1, 0, 5, -2, 3];
    let (neg, non_neg): (Vec<i32>, Vec<i32>) = nums.iter().copied().partition(|&x| x < 0);
    let (zero, pos): (Vec<i32>, Vec<i32>) = non_neg.into_iter().partition(|&x| x == 0);
    println!("Neg: {:?}, Zero: {:?}, Pos: {:?}", neg, zero, pos);

    // Unzip nested pairs
    let nested: Vec<((i32, i32), &str)> = vec![((1, 2), "a"), ((3, 4), "b")];
    let (pairs, labels): (Vec<(i32,i32)>, Vec<&str>) = nested.into_iter().unzip();
    let (lefts, rights): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();
    println!("Lefts: {:?}, Rights: {:?}, Labels: {:?}", lefts, rights, labels);

    // Multi-partition: categorize by multiple criteria
    #[derive(Debug)]
    enum Category { Small, Medium, Large }
    let values = [1u32, 15, 100, 8, 50, 3, 200];
    let mut small = Vec::new();
    let mut medium = Vec::new();
    let mut large = Vec::new();
    for &v in &values {
        match v {
            0..=10 => small.push(v),
            11..=100 => medium.push(v),
            _ => large.push(v),
        }
    }
    println!("Small: {:?}", small);
    println!("Medium: {:?}", medium);
    println!("Large: {:?}", large);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_partition_map_fold() {
        let data = vec!["1", "x", "2"];
        let (nums, _words): (Vec<i32>, Vec<&&str>) = data.iter().fold(
            (Vec::new(), Vec::new()),
            |(mut ns, mut ws), s| {
                match s.parse::<i32>() {
                    Ok(n) => ns.push(n),
                    Err(_) => ws.push(s),
                }
                (ns, ws)
            }
        );
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn test_trisect() {
        let data = [-1i32, 0, 1, -2, 0, 2];
        let (neg, non_neg): (Vec<i32>, Vec<i32>) = data.iter().copied().partition(|&x| x < 0);
        let (zero, pos): (Vec<i32>, Vec<i32>) = non_neg.into_iter().partition(|&x| x == 0);
        assert_eq!(neg.len(), 2);
        assert_eq!(zero.len(), 2);
        assert_eq!(pos.len(), 2);
    }

    #[test]
    fn test_nested_unzip() {
        let pairs: Vec<(i32, i32)> = vec![(1, 10), (2, 20), (3, 30)];
        let (a, b): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();
        assert_eq!(a, vec![1, 2, 3]);
        assert_eq!(b, vec![10, 20, 30]);
    }
}
