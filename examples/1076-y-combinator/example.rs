use std::cell::RefCell;
use std::rc::Rc;

/// Y combinator using Rc<RefCell> for self-reference.
fn y<A: Copy + 'static, R: 'static>(
    f: impl Fn(&dyn Fn(A) -> R, A) -> R + 'static,
) -> Box<dyn Fn(A) -> R> {
    type DynFn<A, R> = Rc<dyn Fn(A) -> R>;
    let holder: Rc<RefCell<Option<DynFn<A, R>>>> = Rc::new(RefCell::new(None));
    let f = Rc::new(f);

    let holder_clone = Rc::clone(&holder);
    let closure: Rc<dyn Fn(A) -> R> = Rc::new(move |a: A| -> R {
        let self_ref = holder_clone.borrow();
        let self_fn = Rc::clone(self_ref.as_ref().expect("closure initialized"));
        drop(self_ref);
        f(&*self_fn, a)
    });

    *holder.borrow_mut() = Some(Rc::clone(&closure));
    Box::new(move |a: A| closure(a))
}

/// Trait-based Y combinator alternative.
trait Recursive<A, R> {
    fn call(&self, arg: A) -> R;
}

struct RecFn<F>(F);

impl<A: Copy, R, F> Recursive<A, R> for RecFn<F>
where
    F: Fn(&dyn Fn(A) -> R, A) -> R,
{
    fn call(&self, arg: A) -> R {
        (self.0)(&|a| self.call(a), arg)
    }
}

fn main() {
    // Y combinator factorial
    let factorial = y(|self_fn: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n == 0 { 1 } else { n * self_fn(n - 1) }
    });

    // Y combinator fibonacci
    let fibonacci = y(|self_fn: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n <= 1 { n } else { self_fn(n - 1) + self_fn(n - 2) }
    });

    println!("10! = {}", factorial(10));
    println!("fib(10) = {}", fibonacci(10));

    // Trait-based approach
    let rec_fact = RecFn(|self_fn: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n == 0 { 1 } else { n * self_fn(n - 1) }
    });
    println!("5! (trait) = {}", rec_fact.call(5));
}

/* Output:
   10! = 3628800
   fib(10) = 55
   5! (trait) = 120
*/
