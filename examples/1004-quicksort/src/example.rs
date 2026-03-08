/// Recursive quicksort matching the OCaml functional style.
pub fn quicksort<T: Clone + Ord, F: Fn(&T, &T) -> bool + Copy>(gt: F, mut xs: Vec<T>) -> Vec<T> {
    if xs.is_empty() {
        return xs;
    }

    let pivot = xs.remove(0);
    let (ys, zs): (Vec<T>, Vec<T>) = xs.into_iter().partition(|x| gt(&pivot, x));

    let mut left = quicksort(gt, ys);
    let mut result = quicksort(gt, zs);
    left.push(pivot);
    left.append(&mut result);
    left
}

/// Idiomatic Rust quicksort using the standard library's `sort`.
pub fn quicksort_idiomatic<T: Ord>(xs: Vec<T>) -> Vec<T> {
    let mut result = xs;
    result.sort();
    result
}

/// Idiomatic Rust quicksort with custom comparator.
pub fn quicksort_idiomatic_by<T, F: Fn(&T, &T) -> std::cmp::Ordering>(
    mut xs: Vec<T>,
    f: F,
) -> Vec<T> {
    xs.sort_by(f);
    xs
}

fn main() {
    // Example 1: Functional recursive quicksort (mimics OCaml)
    let input = vec![4, 65, 2, -31, 0, 99, 83, 782, 1];
    let sorted = quicksort(|a, b| a > b, input.clone());
    println!("Recursive quicksort (ascending):");
    println!("  Input:  {:?}", input);
    println!("  Output: {:?}", sorted);
    println!();

    // Example 2: Idiomatic Rust with std::sort
    let input2 = vec![4, 65, 2, -31, 0, 99, 83, 782, 1];
    let sorted2 = quicksort_idiomatic(input2.clone());
    println!("Idiomatic quicksort (ascending):");
    println!("  Input:  {:?}", input2);
    println!("  Output: {:?}", sorted2);
    println!();

    // Example 3: Descending order with custom comparator
    let input3 = vec![4, 65, 2, -31, 0, 99, 83, 782, 1];
    let sorted3 = quicksort(|a, b| a < b, input3.clone());
    println!("Recursive quicksort (descending):");
    println!("  Input:  {:?}", input3);
    println!("  Output: {:?}", sorted3);
    println!();

    // Example 4: Idiomatic with custom comparator (descending)
    let input4 = vec![4, 65, 2, -31, 0, 99, 83, 782, 1];
    let sorted4 = quicksort_idiomatic_by(input4.clone(), |a, b| b.cmp(a));
    println!("Idiomatic quicksort with comparator (descending):");
    println!("  Input:  {:?}", input4);
    println!("  Output: {:?}", sorted4);
    println!();

    // Example 5: Edge cases
    println!("Edge cases:");
    println!("  Empty: {:?}", quicksort(|a, b| a > b, vec![] as Vec<i32>));
    println!("  Single: {:?}", quicksort(|a, b| a > b, vec![42]));
    println!(
        "  Duplicates: {:?}",
        quicksort(|a, b| a > b, vec![3, 1, 3, 2, 1, 3])
    );
}
