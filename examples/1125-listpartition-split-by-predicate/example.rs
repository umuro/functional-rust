#![allow(dead_code)]

/// Split a slice into two vecs: elements satisfying `pred` and those that don't.
pub fn partition<T: Clone, F>(items: &[T], pred: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    items.iter().cloned().partition(|x| pred(x))
}

/// Partition integers into (small, big) where small means <= threshold.
pub fn partition_threshold(numbers: &[i32], threshold: i32) -> (Vec<i32>, Vec<i32>) {
    partition(numbers, |&x| x <= threshold)
}

/// Partition strings by length.
pub fn partition_by_length<'a>(words: &[&'a str], max_len: usize) -> (Vec<&'a str>, Vec<&'a str>) {
    partition(words, |s| s.len() <= max_len)
}

fn main() {
    let numbers: Vec<i32> = (1..=10).collect();
    let (small, big) = partition_threshold(&numbers, 5);
    println!("numbers = {:?}", numbers);
    println!("small (<=5) = {:?}", small);
    println!("big   (>5)  = {:?}", big);

    let (evens, odds) = partition(&numbers, |&x| x % 2 == 0);
    println!("evens = {:?}", evens);
    println!("odds  = {:?}", odds);

    let words = ["hi", "hello", "ok", "world", "rust"];
    let (short, long) = partition_by_length(&words, 3);
    println!("short (<=3): {:?}", short);
    println!("long  (>3) : {:?}", long);
}

/* Output:
   numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   small (<=5) = [1, 2, 3, 4, 5]
   big   (>5)  = [6, 7, 8, 9, 10]
   evens = [2, 4, 6, 8, 10]
   odds  = [1, 3, 5, 7, 9]
   short (<=3): ["hi", "ok"]
   long  (>3) : ["hello", "world", "rust"]
*/
