// Natural Transformations in Rust

// Option => Vec
fn option_to_vec<A>(opt: Option<A>) -> Vec<A> {
    opt.into_iter().collect()
}

// Vec => Option (head)
fn vec_to_option<A>(v: Vec<A>) -> Option<A> {
    v.into_iter().next()
}

// Verify naturality: η_B ∘ F(f) = G(f) ∘ η_A
fn verify_naturality() -> bool {
    let f = |x: i32| x * 2;
    let a = Some(10);
    
    // Route 1: map then transform
    let r1 = option_to_vec(a.map(f));
    
    // Route 2: transform then map
    let r2: Vec<_> = option_to_vec(a).into_iter().map(f).collect();
    
    r1 == r2
}

fn main() {
    println!("option_to_vec(Some(42)): {:?}", option_to_vec(Some(42)));
    println!("vec_to_option([1,2,3]): {:?}", vec_to_option(vec![1,2,3]));
    println!("Naturality holds: {}", verify_naturality());
}
