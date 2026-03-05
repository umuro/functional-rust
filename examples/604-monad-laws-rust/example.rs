trait Monad: Sized {
    type Inner;
    type Wrapped<B>: Monad<Inner=B>;
    fn unit(a: Self::Inner) -> Self;
    fn bind<B>(self, f: impl FnOnce(Self::Inner) -> Self::Wrapped<B>) -> Self::Wrapped<B>;
}

impl<A> Monad for Option<A> {
    type Inner = A;
    type Wrapped<B> = Option<B>;
    fn unit(a: A) -> Option<A> { Some(a) }
    fn bind<B>(self, f: impl FnOnce(A) -> Option<B>) -> Option<B> { self.and_then(f) }
}

impl<A,E: Clone> Monad for Result<A,E> {
    type Inner = A;
    type Wrapped<B> = Result<B,E>;
    fn unit(a: A) -> Result<A,E> { Ok(a) }
    fn bind<B>(self, f: impl FnOnce(A) -> Result<B,E>) -> Result<B,E> { self.and_then(f) }
}

// Law verification for Option
fn left_identity<A: Clone,B: PartialEq>(a: A, f: impl Fn(A) -> Option<B> + Clone) -> bool {
    let left  = Option::unit(a.clone()).bind(f.clone());
    let right = f(a);
    left == right
}

fn right_identity<A: PartialEq + Clone>(m: Option<A>) -> bool {
    let left = m.clone().bind(Option::unit);
    left == m
}

fn associativity<A: Clone, B: Clone, C: PartialEq>(
    m: Option<A>,
    f: impl Fn(A) -> Option<B> + Clone,
    g: impl Fn(B) -> Option<C> + Clone,
) -> bool {
    let left  = m.clone().bind(f.clone()).bind(g.clone());
    let right = m.bind(move |x| f(x).bind(g.clone()));
    left == right
}

fn main() {
    let f = |x: i32| if x > 0 { Some(x*2) } else { None };
    let g = |x: i32| if x < 100 { Some(x+1) } else { None };

    println!("Left identity (5, f):       {}", left_identity(5, f));
    println!("Right identity (Some(5)):   {}", right_identity(Some(5)));
    println!("Right identity (None):      {}", right_identity(None));
    println!("Associativity (Some(5),f,g):{}", associativity(Some(5), f, g));
    println!("Associativity (None,f,g):   {}", associativity(None, f, g));

    // Do-notation simulation via ? in functions
    fn compute(s: &str) -> Option<i32> {
        let n = s.parse::<i32>().ok()?;
        let doubled = if n > 0 { Some(n*2) } else { None }?;
        Some(doubled + 1)
    }
    println!("compute('5') = {:?}", compute("5"));
    println!("compute('-1')= {:?}", compute("-1"));
    println!("compute('x') = {:?}", compute("x"));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn f(x: i32) -> Option<i32> { if x>0 { Some(x*2) } else { None } }
    fn g(x: i32) -> Option<i32> { if x<100 { Some(x+1) } else { None } }
    #[test] fn test_left_id()  { assert!(left_identity(5, f)); }
    #[test] fn test_right_id() { assert!(right_identity(Some(5))); assert!(right_identity::<i32>(None)); }
    #[test] fn test_assoc()    { assert!(associativity(Some(5), f, g)); }
}
