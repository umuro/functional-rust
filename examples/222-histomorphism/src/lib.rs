#![allow(clippy::all)]
// Example 222: Histomorphism — Cata with Full History (Fibonacci in O(n))

// histo: algebra sees all previous results via Cofree

#[derive(Debug, Clone)]
enum NatF<A> {
    ZeroF,
    SuccF(A),
}

impl<A> NatF<A> {
    fn map<B>(self, f: impl Fn(A) -> B) -> NatF<B> {
        match self {
            NatF::ZeroF => NatF::ZeroF,
            NatF::SuccF(a) => NatF::SuccF(f(a)),
        }
    }
    fn map_ref<B>(&self, f: impl Fn(&A) -> B) -> NatF<B> {
        match self {
            NatF::ZeroF => NatF::ZeroF,
            NatF::SuccF(a) => NatF::SuccF(f(a)),
        }
    }
}

// Cofree: result + history
#[derive(Debug, Clone)]
struct Cofree<A> {
    head: A,
    tail: Box<NatF<Cofree<A>>>,
}

impl<A> Cofree<A> {
    fn new(head: A, tail: NatF<Cofree<A>>) -> Self {
        Cofree {
            head,
            tail: Box::new(tail),
        }
    }
}

#[derive(Debug, Clone)]
struct FixNat(Box<NatF<FixNat>>);

fn zero() -> FixNat {
    FixNat(Box::new(NatF::ZeroF))
}
fn succ(n: FixNat) -> FixNat {
    FixNat(Box::new(NatF::SuccF(n)))
}
fn nat(n: u32) -> FixNat {
    (0..n).fold(zero(), |acc, _| succ(acc))
}

// histo: build cofree bottom-up, algebra sees history chain
fn histo<A: Clone>(alg: &dyn Fn(NatF<Cofree<A>>) -> A, fix: &FixNat) -> A {
    histo_build(alg, fix).head
}

fn histo_build<A: Clone>(alg: &dyn Fn(NatF<Cofree<A>>) -> A, fix: &FixNat) -> Cofree<A> {
    let layer = fix.0.map_ref(|child| histo_build(alg, child));
    let result = alg(layer.clone());
    Cofree::new(result, layer)
}

// NatF derives Clone, which covers NatF<Cofree<A>> when A: Clone

// Approach 1: Fibonacci — algebra looks back 2 steps
fn fib_alg(n: NatF<Cofree<u64>>) -> u64 {
    match n {
        NatF::ZeroF => 0,
        NatF::SuccF(cf) => match cf.tail.as_ref() {
            NatF::ZeroF => 1,                       // fib(1) = 1
            NatF::SuccF(cf2) => cf.head + cf2.head, // fib(n-1) + fib(n-2)
        },
    }
}

// Approach 2: Tribonacci — looks back 3 steps
fn trib_alg(n: NatF<Cofree<u64>>) -> u64 {
    match n {
        NatF::ZeroF => 0,
        NatF::SuccF(cf) => match cf.tail.as_ref() {
            NatF::ZeroF => 0, // trib(1) = 0
            NatF::SuccF(cf2) => match cf2.tail.as_ref() {
                NatF::ZeroF => 1, // trib(2) = 1
                NatF::SuccF(cf3) => cf.head + cf2.head + cf3.head,
            },
        },
    }
}

// Approach 3: General "look back n" pattern
fn lucas_alg(n: NatF<Cofree<u64>>) -> u64 {
    match n {
        NatF::ZeroF => 2,
        NatF::SuccF(cf) => match cf.tail.as_ref() {
            NatF::ZeroF => 1,
            NatF::SuccF(cf2) => cf.head + cf2.head,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_20() {
        assert_eq!(histo(&fib_alg, &nat(20)), 6765);
    }
    #[test]
    fn test_trib_7() {
        assert_eq!(histo(&trib_alg, &nat(7)), 13);
    }
    #[test]
    fn test_lucas_6() {
        assert_eq!(histo(&lucas_alg, &nat(6)), 18);
    }
}
