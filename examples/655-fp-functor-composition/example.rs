// Functor Composition in Rust

// Option<Vec<A>> functor
fn map_option_vec<A, B>(ov: Option<Vec<A>>, f: impl Fn(A) -> B) -> Option<Vec<B>> {
    ov.map(|v| v.into_iter().map(f).collect())
}

// Vec<Option<A>> functor
fn map_vec_option<A, B>(vo: Vec<Option<A>>, f: impl Fn(A) -> B) -> Vec<Option<B>> {
    vo.into_iter().map(|opt| opt.map(&f)).collect()
}

fn main() {
    let ov = Some(vec![1, 2, 3]);
    println!("map_option_vec: {:?}", map_option_vec(ov, |x| x * 2));
    
    let vo = vec![Some(1), None, Some(3)];
    println!("map_vec_option: {:?}", map_vec_option(vo, |x| x * 2));
    
    // Verify composition law: map (f . g) = map f . map g
    let data = Some(vec![1, 2, 3]);
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 2;
    
    let r1 = map_option_vec(data.clone(), |x| g(f(x)));
    let r2 = map_option_vec(map_option_vec(data, f), g);
    println!("Composition law holds: {}", r1 == r2);
}
