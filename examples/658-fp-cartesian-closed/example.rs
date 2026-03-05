// Cartesian Closed Categories in Rust

// Terminal object
type Terminal = ();

// Product
type Product<A, B> = (A, B);

// Exponential (function)
type Exp<A, B> = Box<dyn FnOnce(A) -> B>;

// curry: (A × B → C) → (A → B → C)
fn curry<A: Clone + 'static, B: 'static, C: 'static>(
    f: impl FnOnce((A, B)) -> C + 'static,
) -> impl FnOnce(A) -> Exp<B, C> {
    move |a| Box::new(move |b| f((a, b)))
}

// eval: (B^A × A) → B
fn eval<A, B>(pair: (Exp<A, B>, A)) -> B {
    (pair.0)(pair.1)
}

fn main() {
    // Curry example
    let add = |(a, b): (i32, i32)| a + b;
    let curried = curry(add);
    let add_5 = curried(5);
    println!("5 + 3 = {}", add_5(3));
    
    // Eval example
    let f: Exp<i32, String> = Box::new(|x| format!("Value: {}", x));
    println!("{}", eval((f, 42)));
}
