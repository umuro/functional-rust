#![allow(dead_code)]

use std::collections::HashMap;

/// Generic left fold — mirrors OCaml's `List.fold_left f acc xs`.
pub fn fold_left<T, U, F>(items: &[T], init: U, f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    items.iter().fold(init, f)
}

/// Sum using the specialized `.sum()` adapter.
pub fn sum_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().copied().sum()
}

/// Product using the specialized `.product()` adapter.
pub fn product_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().copied().product()
}

/// Sum via an explicit fold — mirrors OCaml's `List.fold_left (+) 0 xs`.
pub fn sum_fold(numbers: &[i64]) -> i64 {
    fold_left(numbers, 0, |acc, &x| acc + x)
}

/// Maximum value using `reduce`.
pub fn max_val(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().reduce(i64::max)
}

fn main() {
    let numbers: Vec<i64> = (1..=5).collect();
    println!("numbers = {:?}", numbers);
    println!("sum     = {}", sum_idiomatic(&numbers));
    println!("product = {}", product_idiomatic(&numbers));
    println!("max     = {:?}", max_val(&numbers));
    println!("sum (fold) = {}", sum_fold(&numbers));

    // Build a frequency map via fold
    let words = ["apple", "banana", "apple", "cherry", "banana", "apple"];
    let freq: HashMap<&&str, usize> = fold_left(&words, HashMap::new(), |mut map, w| {
        *map.entry(w).or_insert(0) += 1;
        map
    });
    println!("freq: apple={:?}", freq.get(&"apple"));
}

/* Output:
   numbers = [1, 2, 3, 4, 5]
   sum     = 15
   product = 120
   max     = Some(5)
   sum (fold) = 15
   freq: apple=Some(3)
*/
