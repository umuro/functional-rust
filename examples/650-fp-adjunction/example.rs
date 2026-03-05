// Adjunction in Rust - Curry/Uncurry example

/// Curry: (A, B) -> C  ≅  A -> B -> C
fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    F: Fn(A, B) -> C + Clone + 'static,
    A: 'static,
    B: 'static,
    C: 'static,
{
    move |a| {
        let f = f.clone();
        Box::new(move |b| f(a.clone(), b))
    }
}

// State monad from Product-Exponential adjunction
struct State<S, A> {
    run: Box<dyn FnOnce(S) -> (A, S)>,
}

impl<S: Clone + 'static, A: 'static> State<S, A> {
    fn new<F: FnOnce(S) -> (A, S) + 'static>(f: F) -> Self {
        State { run: Box::new(f) }
    }
    
    fn pure(a: A) -> Self {
        State::new(move |s| (a, s))
    }
    
    fn run(self, s: S) -> (A, S) {
        (self.run)(s)
    }
}

fn main() {
    // Curry example
    let add = |a: i32, b: i32| a + b;
    let curried = curry(add);
    let add_5 = curried(5);
    println!("5 + 3 = {}", add_5(3));
    
    // State monad example
    let comp = State::pure(42);
    let (result, state) = comp.run(0);
    println!("Result: {}, State: {}", result, state);
}
