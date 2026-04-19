//! Standalone display copy of example 1216 — `List.filter` in Rust.

pub fn filter<T, F>(predicate: F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().copied().filter(|x| predicate(x)).collect()
}

pub fn filter_recursive<T, F>(predicate: &F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    match items {
        [] => Vec::new(),
        [h, rest @ ..] => {
            let mut tail = filter_recursive(predicate, rest);
            if predicate(h) {
                let mut out = Vec::with_capacity(1 + tail.len());
                out.push(*h);
                out.append(&mut tail);
                out
            } else {
                tail
            }
        }
    }
}

pub fn filter_fold<T, F>(predicate: F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().fold(Vec::new(), |mut acc, x| {
        if predicate(x) {
            acc.push(*x);
        }
        acc
    })
}

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
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];

    let evens = filter(is_even, &numbers);
    let odds = filter(is_odd, &numbers);
    let pos = filter(is_positive, &numbers);

    println!("evens = {:?}", evens);
    println!("odds  = {:?}", odds);
    println!("pos   = {:?}", pos);

    // recursive variant mirrors OCaml pattern matching
    let evens_rec = filter_recursive(&is_even, &numbers);
    println!("evens_rec = {:?}", evens_rec);

    // closure predicate — inline anonymous fn like OCaml `fun x -> x > 5`
    let big = filter(|x: &i32| *x > 5, &numbers);
    println!("big (>5) = {:?}", big);
}

/* Output:
   evens = [2, 4, 6, 8]
   odds  = [1, 3, 5, 7]
   pos   = [1, 2, 3, 4, 5, 6, 7, 8]
   evens_rec = [2, 4, 6, 8]
   big (>5) = [6, 7, 8]
*/
