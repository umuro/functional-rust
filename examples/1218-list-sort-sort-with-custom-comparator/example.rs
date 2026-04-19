//! Standalone display copy of example 1218 — `List.sort` with a custom
//! comparator in Rust.

use std::cmp::Ordering;

pub fn sort_by_comparator<T, F>(items: &[T], cmp: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    let mut out = items.to_vec();
    out.sort_by(cmp);
    out
}

pub fn sort_by_key_fn<T, K, F>(items: &[T], key: F) -> Vec<T>
where
    T: Clone,
    K: Ord,
    F: FnMut(&T) -> K,
{
    let mut out = items.to_vec();
    out.sort_by_key(key);
    out
}

pub fn sort_alphabetical(words: &[String]) -> Vec<String> {
    sort_by_comparator(words, |a, b| a.cmp(b))
}

pub fn sort_by_length(words: &[String]) -> Vec<String> {
    sort_by_key_fn(words, |s| s.len())
}

pub fn sort_descending<T, F>(items: &[T], mut cmp: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    sort_by_comparator(items, move |a, b| cmp(a, b).reverse())
}

fn main() {
    let words: Vec<String> = ["banana", "apple", "cherry", "date"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();

    let by_alphabet = sort_alphabetical(&words);
    let by_length = sort_by_length(&words);
    let by_alpha_desc = sort_descending(&words, |a, b| a.cmp(b));

    println!("Alphabetical: {}", by_alphabet.join(" "));
    println!("By length:    {}", by_length.join(" "));
    println!("Descending:   {}", by_alpha_desc.join(" "));
}

/* Output:
   Alphabetical: apple banana cherry date
   By length:    date apple banana cherry
   Descending:   date cherry banana apple
*/
