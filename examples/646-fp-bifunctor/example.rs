// Bifunctor in Rust

fn bimap<A, B, C, D, F, G>(pair: (A, B), f: F, g: G) -> (C, D)
where F: FnOnce(A) -> C, G: FnOnce(B) -> D {
    (f(pair.0), g(pair.1))
}

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn bimap<L2, R2, F, G>(self, f: F, g: G) -> Either<L2, R2>
    where F: FnOnce(L) -> L2, G: FnOnce(R) -> R2 {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(g(r)),
        }
    }
}

fn main() {
    let pair = (10, "hello");
    let result = bimap(pair, |x| x * 2, |s| s.len());
    println!("bimap tuple: {:?}", result);
    
    let left: Either<i32, &str> = Either::Left(5);
    let mapped = left.bimap(|x| x * 10, |s| s.to_uppercase());
    println!("bimap left: {:?}", mapped);
}
