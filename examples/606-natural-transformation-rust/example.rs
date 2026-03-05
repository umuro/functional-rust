// Natural transformation: F<A> -> G<A>, polymorphic in A
// Rust can express this with generic functions

// η: Option<A> -> Vec<A>
fn opt_to_vec<A>(o: Option<A>) -> Vec<A> {
    match o { Some(x) => vec![x], None => vec![] }
}

// η: Vec<A> -> Option<A> (head)
fn vec_to_opt<A>(v: Vec<A>) -> Option<A> {
    v.into_iter().next()
}

// η: Result<A,E> -> Option<A>
fn result_to_opt<A,E>(r: Result<A,E>) -> Option<A> { r.ok() }

// η: Option<A> -> Result<A,&str>
fn opt_to_result<A>(o: Option<A>) -> Result<A, &'static str> {
    o.ok_or("missing value")
}

// Naturality condition checker:
// fmap_G(f) ∘ η == η ∘ fmap_F(f)
fn naturality_opt_to_vec<A: Clone + PartialEq, B: PartialEq>(
    opt: Option<A>,
    f: impl Fn(A) -> B,
) -> bool {
    // Left side: η(fmap_F(f)(opt)) = opt_to_vec(opt.map(f))
    let left: Vec<B> = opt_to_vec(opt.clone().map(&f));
    // Right side: fmap_G(f)(η(opt)) = opt_to_vec(opt).into_iter().map(f)...
    let right: Vec<B> = opt_to_vec(opt).into_iter().map(f).collect();
    left == right
}

fn main() {
    println!("opt_to_vec(Some(42)) = {:?}", opt_to_vec(Some(42)));
    println!("opt_to_vec(None::<i32>) = {:?}", opt_to_vec(None::<i32>));
    println!("vec_to_opt([1,2,3]) = {:?}", vec_to_opt(vec![1,2,3]));
    println!("vec_to_opt([]::<i32>) = {:?}", vec_to_opt(Vec::<i32>::new()));

    // Verify naturality
    let f = |x: i32| x * 2;
    println!("Naturality (Some(5)):  {}", naturality_opt_to_vec(Some(5), f));
    println!("Naturality (None):     {}", naturality_opt_to_vec(None::<i32>, f));

    // Chain natural transforms
    let r: Result<i32, &str> = Ok(42);
    let o: Option<i32> = result_to_opt(r);
    let v: Vec<i32>    = opt_to_vec(o);
    println!("Result -> Option -> Vec: {:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn naturality_some() { assert!(naturality_opt_to_vec(Some(5i32), |x|x*2)); }
    #[test] fn naturality_none() { assert!(naturality_opt_to_vec(None::<i32>, |x:i32|x*2)); }
    #[test] fn round_trip() {
        let v = vec![1,2,3];
        let o = vec_to_opt(v.clone());
        assert_eq!(o, Some(1));
    }
}
