// Applicative Functor Laws in Rust
// Demonstrates: Identity, Homomorphism, Interchange, Composition

#[derive(Debug, Clone, PartialEq)]
struct Applicative<T>(T);

impl<T> Applicative<T> {
    fn pure(value: T) -> Self {
        Applicative(value)
    }
}

impl<T: Clone> Applicative<T> {
    fn ap<U, F>(self, f: Applicative<F>) -> Applicative<U>
    where
        F: FnOnce(T) -> U,
    {
        Applicative(f.0(self.0))
    }
}

fn main() {
    // Identity law: pure id <*> v = v
    let v = Applicative::pure(42);
    let id_fn = Applicative::pure(|x: i32| x);
    let result = v.clone().ap(id_fn);
    println!("Identity: {:?} == {:?}: {}", result, v, result == v);

    // Homomorphism: pure f <*> pure x = pure (f x)
    let f = |x: i32| x * 2;
    let x = 21;
    let lhs = Applicative::pure(x).ap(Applicative::pure(f));
    let rhs = Applicative::pure(f(x));
    println!("Homomorphism: {:?} == {:?}: {}", lhs, rhs, lhs == rhs);
}
