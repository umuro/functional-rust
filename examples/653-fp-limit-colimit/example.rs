// Limits and Colimits in Rust

// Product (Limit)
type Product<A, B> = (A, B);

// Coproduct (Colimit)
enum Either<A, B> { Left(A), Right(B) }

// Equalizer: filter where f(x) == g(x)
fn equalizer<A: Clone, B: PartialEq>(
    items: Vec<A>,
    f: impl Fn(&A) -> B,
    g: impl Fn(&A) -> B,
) -> Vec<A> {
    items.into_iter().filter(|a| f(a) == g(a)).collect()
}

// Pullback: SQL-style JOIN
fn pullback<A: Clone, B: Clone, K: PartialEq>(
    xs: &[A], ys: &[B],
    f: impl Fn(&A) -> K, g: impl Fn(&B) -> K,
) -> Vec<(A, B)> {
    xs.iter().flat_map(|a| {
        let ka = f(a);
        ys.iter().filter(|b| g(b) == ka).map(|b| (a.clone(), b.clone())).collect::<Vec<_>>()
    }).collect()
}

fn main() {
    // Equalizer: find where x % 2 == x % 4
    let eq = equalizer(vec![0,1,2,3,4,5,6,7,8], |x| x % 2, |x| x % 4);
    println!("Equalizer: {:?}", eq);
    
    // Pullback as JOIN
    let users = vec![(1, "Alice"), (2, "Bob")];
    let orders = vec![(1, "Widget"), (1, "Gadget"), (2, "Thing")];
    let joined = pullback(&users, &orders, |u| u.0, |o| o.0);
    println!("Joined: {:?}", joined);
}
