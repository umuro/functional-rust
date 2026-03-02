// Higher-Order Functions in Rust
// Translation from OCaml: Implementing map, filter, and fold from scratch

// Map: Apply a function to each element
// Note: In production, use Iterator::map instead
fn map<T, U, F>(f: F, list: &[T]) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    match list {
        [] => vec![],
        [x, xs @ ..] => {
            let mut result = vec![f(x)];
            result.extend(map(f, xs));
            result
        }
    }
}

// Filter: Keep only elements that satisfy a predicate
// Note: In production, use Iterator::filter instead
fn filter<T, F>(pred: F, list: &[T]) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    match list {
        [] => vec![],
        [x, xs @ ..] => {
            let mut result = if pred(x) { vec![x.clone()] } else { vec![] };
            result.extend(filter(pred, xs));
            result
        }
    }
}

// Fold (reduce): Accumulate a value by applying a function
// Note: In production, use Iterator::fold instead
fn fold_left<T, U, F>(f: F, acc: U, list: &[T]) -> U
where
    F: Fn(U, &T) -> U + Copy,
{
    match list {
        [] => acc,
        [x, xs @ ..] => fold_left(f, f(acc, x), xs),
    }
}

fn main() {
    // Map: Double each number
    let doubled = map(|x| x * 2, &[1, 2, 3, 4, 5]);
    println!("Doubled: {:?}", doubled);

    // Filter: Keep only even numbers
    let evens = filter(|x| x % 2 == 0, &[1, 2, 3, 4, 5, 6]);
    println!("Evens: {:?}", evens);

    // Fold: Sum all numbers
    let sum = fold_left(|acc, x| acc + x, 0, &[1, 2, 3, 4, 5]);
    println!("Sum: {}", sum);

    // Composition: Double, then keep evens, then sum
    // Using method chaining (Rust idiom)
    let result = [1, 2, 3, 4, 5]
        .iter()
        .map(|x| x * 2)
        .filter(|x| x % 2 == 0)
        .sum::<i32>();
    println!("Composed (idiomatic): {}", result);

    // Same composition using our custom functions
    let numbers = [1, 2, 3, 4, 5];
    let doubled_custom = map(|x| x * 2, &numbers);
    let evens_custom = filter(|x| x % 2 == 0, &doubled_custom);
    let sum_custom = fold_left(|acc, x| acc + x, 0, &evens_custom);
    println!("Composed (custom functions): {}", sum_custom);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_doubles() {
        assert_eq!(map(|x| x * 2, &[1, 2, 3]), vec![2, 4, 6]);
    }

    #[test]
    fn test_map_squares() {
        assert_eq!(map(|x| x * x, &[1, 2, 3, 4]), vec![1, 4, 9, 16]);
    }

    #[test]
    fn test_filter_evens() {
        assert_eq!(filter(|x| x % 2 == 0, &[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_greater_than() {
        assert_eq!(filter(|x| *x > 3, &[1, 2, 3, 4, 5]), vec![4, 5]);
    }

    #[test]
    fn test_fold_sum() {
        assert_eq!(fold_left(|acc, x| acc + x, 0, &[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_fold_product() {
        assert_eq!(fold_left(|acc, x| acc * x, 1, &[1, 2, 3, 4]), 24);
    }

    #[test]
    fn test_composition() {
        let numbers = [1, 2, 3, 4, 5];
        let doubled = map(|x| x * 2, &numbers);
        let evens = filter(|x| x % 2 == 0, &doubled);
        let sum = fold_left(|acc, x| acc + x, 0, &evens);
        assert_eq!(sum, 30); // (2+4+6+8+10)
    }
}
