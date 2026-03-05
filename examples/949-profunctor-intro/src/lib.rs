// Profunctor: contravariant in input, covariant in output.
//
// A profunctor `p a b` supports:
//   dimap :: (c -> a) -> (b -> d) -> p a b -> p c d
//
// Functions `a -> b` are the classic example:
//   dimap f g p  =  g . p . f   ("adapt input with f, output with g")
//
// Rust can't express full HKT profunctors, but we show the concept
// with a concrete `Mapper<A, B>` struct + dimap method.

// ── Concrete Mapper ──────────────────────────────────────────────────────────

pub struct Mapper<A, B> {
    f: Box<dyn Fn(A) -> B>,
}

impl<A: 'static, B: 'static> Mapper<A, B> {
    pub fn new<F: Fn(A) -> B + 'static>(f: F) -> Self {
        Mapper { f: Box::new(f) }
    }

    pub fn apply(&self, a: A) -> B {
        (self.f)(a)
    }

    /// dimap: pre-compose with `pre` (contramap input), post-compose with `post` (map output).
    /// dimap f g p = post ∘ p ∘ pre
    pub fn dimap<C: 'static, D: 'static>(
        self,
        pre: impl Fn(C) -> A + 'static,
        post: impl Fn(B) -> D + 'static,
    ) -> Mapper<C, D> {
        Mapper::new(move |c| post((self.f)(pre(c))))
    }

    /// lmap: adapt only the input (contramap) — dimap f id
    pub fn lmap<C: 'static>(self, pre: impl Fn(C) -> A + 'static) -> Mapper<C, B> {
        Mapper::new(move |c| (self.f)(pre(c)))
    }

    /// rmap: adapt only the output (covariant map) — dimap id g
    pub fn rmap<D: 'static>(self, post: impl Fn(B) -> D + 'static) -> Mapper<A, D> {
        Mapper::new(move |a| post((self.f)(a)))
    }
}

// ── Star: Mapper lifted into a context ──────────────────────────────────────
// Star f a b = a -> f b   (like Mapper but output is wrapped)
// Demonstrates the same dimap pattern in a richer context.

pub struct Star<A, B> {
    run: Box<dyn Fn(A) -> Option<B>>,
}

impl<A: 'static, B: 'static> Star<A, B> {
    pub fn new<F: Fn(A) -> Option<B> + 'static>(f: F) -> Self {
        Star { run: Box::new(f) }
    }

    pub fn apply(&self, a: A) -> Option<B> {
        (self.run)(a)
    }

    pub fn lmap<C: 'static>(self, pre: impl Fn(C) -> A + 'static) -> Star<C, B> {
        Star::new(move |c| (self.run)(pre(c)))
    }

    pub fn rmap<D: 'static>(self, post: impl Fn(B) -> D + 'static) -> Star<A, D> {
        Star::new(move |a| (self.run)(a).map(|b| post(b)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lmap() {
        let m = Mapper::new(|s: String| s.len())
            .lmap(|n: i32| n.to_string());
        // 42.to_string() = "42", len = 2
        assert_eq!(m.apply(42), 2);
    }

    #[test]
    fn test_rmap() {
        let m = Mapper::new(|s: String| s.to_uppercase())
            .rmap(|s: String| s.len());
        assert_eq!(m.apply("hello".to_string()), 5);
    }

    #[test]
    fn test_dimap() {
        // dimap (to_string) (len) (to_uppercase)
        // 7 -> "7" -> "7" -> 1
        let m = Mapper::new(|s: String| s.to_uppercase())
            .dimap(|n: i32| n.to_string(), |s: String| s.len());
        assert_eq!(m.apply(7), 1);
    }

    #[test]
    fn test_profunctor_identity_law() {
        // dimap id id p = p
        let p1 = Mapper::new(|x: i32| x * 2);
        let p2 = Mapper::new(|x: i32| x * 2).dimap(|x| x, |x| x);
        assert_eq!(p1.apply(21), p2.apply(21));
    }

    #[test]
    fn test_star_lmap_rmap() {
        let parse = Star::new(|s: String| s.parse::<i32>().ok())
            .rmap(|n| n + 10);
        assert_eq!(parse.apply("5".to_string()), Some(15));
        assert_eq!(parse.apply("bad".to_string()), None);
    }
}
