//! Standalone display copy of example 1217 — `List.fold_left` in Rust.

pub fn fold_left<T, A, F>(f: F, init: A, items: &[T]) -> A
where
    F: Fn(A, &T) -> A,
{
    items.iter().fold(init, f)
}

pub fn fold_left_recursive<T, A, F>(f: &F, acc: A, items: &[T]) -> A
where
    F: Fn(A, &T) -> A,
{
    match items {
        [] => acc,
        [h, rest @ ..] => fold_left_recursive(f, f(acc, h), rest),
    }
}

pub fn fold_left_loop<T, A, F>(f: F, init: A, items: &[T]) -> A
where
    F: Fn(A, &T) -> A,
{
    let mut acc = init;
    for x in items {
        acc = f(acc, x);
    }
    acc
}

pub fn sum(numbers: &[i32]) -> i32 {
    fold_left(|acc, x| acc + x, 0, numbers)
}

pub fn product(numbers: &[i32]) -> i32 {
    fold_left(|acc, x| acc * x, 1, numbers)
}

pub fn concat_labelled(label: &str, numbers: &[i32]) -> String {
    fold_left(
        |acc, x| acc + " " + &x.to_string(),
        label.to_string(),
        numbers,
    )
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    let s = sum(&numbers);
    let p = product(&numbers);
    let c = concat_labelled("Numbers:", &numbers);

    println!("Sum: {}", s);
    println!("Product: {}", p);
    println!("{}", c);

    // subtraction makes the left-to-right order observable
    let minus = fold_left(|acc, x| acc - x, 0, &numbers);
    println!("fold_left (-) 0 [1;2;3;4;5] = {}", minus);

    // recursive variant — direct OCaml parallel
    let s_rec = fold_left_recursive(&|acc, x| acc + x, 0, &numbers);
    println!("recursive sum = {}", s_rec);
}

/* Output:
   Sum: 15
   Product: 120
   Numbers: 1 2 3 4 5
   fold_left (-) 0 [1;2;3;4;5] = -15
   recursive sum = 15
*/
