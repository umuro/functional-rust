//! # 528. Closures Capturing References
//! How closure lifetimes are constrained by captured borrows.

/// Closure captures &str — lifetime tied to the string's scope
fn make_prefix_checker<'a>(prefix: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |s| s.starts_with(prefix)
}

/// Multiple borrows in a closure
fn make_range_checker<'a>(data: &'a [i32]) -> impl Fn(i32) -> bool + 'a {
    move |target| data.contains(&target)
}

/// Struct holding a closure that borrows
struct Filter<'a, T> {
    data: &'a [T],
    predicate: Box<dyn Fn(&T) -> bool + 'a>,
}

impl<'a, T> Filter<'a, T> {
    fn new(data: &'a [T], pred: impl Fn(&T) -> bool + 'a) -> Self {
        Filter { data, predicate: Box::new(pred) }
    }

    fn apply(&self) -> Vec<&T> {
        self.data.iter().filter(|x| (self.predicate)(x)).collect()
    }
}

fn main() {
    // Closure borrows prefix — closure lifetime <= prefix's lifetime
    let prefix = String::from("hello");
    let checker = make_prefix_checker(&prefix);
    println!("'hello world' starts with 'hello': {}", checker("hello world"));
    println!("'hi there' starts with 'hello': {}", checker("hi there"));
    drop(checker); // drop closure before dropping prefix
    drop(prefix);  // now safe to drop prefix

    // Range checker borrowing a slice
    let allowed = vec![1, 3, 5, 7, 9];
    let is_allowed = make_range_checker(&allowed);
    for n in 0..=10 {
        if is_allowed(n) { print!("{} ", n); }
    }
    println!("are allowed");

    // Filter struct borrowing its data
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let threshold = 5;
    let filter = Filter::new(&numbers, move |&x| x > threshold);
    let result = filter.apply();
    println!("numbers > {}: {:?}", threshold, result);

    // Named lifetime prevents use-after-free
    // This WON'T compile — closure outlives borrow:
    // let checker2;
    // {
    //     let local = String::from("temp");
    //     checker2 = make_prefix_checker(&local); // ERROR
    // }
    // checker2("test"); // local is gone!

    // Shared borrow — multiple closures can borrow the same data
    let data = vec![1, 2, 3, 4, 5];
    let sum_closure = || data.iter().sum::<i32>();
    let max_closure = || data.iter().max().copied();
    println!("sum: {}, max: {:?}", sum_closure(), max_closure());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_checker() {
        let prefix = String::from("rust");
        let check = make_prefix_checker(&prefix);
        assert!(check("rustacean"));
        assert!(!check("python"));
    }

    #[test]
    fn test_range_checker() {
        let allowed = vec![2, 4, 6, 8];
        let check = make_range_checker(&allowed);
        assert!(check(4));
        assert!(!check(5));
    }

    #[test]
    fn test_filter_struct() {
        let data = vec![1, 2, 3, 4, 5];
        let f = Filter::new(&data, |&x| x % 2 == 0);
        let evens = f.apply();
        assert_eq!(evens, vec![&2, &4]);
    }
}
