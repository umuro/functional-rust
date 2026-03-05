trait Functor {
    type Inner;
    type Mapped<B>;
    fn fmap<B>(self, f: impl FnMut(Self::Inner) -> B) -> Self::Mapped<B>;
}

impl<A> Functor for Option<A> {
    type Inner = A;
    type Mapped<B> = Option<B>;
    fn fmap<B>(self, f: impl FnMut(A) -> B) -> Option<B> { self.map(f) }
}

impl<A,E> Functor for Result<A,E> {
    type Inner = A;
    type Mapped<B> = Result<B,E>;
    fn fmap<B>(self, f: impl FnMut(A) -> B) -> Result<B,E> { self.map(f) }
}

impl<A> Functor for Vec<A> {
    type Inner = A;
    type Mapped<B> = Vec<B>;
    fn fmap<B>(self, f: impl FnMut(A) -> B) -> Vec<B> { self.into_iter().map(f).collect() }
}

// Law checking
fn check_identity_option<A: Clone + PartialEq>(x: Option<A>) -> bool {
    x.clone().fmap(|v| v) == x
}

fn check_composition_option<A,B,C>(
    x: Option<A>,
    f: impl Fn(A) -> B + Clone,
    g: impl Fn(B) -> C,
) -> bool
where A: Clone, B: Clone + PartialEq, C: PartialEq,
{
    let gf = {let f2 = f.clone(); move |a| g(f2(a))};
    let compose = x.clone().fmap(gf);
    let sequential = x.fmap(f).fmap(g);
    compose == sequential
}

fn main() {
    // Identity law: map(id) = id
    let x: Option<i32> = Some(42);
    println!("Option identity law: {}", check_identity_option(x));

    let xs = vec![1,2,3,4,5];
    let id_check: Vec<i32> = xs.clone().fmap(|x|x);
    println!("Vec identity law: {}", id_check == xs);

    // Composition law: map(g∘f) == map(g)∘map(f)
    let f = |x: i32| x * 2;
    let g = |x: i32| x + 1;
    println!("Option composition law: {}", check_composition_option(Some(5), f, g));

    let comp: Vec<_> = xs.clone().fmap(|x| g(f(x)));
    let seq:  Vec<_> = xs.clone().fmap(f).fmap(g);
    println!("Vec composition law: {}", comp == seq);

    // Demonstrating fmap on Result
    let r: Result<i32, &str> = Ok(5);
    println!("Result fmap: {:?}", r.fmap(|x| x * 10));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn identity_some()  { assert!(check_identity_option(Some(42))); }
    #[test] fn identity_none()  { assert!(check_identity_option::<i32>(None)); }
    #[test] fn composition()    { assert!(check_composition_option(Some(5), |x:i32|x*2, |x|x+1)); }
}
