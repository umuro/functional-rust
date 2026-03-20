#![allow(clippy::all)]
// 103: Shared Borrowing — &T
// Multiple readers, no writers

fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn count(data: &[i32]) -> usize {
    data.len()
}

fn average(data: &[i32]) -> f64 {
    // Multiple shared borrows simultaneously — perfectly safe
    let s = sum(data); // &data borrow 1
    let c = count(data); // &data borrow 2 — fine!
    s as f64 / c as f64
}

fn first_and_last(data: &[i32]) -> Option<(i32, i32)> {
    if data.is_empty() {
        None
    } else {
        Some((data[0], data[data.len() - 1]))
    }
}

// Multiple shared references can coexist
fn demonstrate_multiple_borrows() {
    let data = vec![1, 2, 3, 4, 5];
    let r1 = &data;
    let r2 = &data;
    let r3 = &data;
    // All three references valid simultaneously
    println!("r1={:?}, r2={:?}, r3={:?}", r1[0], r2[1], r3[2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert!((average(&[1, 2, 3, 4, 5]) - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_first_and_last() {
        assert_eq!(first_and_last(&[10, 20, 30]), Some((10, 30)));
        assert_eq!(first_and_last(&[]), None);
    }

    #[test]
    fn test_multiple_borrows() {
        let v = vec![1, 2, 3];
        let r1 = &v;
        let r2 = &v;
        assert_eq!(r1.len(), r2.len());
    }
}
