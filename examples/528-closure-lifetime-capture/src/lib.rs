//! Closures Capturing References
//!
//! How closure lifetimes are constrained by captured borrows.

/// Closure captures &str — lifetime tied to the string's scope.
pub fn make_prefix_checker<'a>(prefix: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |s| s.starts_with(prefix)
}

/// Multiple borrows in a closure.
pub fn make_range_checker<'a>(data: &'a [i32]) -> impl Fn(i32) -> bool + 'a {
    move |target| data.contains(&target)
}

/// Struct holding a closure that borrows.
pub struct Filter<'a, T> {
    data: &'a [T],
    predicate: Box<dyn Fn(&T) -> bool + 'a>,
}

impl<'a, T> Filter<'a, T> {
    pub fn new(data: &'a [T], predicate: impl Fn(&T) -> bool + 'a) -> Self {
        Filter {
            data,
            predicate: Box::new(predicate),
        }
    }

    pub fn apply(&self) -> Vec<&T> {
        self.data.iter().filter(|x| (self.predicate)(x)).collect()
    }
}

/// Closure borrowing multiple fields from a struct.
pub fn make_validator<'a>(min: &'a i32, max: &'a i32) -> impl Fn(i32) -> bool + 'a {
    move |x| x >= *min && x <= *max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_checker() {
        let prefix = String::from("hello");
        let checker = make_prefix_checker(&prefix);
        assert!(checker("hello world"));
        assert!(!checker("hi there"));
    }

    #[test]
    fn test_range_checker() {
        let data = vec![1, 2, 3, 4, 5];
        let checker = make_range_checker(&data);
        assert!(checker(3));
        assert!(!checker(10));
    }

    #[test]
    fn test_filter_struct() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let filter = Filter::new(&data, |&x| x % 2 == 0);
        let result: Vec<i32> = filter.apply().into_iter().cloned().collect();
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_validator() {
        let min = 10;
        let max = 20;
        let validate = make_validator(&min, &max);
        assert!(validate(15));
        assert!(!validate(5));
        assert!(!validate(25));
    }

    #[test]
    fn test_nested_borrow() {
        let outer = vec![1, 2, 3];
        let checker = make_range_checker(&outer);
        // checker is valid as long as outer is in scope
        assert!(checker(2));
    }
}
