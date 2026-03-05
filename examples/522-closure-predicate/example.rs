//! # 522. Predicate Functions Pattern
//! Composable predicates: and, or, not, all_of, any_of.

/// Combine two predicates with AND
fn pred_and<T, P1, P2>(p1: P1, p2: P2) -> impl Fn(&T) -> bool
where
    P1: Fn(&T) -> bool,
    P2: Fn(&T) -> bool,
{
    move |x| p1(x) && p2(x)
}

/// Combine two predicates with OR
fn pred_or<T, P1, P2>(p1: P1, p2: P2) -> impl Fn(&T) -> bool
where
    P1: Fn(&T) -> bool,
    P2: Fn(&T) -> bool,
{
    move |x| p1(x) || p2(x)
}

/// Negate a predicate
fn pred_not<T, P>(p: P) -> impl Fn(&T) -> bool
where
    P: Fn(&T) -> bool,
{
    move |x| !p(x)
}

/// All predicates must hold
fn all_of<T>(predicates: Vec<Box<dyn Fn(&T) -> bool>>) -> impl Fn(&T) -> bool {
    move |x| predicates.iter().all(|p| p(x))
}

/// At least one predicate must hold
fn any_of<T>(predicates: Vec<Box<dyn Fn(&T) -> bool>>) -> impl Fn(&T) -> bool {
    move |x| predicates.iter().any(|p| p(x))
}

fn main() {
    let is_even     = |x: &i32| x % 2 == 0;
    let is_positive = |x: &i32| *x > 0;
    let is_small    = |x: &i32| *x < 100;

    let is_valid = all_of(vec![
        Box::new(is_even),
        Box::new(is_positive),
        Box::new(is_small),
    ]);

    let nums = vec![-2, 0, 4, 8, 102, -6, 50, 99];
    let valid: Vec<_> = nums.iter().filter(|x| is_valid(x)).collect();
    println!("valid (even, positive, <100): {:?}", valid);

    let extreme = pred_or(|x: &i32| *x < 0, |x: &i32| *x > 50);
    let extremes: Vec<_> = nums.iter().filter(|&&ref x| extreme(x)).collect();
    println!("extreme (<0 or >50): {:?}", extremes);

    // Negation
    let is_odd = pred_not(|x: &i32| x % 2 == 0);
    let odds: Vec<_> = nums.iter().filter(|x| is_odd(x)).collect();
    println!("odds: {:?}", odds);

    // String predicates
    let words = vec!["hello", "hi", "hey", "world", "rust", "hat"];
    let starts_h = |w: &&str| w.starts_with('h');
    let long = |w: &&str| w.len() > 3;
    let h_and_long: Vec<_> = words.iter().filter(|w| starts_h(w) && long(w)).collect();
    println!("starts with 'h' AND len>3: {:?}", h_and_long);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pred_and() {
        let f = pred_and(|x: &i32| *x > 0, |x: &i32| *x < 10);
        assert!(f(&5));
        assert!(!f(&0));
        assert!(!f(&10));
    }

    #[test]
    fn test_pred_or() {
        let f = pred_or(|x: &i32| *x < 0, |x: &i32| *x > 100);
        assert!(f(&-1));
        assert!(f(&200));
        assert!(!f(&50));
    }

    #[test]
    fn test_pred_not() {
        let is_even = |x: &i32| x % 2 == 0;
        let is_odd = pred_not(is_even);
        assert!(is_odd(&3));
        assert!(!is_odd(&4));
    }

    #[test]
    fn test_all_of() {
        let pred = all_of(vec![
            Box::new(|x: &i32| *x > 0),
            Box::new(|x: &i32| *x < 100),
            Box::new(|x: &i32| x % 2 == 0),
        ]);
        assert!(pred(&42));
        assert!(!pred(&-2));
        assert!(!pred(&101));
    }
}
