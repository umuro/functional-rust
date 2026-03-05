// Affine Traversal: zero-or-one focus
struct Affine<S, A> {
    preview_fn: Box<dyn Fn(&S) -> Option<A>>,
    set_fn:     Box<dyn Fn(A, S) -> S>,
}

impl<S: Clone, A: Clone> Affine<S, A> {
    fn new(preview_fn: impl Fn(&S)->Option<A>+'static, set_fn: impl Fn(A,S)->S+'static) -> Self {
        Affine { preview_fn: Box::new(preview_fn), set_fn: Box::new(set_fn) }
    }
    fn preview(&self, s: &S) -> Option<A> { (self.preview_fn)(s) }
    fn set(&self, a: A, s: S) -> S { (self.set_fn)(a, s) }
    fn over(&self, f: impl Fn(A)->A, s: S) -> S {
        match self.preview(&s) {
            Some(a) => self.set(f(a), s),
            None    => s,
        }
    }
}

// Domain: optional nested access
#[derive(Debug,Clone)]
struct Address { city: String, zip: String }
#[derive(Debug,Clone)]
struct User { name: String, address: Option<Address> }

fn main() {
    // Affine for optional address city
    let user_city: Affine<User, String> = Affine::new(
        |u| u.address.as_ref().map(|a| a.city.clone()),
        |city, mut u| {
            if let Some(ref mut a) = u.address { a.city = city; }
            u
        },
    );

    let users = vec![
        User { name: "Alice".into(), address: Some(Address { city:"Boston".into(), zip:"02101".into() }) },
        User { name: "Bob".into(),   address: None },
    ];

    for u in &users {
        match user_city.preview(u) {
            Some(c) => println!("{} lives in {}", u.name, c),
            None    => println!("{} has no address", u.name),
        }
    }

    // Update Alice's city
    let alice = users[0].clone();
    let alice2 = user_city.set("Cambridge".into(), alice);
    println!("Alice moved to: {:?}", user_city.preview(&alice2));

    // Over: no-op on Bob (no address)
    let bob = users[1].clone();
    let bob2 = user_city.over(|c| c.to_uppercase(), bob);
    println!("Bob's city: {:?}", user_city.preview(&bob2));

    // Affine for first element of a list
    let head: Affine<Vec<i32>, i32> = Affine::new(
        |v| v.first().copied(),
        |x, mut v| { if !v.is_empty() { v[0]=x; } v },
    );
    let v = vec![1,2,3];
    println!("head: {:?}", head.preview(&v));
    let v2 = head.over(|x|x*10, v);
    println!("head*10: {:?}", v2);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_user() -> User {
        User{name:"A".into(), address:Some(Address{city:"B".into(),zip:"".into()})}
    }
    #[test] fn preview_some() {
        let a: Affine<User,String> = Affine::new(
            |u|u.address.as_ref().map(|a|a.city.clone()),
            |c,mut u|{if let Some(ref mut a)=u.address{a.city=c;}u});
        assert_eq!(a.preview(&make_user()), Some("B".into()));
    }
}
