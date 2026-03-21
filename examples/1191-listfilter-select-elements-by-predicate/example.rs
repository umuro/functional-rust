pub fn filter_by<T, F>(items: &[T], predicate: F) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| predicate(x)).copied().collect()
}

pub fn filter_recursive<T, F>(list: &[T], predicate: &F) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    match list {
        [] => vec![],
        [head, tail @ ..] => {
            let mut rest = filter_recursive(tail, predicate);
            if predicate(head) {
                rest.insert(0, *head);
            }
            rest
        }
    }
}

pub fn partition_by<T, F>(items: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().partition(|x| predicate(x))
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];

    let evens = filter_by(&numbers, |x| x % 2 == 0);
    let odds = filter_by(&numbers, |x| x % 2 != 0);
    println!("Evens: {:?}", evens);
    println!("Odds:  {:?}", odds);

    let evens_rec = filter_recursive(&numbers, &|x| x % 2 == 0);
    println!("Evens (recursive): {:?}", evens_rec);

    let (evens_p, odds_p) = partition_by(&numbers, |x| x % 2 == 0);
    println!("Partition evens: {:?}", evens_p);
    println!("Partition odds:  {:?}", odds_p);
}

/* Output:
   Evens: [2, 4, 6, 8]
   Odds:  [1, 3, 5, 7]
   Evens (recursive): [2, 4, 6, 8]
   Partition evens: [2, 4, 6, 8]
   Partition odds:  [1, 3, 5, 7]
*/
