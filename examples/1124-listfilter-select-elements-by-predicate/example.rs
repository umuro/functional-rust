#![allow(dead_code)]

/// Idiomatic Rust: filter a slice using an iterator adapter.
pub fn filter<T: Clone, F>(items: &[T], pred: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| pred(x)).cloned().collect()
}

/// Functional/recursive: mirrors OCaml pattern matching on the list spine.
pub fn filter_recursive<T: Clone, F>(items: &[T], pred: &F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    match items {
        [] => vec![],
        [head, rest @ ..] => {
            let mut tail = filter_recursive(rest, pred);
            if pred(head) {
                tail.insert(0, head.clone());
            }
            tail
        }
    }
}

/// Keep only even integers.
pub fn filter_evens(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

/// Keep only odd integers.
pub fn filter_odds(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&x| x % 2 != 0).copied().collect()
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8_i32];
    println!("numbers = {:?}", numbers);
    println!("evens   = {:?}", filter_evens(&numbers));
    println!("odds    = {:?}", filter_odds(&numbers));
    println!("> 4     = {:?}", filter(&numbers, |&x| x > 4));

    let words = ["hi", "hello", "hey", "salutation", "ok"];
    let long: Vec<&str> = filter(&words, |s| s.len() > 3);
    println!("long words (>3): {:?}", long);
}

/* Output:
   numbers = [1, 2, 3, 4, 5, 6, 7, 8]
   evens   = [2, 4, 6, 8]
   odds    = [1, 3, 5, 7]
   > 4     = [5, 6, 7, 8]
   long words (>3): ["hello", "salutation"]
*/
