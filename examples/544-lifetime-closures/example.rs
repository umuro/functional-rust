//! # 544. Lifetimes in Closures
//! Captured references constrain closure lifetimes.

/// Return a closure that captures a reference to data
/// The closure can't outlive `data` — bounded by `'data`
fn make_sum_adder<'data>(data: &'data [i32]) -> impl Fn(i32) -> i32 + 'data {
    // closure captures &'data [i32]
    let sum: i32 = data.iter().sum();
    move |x| x + sum // sum is i32 (Copy), no lifetime issue
    // If we captured data directly: move |x| data.iter().sum::<i32>() + x
    // That would also require + 'data
}

/// Closure capturing a &str — lifetime annotation required
fn make_prefixer<'a>(prefix: &'a str) -> impl Fn(&str) -> String + 'a {
    move |s| format!("{}: {}", prefix, s)
}

/// Closure capturing multiple references with a lifetime bound
/// 'b: 'a means prefix outlives suffix — closure lives for 'a (shorter)
fn make_formatter<'a>(
    prefix: &'a str,
    suffix: &'a str,
) -> impl Fn(&str) -> String + 'a {
    move |s| format!("{}{}{}", prefix, s, suffix)
}

/// Closure in a struct — must annotate
struct Filter<'a> {
    predicate: Box<dyn Fn(i32) -> bool + 'a>,
}

impl<'a> Filter<'a> {
    fn from_slice(allowed: &'a [i32]) -> Self {
        Filter {
            predicate: Box::new(move |x| allowed.contains(&x)),
        }
    }

    fn check(&self, x: i32) -> bool { (self.predicate)(x) }
}

fn main() {
    // Closure captures by-value (no lifetime issue)
    let data = vec![1, 2, 3, 4, 5];
    let add_sum = make_sum_adder(&data);
    println!("add_sum(10) = {}", add_sum(10)); // 10 + 15 = 25
    // data still usable (closure borrowed it, sum is Copy)
    println!("data: {:?}", data);

    // Closure captures &str
    let prefix = String::from("INFO");
    let log;
    {
        let p = prefix.as_str();
        log = make_prefixer(p);
        println!("{}", log("server started"));
        println!("{}", log("connected"));
    } // p is a borrow — log can't be used after this if p is dropped
    // But prefix is still alive, so log is valid through prefix:
    println!("{}", make_prefixer(&prefix)("still works"));

    // Multi-lifetime closure
    let pre = String::from("[");
    let suf = String::from("]");
    let bracket = make_formatter(&pre, &suf);
    println!("{}", bracket("hello"));
    println!("{}", bracket("world"));

    // Closure in struct borrowing slice
    let allowed = vec![2, 4, 6, 8, 10];
    let filter = Filter::from_slice(&allowed);
    for n in 1..=12 {
        if filter.check(n) { print!("{} ", n); }
    }
    println!("(allowed even numbers)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_adder() {
        let data = vec![1, 2, 3];
        let f = make_sum_adder(&data);
        assert_eq!(f(0), 6);
        assert_eq!(f(10), 16);
    }

    #[test]
    fn test_prefixer() {
        let p = String::from("LOG");
        let f = make_prefixer(&p);
        assert_eq!(f("msg"), "LOG: msg");
    }

    #[test]
    fn test_filter_from_slice() {
        let allowed = vec![1, 3, 5];
        let f = Filter::from_slice(&allowed);
        assert!(f.check(1));
        assert!(f.check(3));
        assert!(!f.check(2));
    }
}
