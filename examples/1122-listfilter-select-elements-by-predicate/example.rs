pub fn filter_idiomatic<T, F>(list: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    list.iter().filter(|x| predicate(x)).cloned().collect()
}

pub fn partition_by<T, F>(list: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    let (yes, no): (Vec<&T>, Vec<&T>) = list.iter().partition(|x| predicate(x));
    (
        yes.into_iter().cloned().collect(),
        no.into_iter().cloned().collect(),
    )
}

pub fn filter_recursive<T, F>(list: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    fn go<T, F>(list: &[T], pred: &F) -> Vec<T>
    where
        T: Clone,
        F: Fn(&T) -> bool,
    {
        match list {
            [] => vec![],
            [head, tail @ ..] => {
                let mut rest = go(tail, pred);
                if pred(head) {
                    rest.insert(0, head.clone());
                }
                rest
            }
        }
    }
    go(list, &predicate)
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];

    // Idiomatic: filter with a predicate closure
    let evens = filter_idiomatic(&numbers, |x| x % 2 == 0);
    let odds = filter_idiomatic(&numbers, |x| x % 2 != 0);
    println!("Evens: {:?}", evens);
    println!("Odds:  {:?}", odds);

    // Partition: both halves in a single pass
    let (evens2, odds2) = partition_by(&numbers, |x| x % 2 == 0);
    println!("Partition evens: {:?}", evens2);
    println!("Partition odds:  {:?}", odds2);

    // Recursive: OCaml-style structural recursion
    let gt4 = filter_recursive(&numbers, |x| *x > 4);
    println!("Greater than 4: {:?}", gt4);
}

/* Output:
   Evens: [2, 4, 6, 8]
   Odds:  [1, 3, 5, 7]
   Partition evens: [2, 4, 6, 8]
   Partition odds:  [1, 3, 5, 7]
   Greater than 4: [5, 6, 7, 8]
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_empty() {
        let empty: &[i32] = &[];
        assert_eq!(filter_idiomatic(empty, |x| x % 2 == 0), Vec::<i32>::new());
    }

    #[test]
    fn test_filter_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(
            filter_idiomatic(&numbers, |x| x % 2 == 0),
            vec![2, 4, 6, 8]
        );
    }

    #[test]
    fn test_filter_odds() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(
            filter_idiomatic(&numbers, |x| x % 2 != 0),
            vec![1, 3, 5, 7]
        );
    }

    #[test]
    fn test_partition_evens_and_odds() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let (evens, odds) = partition_by(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6, 8]);
        assert_eq!(odds, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_recursive_matches_idiomatic() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(
            filter_recursive(&numbers, |x| x % 2 == 0),
            filter_idiomatic(&numbers, |x| x % 2 == 0)
        );
    }
}
