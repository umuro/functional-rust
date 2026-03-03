/// Safe head: `&[T]` → `Option<T>` (a natural transformation from List to Option).
///
/// Idiomatic Rust: delegate to `slice::first()` and clone the element.
pub fn safe_head<T: Clone>(list: &[T]) -> Option<T> {
    list.first().cloned()
}

/// Safe head — recursive, OCaml-style pattern matching.
pub fn safe_head_recursive<T: Clone>(list: &[T]) -> Option<T> {
    match list {
        [] => None,
        [x, ..] => Some(x.clone()),
    }
}

/// Safe last: `&[T]` → `Option<T>` (another natural transformation from List to Option).
pub fn safe_last<T: Clone>(list: &[T]) -> Option<T> {
    list.last().cloned()
}

/// `Option<T>` → `Vec<T>` (natural transformation: singleton list or empty list).
pub fn option_to_vec<T>(opt: Option<T>) -> Vec<T> {
    match opt {
        None => vec![],
        Some(x) => vec![x],
    }
}

/// Verify the naturality condition for a natural transformation `nat: &[T] → Option<T>`.
///
/// The naturality square commutes iff:
///   `nat_u(list.map(f)) == nat_t(list).map(f)`
pub fn verify_naturality<T, U>(
    f: impl Fn(T) -> U,
    nat_t: impl Fn(&[T]) -> Option<T>,
    nat_u: impl Fn(&[U]) -> Option<U>,
    list: &[T],
) -> bool
where
    T: Clone,
    U: PartialEq,
{
    let mapped: Vec<U> = list.iter().map(|x| f(x.clone())).collect();
    let lhs = nat_u(&mapped);
    let rhs = nat_t(list).map(f);
    lhs == rhs
}

/// Composed natural transformation: `&[T]` → `Vec<T>`
/// via `&[T]` -[safe_head]→ `Option<T>` -[option_to_vec]→ `Vec<T>`.
pub fn nat_composed<T: Clone>(list: &[T]) -> Vec<T> {
    option_to_vec(safe_head(list))
}

fn main() {
    // Basic natural transformations
    println!("safe_head([1,2,3])    = {:?}", safe_head(&[1, 2, 3]));
    println!("safe_head([])         = {:?}", safe_head::<i32>(&[]));
    println!("safe_head_recursive([1,2,3]) = {:?}", safe_head_recursive(&[1, 2, 3]));
    println!("safe_last([1,2,3])    = {:?}", safe_last(&[1, 2, 3]));
    println!("safe_last([])         = {:?}", safe_last::<i32>(&[]));

    println!();

    // option_to_vec: Option → Vec
    println!("option_to_vec(Some(42)) = {:?}", option_to_vec(Some(42)));
    println!("option_to_vec(None)     = {:?}", option_to_vec::<i32>(None));

    println!();

    // Verify naturality: safe_head is a natural transformation w.r.t. i32 -> String
    let lists: &[&[i32]] = &[&[], &[1], &[1, 2, 3], &[42, 0, -1]];
    let all_natural = lists
        .iter()
        .all(|lst| verify_naturality(|x: i32| x.to_string(), safe_head, safe_head, lst));
    println!("safe_head is natural (int->string)? {all_natural}");

    println!();

    // Composition of natural transformations
    println!(
        "nat_composed([1,2,3]) = {:?}",
        nat_composed(&[1, 2, 3])
    );
    println!("nat_composed([])      = {:?}", nat_composed::<i32>(&[]));
}

/* Output:
   safe_head([1,2,3])    = Some(1)
   safe_head([])         = None
   safe_head_recursive([1,2,3]) = Some(1)
   safe_last([1,2,3])    = Some(3)
   safe_last([])         = None

   option_to_vec(Some(42)) = [42]
   option_to_vec(None)     = []

   safe_head is natural (int->string)? true

   nat_composed([1,2,3]) = [1]
   nat_composed([])      = []
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_head_empty() {
        assert_eq!(safe_head::<i32>(&[]), None);
    }

    #[test]
    fn test_safe_head_single() {
        assert_eq!(safe_head(&[42]), Some(42));
    }

    #[test]
    fn test_safe_head_multiple() {
        assert_eq!(safe_head(&[1, 2, 3]), Some(1));
    }

    #[test]
    fn test_safe_head_recursive_agrees_with_idiomatic() {
        let cases: &[&[i32]] = &[&[], &[7], &[1, 2, 3], &[42, 0, -1]];
        for list in cases {
            assert_eq!(safe_head(list), safe_head_recursive(list));
        }
    }

    #[test]
    fn test_safe_last_empty() {
        assert_eq!(safe_last::<i32>(&[]), None);
    }

    #[test]
    fn test_safe_last_single() {
        assert_eq!(safe_last(&[99]), Some(99));
    }

    #[test]
    fn test_safe_last_multiple() {
        assert_eq!(safe_last(&[1, 2, 3]), Some(3));
    }

    #[test]
    fn test_option_to_vec_none() {
        assert_eq!(option_to_vec::<i32>(None), vec![]);
    }

    #[test]
    fn test_option_to_vec_some() {
        assert_eq!(option_to_vec(Some(5)), vec![5]);
    }

    #[test]
    fn test_naturality_safe_head_int_to_string() {
        let cases: &[&[i32]] = &[&[], &[1], &[1, 2, 3], &[42, 0, -1]];
        for list in cases {
            assert!(
                verify_naturality(|x: i32| x.to_string(), safe_head, safe_head, list),
                "naturality failed for {list:?}"
            );
        }
    }

    #[test]
    fn test_naturality_safe_last_int_to_int() {
        let cases: &[&[i32]] = &[&[], &[1], &[1, 2, 3], &[42, 0, -1]];
        for list in cases {
            assert!(
                verify_naturality(|x: i32| x * 2, safe_last, safe_last, list),
                "naturality failed for {list:?}"
            );
        }
    }

    #[test]
    fn test_nat_composed_nonempty() {
        assert_eq!(nat_composed(&[1, 2, 3]), vec![1]);
    }

    #[test]
    fn test_nat_composed_empty() {
        assert_eq!(nat_composed::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_nat_composed_single() {
        assert_eq!(nat_composed(&[42]), vec![42]);
    }
}
