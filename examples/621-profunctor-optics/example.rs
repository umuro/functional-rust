// Profunctor optics — the most general encoding
// We'll implement the core profunctor infrastructure

// A Profunctor: contravariant in first arg, covariant in second
trait Profunctor {
    type P<A,B>;
    fn dimap<A,B,C,D>(f: impl Fn(C)->A, g: impl Fn(B)->D, p: Self::P<A,B>) -> Self::P<C,D>;
}

// Strong profunctor: has first/second (enables Lens)
trait Strong: Profunctor {
    fn first<A,B,C>(p: Self::P<A,B>) -> Self::P<(A,C),(B,C)>;
    fn second<A,B,C>(p: Self::P<A,B>) -> Self::P<(C,A),(C,B)>;
}

// Choice profunctor: has left/right (enables Prism)
trait Choice: Profunctor {
    fn left<A,B,C>(p: Self::P<A,B>) -> Self::P<Result<A,C>,Result<B,C>>;
}

// The Function profunctor: most natural profunctor
struct FnPro;
// We encode P<A,B> as a closure A->B stored as a concrete type
// Rust makes this hard with GATs, so we use a concrete function wrapper

// Practical encoding: Lens as (S -> A, (B, S) -> T)
// This is the van Laarhoven / concrete encoding

type OpticFn<S, T, A, B> = Box<dyn Fn(Box<dyn Fn(A) -> B>) -> Box<dyn Fn(S) -> T>>;

// Simple concrete lens via function composition
fn lens_via_fn<S, T, A, B>(
    get: impl Fn(&S) -> A + 'static,
    set: impl Fn(B, &S) -> T + 'static,
) -> impl Fn(impl Fn(A)->B+'static) -> impl Fn(S)->T {
    move |f| {
        let g = f;
        move |s| set(g(get(&s)), &s)
    }
}

// Prism via function composition
fn prism_via_fn<S, T, A, B>(
    preview: impl Fn(&S) -> Option<A> + Clone,
    review:  impl Fn(B) -> T + Clone,
    inject:  impl Fn(S) -> T + Clone,  // how to "pass through" when no match
) -> impl Fn(impl Fn(A)->B) -> impl Fn(S)->T {
    move |f| {
        let preview = preview.clone();
        let review  = review.clone();
        let inject  = inject.clone();
        move |s| match preview(&s) {
            Some(a) => review(f(a)),
            None    => inject(s),
        }
    }
}

fn main() {
    // Lens as profunctor optic
    #[derive(Debug,Clone)] struct Point { x: f64, y: f64 }
    let x_lens = lens_via_fn::<Point, Point, f64, f64>(
        |p| p.x,
        |x, p| Point { x, y: p.y },
    );

    let p = Point { x: 1.0, y: 2.0 };
    let p2 = x_lens(|x| x * 10.0)(p);
    println!("x*10: {:?}", p2);

    // Prism as profunctor optic
    let some_prism = prism_via_fn::<Option<i32>,Option<i32>,i32,i32>(
        |o| *o,
        |b| Some(b),
        |_| None,
    );
    println!("Some(5) *2 = {:?}", some_prism(|x| x*2)(Some(5)));
    println!("None    *2 = {:?}", some_prism(|x| x*2)(None));

    // Composition is just function composition
    // (lens1 . lens2)(f) = lens1(lens2(f))
    println!("Profunctor optics: composition = function composition");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug,Clone,PartialEq)] struct P { x:f64,y:f64 }
    #[test] fn lens_test() {
        let lx = lens_via_fn::<P,P,f64,f64>(|p|p.x, |x,p|P{x,y:p.y});
        let p = P{x:3.0,y:4.0};
        let p2 = lx(|x|x+1.0)(p);
        assert!((p2.x - 4.0).abs() < 1e-10);
    }
}
