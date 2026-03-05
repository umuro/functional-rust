//! # Futumorphism
//! Unfold that can produce multiple layers at once.

pub fn futu<S, A>(seed: S, step: impl Fn(S) -> FutuStep<A, S>) -> Vec<A> {
    let mut result = Vec::new();
    let mut s = seed;
    loop {
        match step(s) {
            FutuStep::Done => break,
            FutuStep::One(a, next) => { result.push(a); s = next; }
            FutuStep::Two(a, b, next) => { result.push(a); result.push(b); s = next; }
        }
    }
    result
}

pub enum FutuStep<A, S> { Done, One(A, S), Two(A, A, S) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_futu() {
        let xs = futu(0, |n| if n >= 4 { FutuStep::Done } else { FutuStep::One(n, n+1) });
        assert_eq!(xs, vec![0,1,2,3]);
    }
}
