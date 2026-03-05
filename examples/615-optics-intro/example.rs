// Lens: composable getter/setter for product types
#[derive(Clone)]
struct Lens<S, A> {
    get: fn(&S) -> A,
    set: fn(A, S) -> S,
}

impl<S: Clone, A: Clone> Lens<S, A> {
    fn new(get: fn(&S) -> A, set: fn(A, S) -> S) -> Self { Lens { get, set } }
    fn view(&self, s: &S) -> A { (self.get)(s) }
    fn over(&self, f: impl Fn(A) -> A, s: S) -> S {
        let a = (self.get)(&s);
        (self.set)(f(a), s)
    }
    fn compose<B: Clone>(&self, other: &Lens<A, B>) -> Lens<S, B> {
        let get1 = self.get;
        let set1 = self.set;
        let get2 = other.get;
        let set2 = other.set;
        Lens {
            get: {
                let _ = (get1, get2);
                // We use closures boxed in a workaround since fn pointers can't close
                move |s| get2(&get1(s))
            },
            set: move |b, s| {
                let a = get1(&s);
                set1(set2(b, a), s)
            },
        }
    }
}

// Prism: composable getter/setter for sum types
struct Prism<S, A> {
    preview: fn(&S) -> Option<A>,
    review:  fn(A) -> S,
}

impl<S, A: Clone> Prism<S, A> {
    fn new(preview: fn(&S) -> Option<A>, review: fn(A) -> S) -> Self { Prism { preview, review } }
    fn preview(&self, s: &S) -> Option<A> { (self.preview)(s) }
    fn review(&self, a: A) -> S { (self.review)(a) }
}

// Domain types
#[derive(Debug,Clone)]
struct Address { street: String, city: String, zip: String }
#[derive(Debug,Clone)]
struct Person  { name: String, age: u32, address: Address }

fn main() {
    let name_lens: Lens<Person, String> = Lens::new(
        |p| p.name.clone(),
        |v, mut p| { p.name = v; p },
    );
    let age_lens: Lens<Person, u32> = Lens::new(
        |p| p.age,
        |v, mut p| { p.age = v; p },
    );
    let addr_lens: Lens<Person, Address> = Lens::new(
        |p| p.address.clone(),
        |v, mut p| { p.address = v; p },
    );
    let city_lens: Lens<Address, String> = Lens::new(
        |a| a.city.clone(),
        |v, mut a| { a.city = v; a },
    );

    let p = Person {
        name: "Alice".into(), age: 30,
        address: Address { street: "1 Main St".into(), city: "Boston".into(), zip: "02101".into() },
    };

    println!("name: {}", name_lens.view(&p));
    let city_from_addr = city_lens.view(&addr_lens.view(&p));
    println!("city: {}", city_from_addr);

    let p2 = age_lens.over(|a| a+1, p.clone());
    println!("age+1: {}", age_lens.view(&p2));

    // Prism over Option
    let some_prism: Prism<Option<i32>, i32> = Prism::new(
        |o| *o,
        |a| Some(a),
    );
    println!("preview Some(42): {:?}", some_prism.preview(&Some(42)));
    println!("preview None: {:?}", some_prism.preview(&None));
    println!("review 7: {:?}", some_prism.review(7));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn lens_get() {
        let l: Lens<Person,u32> = Lens::new(|p|p.age, |v,mut p|{p.age=v;p});
        let p = Person{name:"".into(),age:25,address:Address{street:"".into(),city:"".into(),zip:"".into()}};
        assert_eq!(l.view(&p), 25);
    }
}
