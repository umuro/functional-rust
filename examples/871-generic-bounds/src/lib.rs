// Example 077: Generic Bounds
// OCaml type constraints → Rust <T: Trait> bounds

use std::fmt::Display;

// === Approach 1: Single trait bound ===
fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    slice.iter().reduce(|a, b| if a >= b { a } else { b })
}

fn find_min<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    slice.iter().reduce(|a, b| if a <= b { a } else { b })
}

// === Approach 2: Multiple trait bounds ===
fn print_max<T: PartialOrd + Display>(slice: &[T]) -> Option<String> {
    find_max(slice).map(|v| format!("Max: {}", v))
}

fn clamp<T: PartialOrd>(value: T, lo: T, hi: T) -> T {
    if value < lo {
        lo
    } else if value > hi {
        hi
    } else {
        value
    }
}

// === Approach 3: Custom trait with bounds ===
trait Summarize: Display {
    fn summary(&self) -> String;
}

struct Stats {
    name: String,
    value: f64,
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={:.2}", self.name, self.value)
    }
}

impl Summarize for Stats {
    fn summary(&self) -> String {
        format!("[{}]", self)
    }
}

fn print_summaries<T: Summarize>(items: &[T]) -> String {
    items.iter().map(|i| i.summary()).collect::<Vec<_>>().join(", ")
}

// Generic pair operations with bounds
fn pair_map<T, U, F: Fn(T) -> U>(pair: (T, T), f: F) -> (U, U) {
    (f(pair.0), f(pair.1))
}

fn pair_fold<T, A, F: Fn(A, T) -> A>(init: A, pair: (T, T), f: F) -> A {
    let acc = f(init, pair.0);
    f(acc, pair.1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[3, 1, 4, 1, 5, 9]), Some(&9));
        assert_eq!(find_max::<i32>(&[]), None);
        assert_eq!(find_max(&[42]), Some(&42));
    }

    #[test]
    fn test_find_min() {
        assert_eq!(find_min(&[3, 1, 4, 1, 5, 9]), Some(&1));
        assert_eq!(find_min::<i32>(&[]), None);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(15, 0, 10), 10);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(1.5, 0.0, 1.0), 1.0);
    }

    #[test]
    fn test_print_max() {
        assert_eq!(print_max(&[1, 2, 3]), Some("Max: 3".to_string()));
        assert_eq!(print_max::<i32>(&[]), None);
    }

    #[test]
    fn test_pair_map() {
        assert_eq!(pair_map((3, 4), |x| x * 2), (6, 8));
        assert_eq!(pair_map((1.0, 2.0), |x: f64| x.sqrt()), (1.0, std::f64::consts::SQRT_2));
    }

    #[test]
    fn test_pair_fold() {
        assert_eq!(pair_fold(0, (3, 4), |acc, x| acc + x), 7);
    }

    #[test]
    fn test_summarize() {
        let s = Stats { name: "x".into(), value: 1.0 };
        assert_eq!(s.summary(), "[x=1.00]");
    }
}
