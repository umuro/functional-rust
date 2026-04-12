/// Partition a slice into two Vecs based on a predicate.
pub fn partition_idiomatic<T, F>(items: &[T], pred: F) -> (Vec<&T>, Vec<&T>)
where
    F: Fn(&T) -> bool,
{
    items.iter().partition(|x| pred(x))
}

pub fn partition_fold<T, F>(items: &[T], pred: F) -> (Vec<&T>, Vec<&T>)
where
    F: Fn(&T) -> bool,
{
    items.iter().fold((vec![], vec![]), |(mut yes, mut no), x| {
        if pred(x) {
            yes.push(x);
        } else {
            no.push(x);
        }
        (yes, no)
    })
}

pub fn partition_recursive<'a, T, F>(items: &'a [T], pred: &F) -> (Vec<&'a T>, Vec<&'a T>)
where
    F: Fn(&T) -> bool,
{
    match items {
        [] => (vec![], vec![]),
        [head, rest @ ..] => {
            let (mut yes, mut no) = partition_recursive(rest, pred);
            if pred(head) {
                yes.insert(0, head);
            } else {
                no.insert(0, head);
            }
            (yes, no)
        }
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10_i32];

    let (small, big) = partition_idiomatic(&numbers, |x| *x <= 5);
    println!(
        "Small: {}",
        small
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "Big: {}",
        big.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let (evens, odds) = partition_fold(&numbers, |x| *x % 2 == 0);
    println!(
        "Evens (fold): {}",
        evens
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "Odds (fold): {}",
        odds.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let pred = |x: &i32| *x <= 5;
    let (small_r, big_r) = partition_recursive(&numbers, &pred);
    println!(
        "Small (recursive): {}",
        small_r
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "Big (recursive): {}",
        big_r
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

/* Output:
   Small: 1 2 3 4 5
   Big: 6 7 8 9 10
   Evens (fold): 2 4 6 8 10
   Odds (fold): 1 3 5 7 9
   Small (recursive): 1 2 3 4 5
   Big (recursive): 6 7 8 9 10
*/
