//! # 520. Higher-Order Functions
//! Rust's iterator HOFs: map, filter, fold, flat_map, zip, and custom ones.

/// Custom HOF: zip two slices with a combining function
fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where
    F: Fn(&A, &B) -> C,
{
    a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()
}

/// Custom HOF: scan (like fold but returns all intermediate values)
fn scan_left<T: Clone, U: Clone, F>(items: &[T], init: U, f: F) -> Vec<U>
where
    F: Fn(U, &T) -> U,
{
    let mut acc = init;
    let mut result = vec![acc.clone()];
    for item in items {
        acc = f(acc, item);
        result.push(acc.clone());
    }
    result
}

/// Custom HOF: group consecutive elements by a key
fn group_by<T, K, F>(items: Vec<T>, key: F) -> Vec<(K, Vec<T>)>
where
    K: PartialEq,
    F: Fn(&T) -> K,
{
    let mut groups: Vec<(K, Vec<T>)> = Vec::new();
    for item in items {
        let k = key(&item);
        if let Some(last) = groups.last_mut() {
            if last.0 == k {
                last.1.push(item);
                continue;
            }
        }
        groups.push((k, vec![item]));
    }
    groups
}

fn main() {
    let nums: Vec<i32> = (1..=10).collect();

    // map
    let squares: Vec<i32> = nums.iter().map(|&x| x * x).collect();
    println!("squares: {:?}", squares);

    // filter
    let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("evens: {:?}", evens);

    // fold
    let sum: i32 = nums.iter().fold(0, |acc, &x| acc + x);
    println!("sum: {}", sum);

    // chained pipeline (lazy — no intermediate allocations)
    let sum_even_squares: i32 = nums.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    println!("sum of even squares: {}", sum_even_squares);

    // flat_map
    let expanded: Vec<i32> = [1, 2, 3].iter()
        .flat_map(|&x| vec![x, x * 10])
        .collect();
    println!("flat_map: {:?}", expanded);

    // zip
    let a = [1, 2, 3];
    let b = [10, 20, 30];
    let sums = zip_with(&a, &b, |x, y| x + y);
    println!("zip_with(+): {:?}", sums);

    // any / all
    println!("any > 5: {}", nums.iter().any(|&x| x > 5));
    println!("all > 0: {}", nums.iter().all(|&x| x > 0));

    // take_while / skip_while
    let ascending: Vec<i32> = nums.iter().copied().take_while(|&x| x <= 5).collect();
    println!("take_while <=5: {:?}", ascending);

    // scan (running totals)
    let running = scan_left(&nums[..5], 0, |acc, &x| acc + x);
    println!("running totals: {:?}", running);

    // group_by
    let words = vec!["ant", "ape", "bear", "bee", "cat"];
    let grouped = group_by(words, |w| w.chars().next().unwrap());
    for (letter, group) in &grouped {
        println!("  '{}': {:?}", letter, group);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_filter_fold() {
        let v = vec![1, 2, 3, 4, 5];
        let result: i32 = v.iter()
            .filter(|&&x| x % 2 != 0)
            .map(|&x| x * x)
            .sum();
        assert_eq!(result, 1 + 9 + 25); // 35
    }

    #[test]
    fn test_zip_with() {
        let a = [1, 2, 3];
        let b = [10, 20, 30];
        assert_eq!(zip_with(&a, &b, |x, y| x * y), vec![10, 40, 90]);
    }

    #[test]
    fn test_scan_left() {
        let v = [1, 2, 3, 4];
        let running = scan_left(&v, 0, |acc, &x| acc + x);
        assert_eq!(running, vec![0, 1, 3, 6, 10]);
    }

    #[test]
    fn test_flat_map() {
        let v: Vec<i32> = [1, 2, 3].iter().flat_map(|&x| [x, -x]).collect();
        assert_eq!(v, [1, -1, 2, -2, 3, -3]);
    }
}
