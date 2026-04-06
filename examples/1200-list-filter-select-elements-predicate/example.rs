#![allow(dead_code)]

/// Idiomatic Rust: filter a slice using a predicate, returning borrowed refs.
pub fn filter<T, F>(list: &[T], predicate: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    list.iter().filter(|x| predicate(x)).collect()
}

/// Filter and clone elements (owned output), mirroring OCaml's List.filter.
pub fn filter_cloned<T: Clone, F>(list: &[T], predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    list.iter().filter(|x| predicate(x)).cloned().collect()
}

/// Functional/recursive — mirrors the OCaml recursive pattern explicitly.
pub fn filter_recursive<T: Clone, F>(list: &[T], predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    fn go<T: Clone>(list: &[T], predicate: &dyn Fn(&T) -> bool) -> Vec<T> {
        match list {
            [] => vec![],
            [head, tail @ ..] => {
                let mut rest = go(tail, predicate);
                if predicate(head) {
                    let mut result = vec![head.clone()];
                    result.append(&mut rest);
                    result
                } else {
                    rest
                }
            }
        }
    }
    go(list, &predicate)
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];

    let evens = filter_cloned(&numbers, |x| x % 2 == 0);
    let odds = filter_cloned(&numbers, |x| x % 2 != 0);

    println!(
        "Evens: {}",
        evens
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "Odds: {}",
        odds.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Recursive style
    let evens_rec = filter_recursive(&numbers, |x| x % 2 == 0);
    println!("Evens (recursive): {:?}", evens_rec);

    // Borrowed refs style
    let long_words = ["apple", "banana", "cherry", "date"];
    let long: Vec<&&str> = filter(&long_words, |w| w.len() > 5);
    println!("Words longer than 5 chars: {:?}", long);
}

/* Output:
   Evens: 2, 4, 6, 8
   Odds: 1, 3, 5, 7
   Evens (recursive): [2, 4, 6, 8]
   Words longer than 5 chars: ["banana", "cherry"]
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_empty() {
        let empty: &[i32] = &[];
        assert_eq!(filter(empty, |x| x % 2 == 0), Vec::<&i32>::new());
    }

    #[test]
    fn test_filter_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let evens: Vec<&i32> = filter(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, [&2, &4, &6, &8]);
    }

    #[test]
    fn test_filter_odds() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let odds: Vec<&i32> = filter(&numbers, |x| x % 2 != 0);
        assert_eq!(odds, [&1, &3, &5, &7]);
    }

    #[test]
    fn test_filter_recursive_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let evens = filter_recursive(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, [2, 4, 6, 8]);
    }
}
