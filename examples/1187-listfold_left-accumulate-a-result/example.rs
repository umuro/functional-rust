// Solution 1: Idiomatic Rust — using .sum(), .product(), .max() from std
pub fn sum_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().sum()
}

pub fn product_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().product()
}

pub fn max_idiomatic(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().max()
}

// Solution 2: Functional — generic fold_left mirroring OCaml's List.fold_left
// fold_left f acc [a; b; c] = f (f (f acc a) b) c
pub fn fold_left<T, Acc, F>(f: F, init: Acc, list: &[T]) -> Acc
where
    F: Fn(Acc, &T) -> Acc,
{
    list.iter().fold(init, f)
}

// Solution 3: Recursive fold — explicit recursion as in OCaml
pub fn fold_left_rec<T, Acc, F>(f: &F, init: Acc, list: &[T]) -> Acc
where
    F: Fn(Acc, &T) -> Acc,
{
    match list {
        [] => init,
        [head, tail @ ..] => fold_left_rec(f, f(init, head), tail),
    }
}

fn main() {
    let numbers: &[i64] = &[1, 2, 3, 4, 5];

    // Idiomatic std approach
    let sum = sum_idiomatic(numbers);
    let product = product_idiomatic(numbers);
    let max = max_idiomatic(numbers);
    println!("Idiomatic — Sum: {sum}, Product: {product}, Max: {max:?}");

    // fold_left approach (mirrors OCaml)
    let sum2 = fold_left(|acc, &x| acc + x, 0, numbers);
    let product2 = fold_left(|acc, &x| acc * x, 1, numbers);
    println!("fold_left  — Sum: {sum2}, Product: {product2}");

    // Recursive fold
    let sum3 = fold_left_rec(&|acc, &x| acc + x, 0, numbers);
    println!("Recursive  — Sum: {sum3}");
}

/* Output:
   Idiomatic — Sum: 15, Product: 120, Max: Some(5)
   fold_left  — Sum: 15, Product: 120
   Recursive  — Sum: 15
*/
