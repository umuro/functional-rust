// Kan Extensions in Rust

// Codensity monad = Right Kan Extension of Id along Id
struct Codensity<A> {
    run: Box<dyn FnOnce(Box<dyn FnOnce(A) -> A>) -> A>,
}

impl<A: 'static> Codensity<A> {
    fn pure(a: A) -> Self {
        Codensity { run: Box::new(move |k| k(a)) }
    }
    
    fn flat_map<B: 'static, F: FnOnce(A) -> Codensity<B> + 'static>(self, f: F) -> Codensity<B> {
        Codensity {
            run: Box::new(move |k| (self.run)(Box::new(move |a| (f(a).run)(k))))
        }
    }
    
    fn run(self) -> A {
        (self.run)(Box::new(|a| a))
    }
}

fn main() {
    // Codensity improves left-associative bind performance
    let result = Codensity::pure(1)
        .flat_map(|x| Codensity::pure(x + 1))
        .flat_map(|x| Codensity::pure(x * 2))
        .flat_map(|x| Codensity::pure(x + 10))
        .run();
    
    println!("Result: {}", result); // 14
}
