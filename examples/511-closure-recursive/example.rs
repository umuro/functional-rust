//! # 511. Recursive Closures (Y Combinator)
//! Techniques for self-referential closures and the Y combinator in Rust.

/// Approach 1: Named recursive function (simplest — always prefer this)
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

/// Approach 2: Open recursion — pass "self" as argument
/// The closure takes itself as a parameter
fn factorial_open() {
    let fact_step = |self_: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n <= 1 { 1 } else { n * self_(n - 1) }
    };

    // We need a wrapper that threads the self reference
    // This requires a recursive function:
    fn apply(step: &dyn Fn(&dyn Fn(u64) -> u64, u64) -> u64, n: u64) -> u64 {
        step(&|m| apply(step, m), n)
    }

    println!("open recursion factorial(6) = {}", apply(&fact_step, 6));
}

/// Approach 3: Y combinator using Box<dyn Fn>
/// Breaks the type recursion with boxing
struct Y<A, B>(Box<dyn Fn(&Y<A, B>, A) -> B>);

impl<A, B> Y<A, B> {
    fn call(&self, arg: A) -> B {
        (self.0)(self, arg)
    }
}

fn y_combinator<A, B, F>(f: F) -> impl Fn(A) -> B
where
    F: Fn(&dyn Fn(A) -> B, A) -> B + 'static,
    A: 'static,
    B: 'static,
{
    let step = Y(Box::new(move |this: &Y<A, B>, arg: A| f(&|a| this.call(a), arg)));
    move |arg| step.call(arg)
}

/// Approach 4: Using a closure that captures a Rc<RefCell<...>>
fn recursive_via_rc() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let fib: Rc<RefCell<Box<dyn Fn(u64) -> u64>>> =
        Rc::new(RefCell::new(Box::new(|_| 0)));

    let fib_clone = fib.clone();
    *fib.borrow_mut() = Box::new(move |n| {
        if n <= 1 { n }
        else { fib_clone.borrow()(n - 1) + fib_clone.borrow()(n - 2) }
    });

    println!("fib(10) via Rc<RefCell> = {}", fib.borrow()(10));
}

fn main() {
    // Named recursive function (preferred)
    println!("factorial(5) = {}", factorial(5));
    println!("factorial(10) = {}", factorial(10));

    // Open recursion
    factorial_open();

    // Y combinator
    let fact_y = y_combinator(|self_: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n <= 1 { 1 } else { n * self_(n - 1) }
    });
    println!("Y combinator factorial(7) = {}", fact_y(7));

    let fib_y = y_combinator(|self_: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n <= 1 { n } else { self_(n - 1) + self_(n - 2) }
    });
    println!("Y combinator fib(10) = {}", fib_y(10));

    // Rc<RefCell> approach
    recursive_via_rc();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_y_factorial() {
        let fact = y_combinator(|self_: &dyn Fn(u64) -> u64, n: u64| -> u64 {
            if n <= 1 { 1 } else { n * self_(n - 1) }
        });
        assert_eq!(fact(5), 120);
        assert_eq!(fact(0), 1);
    }

    #[test]
    fn test_y_fibonacci() {
        let fib = y_combinator(|self_: &dyn Fn(u64) -> u64, n: u64| -> u64 {
            match n {
                0 => 0,
                1 => 1,
                _ => self_(n - 1) + self_(n - 2),
            }
        });
        assert_eq!(fib(0), 0);
        assert_eq!(fib(10), 55);
    }
}
