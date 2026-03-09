/// A lens is a first-class getter/setter pair that composes.
///
/// In OCaml, a lens is a record with `get` and `set` fields.
/// In Rust, we model it as a struct holding two closures.
use std::rc::Rc;

pub struct Lens<S, A> {
    getter: Box<dyn Fn(&S) -> A>,
    setter: Box<dyn Fn(A, &S) -> S>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    pub fn new(
        getter: impl Fn(&S) -> A + 'static,
        setter: impl Fn(A, &S) -> S + 'static,
    ) -> Self {
        Lens {
            getter: Box::new(getter),
            setter: Box::new(setter),
        }
    }

    pub fn get(&self, s: &S) -> A {
        (self.getter)(s)
    }

    pub fn set(&self, a: A, s: &S) -> S {
        (self.setter)(a, s)
    }

    /// Apply a function to the focused value — the `over` combinator.
    pub fn over(&self, f: impl Fn(A) -> A, s: &S) -> S {
        let a = self.get(s);
        self.set(f(a), s)
    }

    /// Compose two lenses: outer(A inside S) + inner(B inside A) → (B inside S).
    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where
        A: Clone,
    {
        let outer_get = Rc::new(self.getter);
        let outer_set = Rc::new(self.setter);
        let inner_get = Rc::new(inner.getter);
        let inner_set = Rc::new(inner.setter);

        let og = Rc::clone(&outer_get);
        let ig = Rc::clone(&inner_get);
        let composed_get = move |s: &S| -> B { ig(&og(s)) };

        let og2 = Rc::clone(&outer_get);
        let os2 = Rc::clone(&outer_set);
        let is2 = Rc::clone(&inner_set);
        let composed_set = move |b: B, s: &S| -> S {
            let a = og2(s);
            let a_new = is2(b, &a);
            os2(a_new, s)
        };

        Lens::new(composed_get, composed_set)
    }
}

// Domain types
#[derive(Debug, Clone, PartialEq)]
struct Address {
    street: String,
    city: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    addr: Address,
}

fn addr_lens() -> Lens<Person, Address> {
    Lens::new(
        |p: &Person| p.addr.clone(),
        |a, p| Person {
            name: p.name.clone(),
            addr: a,
        },
    )
}

fn city_lens() -> Lens<Address, String> {
    Lens::new(
        |a: &Address| a.city.clone(),
        |c, a| Address {
            street: a.street.clone(),
            city: c,
        },
    )
}

fn person_city_lens() -> Lens<Person, String> {
    addr_lens().compose(city_lens())
}

fn main() {
    let p = Person {
        name: "Alice".to_string(),
        addr: Address {
            street: "Main St".to_string(),
            city: "NYC".to_string(),
        },
    };

    let lens = person_city_lens();

    println!("City: {}", lens.get(&p));

    let p2 = lens.over(|c| c.to_uppercase(), &p);
    println!("City: {}", lens.get(&p2));

    // Demonstrate immutability — original is unchanged
    println!("Original city: {}", lens.get(&p));

    // Demonstrate set
    let p3 = lens.set("Boston".to_string(), &p);
    println!("After set: {}", lens.get(&p3));
}

/* Output:
   City: NYC
   City: NYC
   Original city: NYC
   After set: Boston
*/

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_person() -> Person {
        Person {
            name: "Alice".to_string(),
            addr: Address {
                street: "Main St".to_string(),
                city: "NYC".to_string(),
            },
        }
    }

    #[test]
    fn test_get_city() {
        let lens = city_lens();
        let addr = &sample_person().addr;
        assert_eq!(lens.get(addr), "NYC");
    }

    #[test]
    fn test_set_city() {
        let p = sample_person();
        let lens = city_lens();
        let new_addr = lens.set("LA".to_string(), &p.addr);
        assert_eq!(new_addr.city, "LA");
        assert_eq!(p.addr.city, "NYC");
    }

    #[test]
    fn test_composed_get() {
        let p = sample_person();
        let lens = person_city_lens();
        assert_eq!(lens.get(&p), "NYC");
    }

    #[test]
    fn test_composed_set() {
        let p = sample_person();
        let lens = person_city_lens();
        let p2 = lens.set("Boston".to_string(), &p);
        assert_eq!(lens.get(&p2), "Boston");
        assert_eq!(p2.name, "Alice");
        assert_eq!(p2.addr.street, "Main St");
    }

    #[test]
    fn test_over_transforms() {
        let p = Person {
            name: "Bob".to_string(),
            addr: Address {
                street: "Elm St".to_string(),
                city: "london".to_string(),
            },
        };
        let lens = person_city_lens();
        let p2 = lens.over(|c| c.to_uppercase(), &p);
        assert_eq!(lens.get(&p2), "LONDON");
    }

    #[test]
    fn test_get_set_law() {
        let p = sample_person();
        let lens = person_city_lens();
        let city = lens.get(&p);
        let p2 = lens.set(city, &p);
        assert_eq!(p, p2);
    }

    #[test]
    fn test_set_get_law() {
        let p = sample_person();
        let lens = person_city_lens();
        let p2 = lens.set("Denver".to_string(), &p);
        assert_eq!(lens.get(&p2), "Denver");
    }

    #[test]
    fn test_set_set_law() {
        let p = sample_person();
        let lens = person_city_lens();
        let p2 = lens.set("A".to_string(), &p);
        let p3 = lens.set("B".to_string(), &p2);
        let p4 = lens.set("B".to_string(), &p);
        assert_eq!(p3, p4);
    }
}
