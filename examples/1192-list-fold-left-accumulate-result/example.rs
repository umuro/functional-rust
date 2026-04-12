/// List.fold_left — Accumulate a Result
/// Generic fold and concrete sum/product/max helpers.

pub fn fold_left<T, U, F>(items: &[T], init: U, f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    items.iter().fold(init, f)
}

pub fn sum(numbers: &[i64]) -> i64 {
    fold_left(numbers, 0, |acc, &x| acc + x)
}

pub fn product(numbers: &[i64]) -> i64 {
    fold_left(numbers, 1, |acc, &x| acc * x)
}

pub fn max_val(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().reduce(|a, b| a.max(b))
}

pub fn fold_left_recursive<T, U, F>(items: &[T], acc: U, f: &F) -> U
where
    F: Fn(U, &T) -> U,
{
    match items {
        [] => acc,
        [head, rest @ ..] => {
            let new_acc = f(acc, head);
            fold_left_recursive(rest, new_acc, f)
        }
    }
}

fn main() {
    let numbers = [1_i64, 2, 3, 4, 5];
    println!("sum([1..5])     = {}", sum(&numbers));
    println!("product([1..5]) = {}", product(&numbers));
    println!("max([1..5])     = {:?}", max_val(&numbers));
    println!("max([])         = {:?}", max_val(&[]));

    let words = ["hello", " ", "world"];
    let joined = fold_left(&words, String::new(), |acc, s| acc + s);
    println!("joined words    = {:?}", joined);
}

/* Output:
   sum([1..5])     = 15
   product([1..5]) = 120
   max([1..5])     = Some(5)
   max([])         = None
   joined words    = "hello world"
*/
