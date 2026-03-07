/// Filter removes elements that don't satisfy the predicate.
/// Idiomatic Rust: uses iterator chains like the Rust standard library
pub fn filter<T: Clone>(predicate: fn(&T) -> bool, items: &[T]) -> Vec<T> {
    items.iter().filter(|x| predicate(x)).cloned().collect()
}

/// Filter using immutable recursion — closer to OCaml style.
/// Shows the explicit pattern matching on the list structure.
pub fn filter_recursive<T: Clone>(predicate: fn(&T) -> bool, items: &[T]) -> Vec<T> {
    match items {
        [] => vec![],
        [h, rest @ ..] => {
            let mut tail = filter_recursive(predicate, rest);
            if predicate(h) {
                let mut result = vec![h.clone()];
                result.append(&mut tail);
                result
            } else {
                tail
            }
        }
    }
}

/// Filter using fold/reduce pattern — functional accumulation.
/// Demonstrates left fold over the slice.
pub fn filter_fold<T: Clone>(predicate: fn(&T) -> bool, items: &[T]) -> Vec<T> {
    items.iter().fold(vec![], |mut acc, x| {
        if predicate(x) {
            acc.push(x.clone());
        }
        acc
    })
}

// Predicates
pub fn is_even(x: &i32) -> bool {
    x % 2 == 0
}

pub fn is_odd(x: &i32) -> bool {
    x % 2 != 0
}

pub fn is_positive(x: &i32) -> bool {
    *x > 0
}

fn main() {
    let nums = vec![-2, -1, 0, 1, 2, 3, 4];

    print!("Evens: ");
    let evens = filter(is_even, &nums);
    for n in evens {
        print!("{} ", n);
    }
    println!();

    print!("Positives: ");
    let positives = filter(is_positive, &nums);
    for n in positives {
        print!("{} ", n);
    }
    println!();

    print!("Odds: ");
    let odds = filter(is_odd, &nums);
    for n in odds {
        print!("{} ", n);
    }
    println!();
}

/* Output:
   Evens: -2 0 2 4
   Positives: 1 2 3 4
   Odds: -1 1 3
*/
