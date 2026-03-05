/// Y Combinator — Fixed-point combinator for anonymous recursive functions.
///
/// The Y combinator enables recursion without named function bindings.
/// In OCaml, a recursive type wrapper (`Fix`) is needed because the type
/// system doesn't allow infinite types directly. In Rust, we use a similar
/// approach with a newtype wrapper and `Fn` trait objects.
use std::cell::RefCell;
use std::rc::Rc;

// ── Solution 1: Idiomatic Rust — using Rc<RefCell> for self-reference ──

/// Y combinator: takes a "template" function that receives a self-reference
/// as its first argument, and returns a closure that is fully recursive.
///
/// OCaml: `val y : (('a -> 'b) -> 'a -> 'b) -> 'a -> 'b`
///
/// In OCaml, `type 'a fix = Fix of ('a fix -> 'a)` wraps the recursive type.
/// In Rust, we use `Rc<dyn Fn>` to erase the recursive type behind a pointer.
pub fn y<A: Copy + 'static, R: 'static>(
    f: impl Fn(&dyn Fn(A) -> R, A) -> R + 'static,
) -> Box<dyn Fn(A) -> R> {
    // Store the final closure in a RefCell so it can reference itself
    type DynFn<A, R> = Rc<dyn Fn(A) -> R>;
    let holder: Rc<RefCell<Option<DynFn<A, R>>>> = Rc::new(RefCell::new(None));
    let f = Rc::new(f);

    let holder_clone = Rc::clone(&holder);
    let closure: Rc<dyn Fn(A) -> R> = Rc::new(move |a: A| -> R {
        let self_ref = holder_clone.borrow();
        let self_fn = Rc::clone(self_ref.as_ref().expect("closure initialized"));
        drop(self_ref); // Release borrow before calling (f may recurse)
        f(&*self_fn, a)
    });

    *holder.borrow_mut() = Some(Rc::clone(&closure));

    Box::new(move |a: A| closure(a))
}

// ── Solution 2: Struct-based approach (closer to OCaml's Fix type) ──

/// Wrapper type analogous to OCaml's `type 'a fix = Fix of ('a fix -> 'a)`.
/// The struct wraps a function that takes a reference to itself.
type FixFn<'a, A, R> = Box<dyn Fn(&Fix<'a, A, R>, A) -> R + 'a>;

struct Fix<'a, A, R> {
    f: FixFn<'a, A, R>,
}

/// Y combinator using the Fix wrapper — mirrors the OCaml implementation directly.
///
/// OCaml:
/// ```text
/// let y f =
///   let g (Fix x as w) = f (fun a -> x w a) in
///   g (Fix g)
/// ```
pub fn y_fix<A: Copy + 'static, R: 'static>(
    f: impl Fn(&dyn Fn(A) -> R, A) -> R + 'static,
) -> Box<dyn Fn(A) -> R> {
    let f = std::sync::Arc::new(f);
    let f_clone = std::sync::Arc::clone(&f);

    let fix = Fix {
        f: Box::new(move |fix: &Fix<A, R>, a: A| {
            let self_fn = |arg: A| (fix.f)(fix, arg);
            f_clone(&self_fn, a)
        }),
    };

    Box::new(move |a: A| (fix.f)(&fix, a))
}

// ── Solution 3: Trait-based approach — using a helper trait ──

/// Helper trait that makes a function callable with self-reference.
pub trait Recursive<A, R> {
    fn call(&self, arg: A) -> R;
}

/// Wrapper that ties the recursive knot via a trait implementation.
pub struct RecFn<F>(pub F);

impl<A: Copy, R, F> Recursive<A, R> for RecFn<F>
where
    F: Fn(&dyn Fn(A) -> R, A) -> R,
{
    fn call(&self, arg: A) -> R {
        (self.0)(&|a| self.call(a), arg)
    }
}

/// Create factorial using the Y combinator.
/// OCaml: `let factorial = y (fun self n -> if n = 0 then 1 else n * self (n - 1))`
pub fn factorial_y() -> Box<dyn Fn(u64) -> u64> {
    y(|self_fn: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n == 0 {
            1
        } else {
            n * self_fn(n - 1)
        }
    })
}

/// Create fibonacci using the Y combinator.
/// OCaml: `let fibonacci = y (fun self n -> if n <= 1 then n else self (n-1) + self (n-2))`
pub fn fibonacci_y() -> Box<dyn Fn(u64) -> u64> {
    y(|self_fn: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n <= 1 {
            n
        } else {
            self_fn(n - 1) + self_fn(n - 2)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        let fact = factorial_y();
        assert_eq!(fact(0), 1);
    }

    #[test]
    fn test_factorial_ten() {
        let fact = factorial_y();
        assert_eq!(fact(10), 3_628_800);
    }

    #[test]
    fn test_fibonacci_base_cases() {
        let fib = fibonacci_y();
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_fibonacci_ten() {
        let fib = fibonacci_y();
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn test_y_fix_factorial() {
        let fact = y_fix(|self_fn: &dyn Fn(u64) -> u64, n: u64| -> u64 {
            if n == 0 {
                1
            } else {
                n * self_fn(n - 1)
            }
        });
        assert_eq!(fact(5), 120);
        assert_eq!(fact(10), 3_628_800);
    }

    #[test]
    fn test_trait_based_factorial() {
        let rec_fact = RecFn(|self_fn: &dyn Fn(u64) -> u64, n: u64| -> u64 {
            if n == 0 {
                1
            } else {
                n * self_fn(n - 1)
            }
        });
        assert_eq!(rec_fact.call(0), 1);
        assert_eq!(rec_fact.call(5), 120);
        assert_eq!(rec_fact.call(10), 3_628_800);
    }

    #[test]
    fn test_trait_based_fibonacci() {
        let rec_fib = RecFn(|self_fn: &dyn Fn(u64) -> u64, n: u64| -> u64 {
            if n <= 1 {
                n
            } else {
                self_fn(n - 1) + self_fn(n - 2)
            }
        });
        assert_eq!(rec_fib.call(0), 0);
        assert_eq!(rec_fib.call(1), 1);
        assert_eq!(rec_fib.call(10), 55);
    }
}
