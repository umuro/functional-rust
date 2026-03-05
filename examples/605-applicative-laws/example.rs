trait Applicative: Sized {
    type Inner;
    type Wrapped<B>;
    fn pure_(a: Self::Inner) -> Self;
    fn ap<B>(self, f: Self) -> Self::Wrapped<B>
    where Self: Applicative<Inner = fn(Self::Inner) -> B>;
}

// Practical Option applicative
fn ap_option<A, B>(mf: Option<fn(A)->B>, ma: Option<A>) -> Option<B> {
    match (mf, ma) { (Some(f), Some(a)) => Some(f(a)), _ => None }
}

fn pure_opt<A>(a: A) -> Option<A> { Some(a) }

// Law verification
fn identity_law<A: PartialEq + Copy>(v: Option<A>) -> bool {
    fn id<T>(x: T) -> T { x }
    ap_option(pure_opt(id as fn(A)->A), v) == v
}

fn homomorphism_law<A: PartialEq + Copy>(f: fn(A)->A, x: A) -> bool {
    ap_option(pure_opt(f), pure_opt(x)) == pure_opt(f(x))
}

fn interchange_law<A: PartialEq + Copy>(u: Option<fn(A)->A>, y: A) -> bool {
    let apply_to_y = move |f: fn(A)->A| f(y);
    ap_option(u, pure_opt(y)) == ap_option(pure_opt(apply_to_y as fn(fn(A)->A)->A), u)
}

// Applicative style: combine independent Options
fn validate_user(name: Option<&str>, age: Option<u32>) -> Option<String> {
    match (name, age) {
        (Some(n), Some(a)) if !n.is_empty() && a >= 18 => Some(format!("{} ({})", n, a)),
        _ => None,
    }
}

// mapN using ap — N independent effects
fn map2<A: Copy, B: Copy, C>(fa: Option<A>, fb: Option<B>, f: impl Fn(A,B)->C) -> Option<C> {
    match (fa, fb) { (Some(a),Some(b)) => Some(f(a,b)), _ => None }
}

fn main() {
    fn double(x: i32) -> i32 { x*2 }
    println!("identity(Some(42)):    {}", identity_law(Some(42)));
    println!("identity(None):        {}", identity_law::<i32>(None));
    println!("homomorphism(double,5):{}", homomorphism_law(double, 5));
    println!("interchange:           {}",
        interchange_law(Some(double as fn(i32)->i32), 10));

    println!("validate('Alice',25): {:?}", validate_user(Some("Alice"), Some(25)));
    println!("validate('',25):      {:?}", validate_user(Some(""),     Some(25)));
    println!("map2 Some(3),Some(4): {:?}", map2(Some(3i32), Some(4i32), |a,b|a+b));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn double(x: i32) -> i32 { x*2 }
    #[test] fn test_identity()     { assert!(identity_law(Some(5))); }
    #[test] fn test_homomorphism() { assert!(homomorphism_law(double, 5)); }
    #[test] fn test_interchange()  { assert!(interchange_law(Some(double as fn(i32)->i32), 10)); }
}
