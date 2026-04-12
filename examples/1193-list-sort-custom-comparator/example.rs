/// List.sort — Sort with Custom Comparator
/// Lexicographic, length-based, descending, and merge sort implementations.

use std::cmp::Ordering;

pub fn sort_strings<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort();
    result
}

pub fn sort_by_length<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    result
}

pub fn sort_descending<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort_by(|a, b| b.cmp(a));
    result
}

pub fn sort_with<T: Clone, F>(items: &[T], cmp: F) -> Vec<T>
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut result = items.to_vec();
    result.sort_by(|a, b| cmp(a, b));
    result
}

fn main() {
    let words = ["banana", "apple", "cherry", "date"];
    println!("original:    {:?}", words);
    println!("sorted:      {:?}", sort_strings(&words));
    println!("by length:   {:?}", sort_by_length(&words));
    println!("descending:  {:?}", sort_descending(&words));

    let numbers = [5, 1, 4, 2, 3];
    println!("sort numbers: {:?}", sort_with(&numbers, |a, b| a.cmp(b)));
}

/* Output:
   original:    ["banana", "apple", "cherry", "date"]
   sorted:      ["apple", "banana", "cherry", "date"]
   by length:   ["date", "apple", "banana", "cherry"]
   descending:  ["date", "cherry", "banana", "apple"]
   sort numbers: [1, 2, 3, 4, 5]
*/
