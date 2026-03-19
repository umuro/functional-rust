//! # Boxing Closures — Dynamic Dispatch

use std::collections::HashMap;

/// Box closure for dynamic dispatch
pub type BoxedFn = Box<dyn Fn(i32) -> i32>;
pub type BoxedFnMut = Box<dyn FnMut(i32) -> i32>;
pub type BoxedFnOnce = Box<dyn FnOnce(i32) -> i32>;

pub fn make_boxed_adder(n: i32) -> BoxedFn {
    Box::new(move |x| x + n)
}

/// Store different closures in a collection
pub fn closure_map() -> HashMap<String, BoxedFn> {
    let mut map: HashMap<String, BoxedFn> = HashMap::new();
    map.insert("double".into(), Box::new(|x| x * 2));
    map.insert("square".into(), Box::new(|x| x * x));
    map.insert("negate".into(), Box::new(|x| -x));
    map
}

/// Chain of boxed closures
pub fn chain_closures(closures: Vec<BoxedFn>, value: i32) -> i32 {
    closures.iter().fold(value, |acc, f| f(acc))
}

/// Conditional closure selection
pub fn select_operation(op: &str) -> Option<BoxedFn> {
    match op {
        "add1" => Some(Box::new(|x| x + 1)),
        "double" => Some(Box::new(|x| x * 2)),
        "square" => Some(Box::new(|x| x * x)),
        _ => None,
    }
}

/// Vector of closures
pub struct ClosureVec {
    closures: Vec<BoxedFn>,
}

impl ClosureVec {
    pub fn new() -> Self {
        Self {
            closures: Vec::new(),
        }
    }

    pub fn add<F: Fn(i32) -> i32 + 'static>(&mut self, f: F) {
        self.closures.push(Box::new(f));
    }

    pub fn apply_all(&self, x: i32) -> Vec<i32> {
        self.closures.iter().map(|f| f(x)).collect()
    }
}

impl Default for ClosureVec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxed_adder() {
        let add5 = make_boxed_adder(5);
        assert_eq!(add5(10), 15);
    }

    #[test]
    fn test_closure_map() {
        let ops = closure_map();
        assert_eq!(ops.get("double").unwrap()(5), 10);
        assert_eq!(ops.get("square").unwrap()(4), 16);
    }

    #[test]
    fn test_chain() {
        let closures: Vec<BoxedFn> = vec![
            Box::new(|x| x + 1),
            Box::new(|x| x * 2),
            Box::new(|x| x - 3),
        ];
        // (5+1)*2-3 = 9
        assert_eq!(chain_closures(closures, 5), 9);
    }

    #[test]
    fn test_select() {
        let op = select_operation("double").unwrap();
        assert_eq!(op(21), 42);
    }

    #[test]
    fn test_closure_vec() {
        let mut cv = ClosureVec::new();
        cv.add(|x| x + 1);
        cv.add(|x| x * 2);
        cv.add(|x| x - 5);
        assert_eq!(cv.apply_all(10), vec![11, 20, 5]);
    }
}
