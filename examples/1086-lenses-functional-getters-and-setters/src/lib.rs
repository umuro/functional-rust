/// A `Lens<S, A>` focuses on a part `A` within a whole `S`.
///
/// A lens is a first-class getter/setter pair that composes.
/// In OCaml, a lens is a record with `get` and `set` fields.
/// In Rust, we model it as a struct holding two boxed closures.
///
/// - `get`: extracts the focused value from the whole
/// - `set`: replaces the focused value, returning a new whole
pub struct Lens<S, A> {
    getter: Getter<S, A>,
    setter: Setter<S, A>,
}

type Getter<S, A> = Box<dyn Fn(&S) -> A>;
type Setter<S, A> = Box<dyn Fn(A, &S) -> S>;

impl<S: 'static, A: 'static> Lens<S, A> {
    /// Create a lens from a getter and setter function.
    pub fn new(getter: impl Fn(&S) -> A + 'static, setter: impl Fn(A, &S) -> S + 'static) -> Self {
        Lens {
            getter: Box::new(getter),
            setter: Box::new(setter),
        }
    }

    /// Get the focused value from the whole.
    pub fn get(&self, s: &S) -> A {
        (self.getter)(s)
    }

    /// Set the focused value, returning a new whole (immutable update).
    pub fn set(&self, a: A, s: &S) -> S {
        (self.setter)(a, s)
    }

    /// Apply a function to the focused value — the `over` combinator.
    ///
    /// OCaml: `let over lens f s = lens.set (f (lens.get s)) s`
    /// This is the functional equivalent of "modify in place".
    pub fn over(&self, f: impl Fn(A) -> A, s: &S) -> S {
        let a = self.get(s);
        self.set(f(a), s)
    }

    /// Compose two lenses: `self` focuses on `A` inside `S`,
    /// `inner` focuses on `B` inside `A`.
    /// The result focuses on `B` inside `S`.
    ///
    /// OCaml:
    /// ```text
    /// let compose outer inner = {
    ///   get = (fun s -> inner.get (outer.get s));
    ///   set = (fun a s -> outer.set (inner.set a (outer.get s)) s);
    /// }
    /// ```
    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where
        A: Clone,
    {
        // Move both lenses into Rc so the closures can share them
        use std::rc::Rc;
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

// ---------------------------------------------------------------------------
// Domain types — mirrors the OCaml example
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    pub street: String,
    pub city: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub addr: Address,
}

/// Lens focusing on the `addr` field of a `Person`.
pub fn addr_lens() -> Lens<Person, Address> {
    Lens::new(
        |p: &Person| p.addr.clone(), // clone needed: we return an owned Address
        |a, p| Person {
            name: p.name.clone(), // immutable update: rebuild with new addr
            addr: a,
        },
    )
}

/// Lens focusing on the `city` field of an `Address`.
pub fn city_lens() -> Lens<Address, String> {
    Lens::new(
        |a: &Address| a.city.clone(),
        |c, a| Address {
            street: a.street.clone(),
            city: c,
        },
    )
}

/// Lens focusing on the `street` field of an `Address`.
pub fn street_lens() -> Lens<Address, String> {
    Lens::new(
        |a: &Address| a.street.clone(),
        |s, a| Address {
            street: s,
            city: a.city.clone(),
        },
    )
}

/// Lens focusing on the `name` field of a `Person`.
pub fn name_lens() -> Lens<Person, String> {
    Lens::new(
        |p: &Person| p.name.clone(),
        |n, p| Person {
            name: n,
            addr: p.addr.clone(),
        },
    )
}

/// Composed lens: Person → Address → city (String).
pub fn person_city_lens() -> Lens<Person, String> {
    addr_lens().compose(city_lens())
}

/// Composed lens: Person → Address → street (String).
pub fn person_street_lens() -> Lens<Person, String> {
    addr_lens().compose(street_lens())
}

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

    // -- Basic lens get/set --

    #[test]
    fn test_get_city() {
        let p = sample_person();
        let lens = city_lens();
        let addr = &p.addr;
        assert_eq!(lens.get(addr), "NYC");
    }

    #[test]
    fn test_set_city() {
        let p = sample_person();
        let lens = city_lens();
        let new_addr = lens.set("LA".to_string(), &p.addr);
        assert_eq!(new_addr.city, "LA");
        // Original unchanged (immutable update)
        assert_eq!(p.addr.city, "NYC");
    }

    #[test]
    fn test_get_addr() {
        let p = sample_person();
        let lens = addr_lens();
        let addr = lens.get(&p);
        assert_eq!(addr.city, "NYC");
        assert_eq!(addr.street, "Main St");
    }

    #[test]
    fn test_set_addr() {
        let p = sample_person();
        let lens = addr_lens();
        let new_addr = Address {
            street: "Broadway".to_string(),
            city: "SF".to_string(),
        };
        let p2 = lens.set(new_addr, &p);
        assert_eq!(p2.addr.city, "SF");
        assert_eq!(p2.name, "Alice");
    }

    // -- Composed lens --

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
        // Name and street are preserved
        assert_eq!(p2.name, "Alice");
        assert_eq!(p2.addr.street, "Main St");
    }

    // -- Over combinator --

    #[test]
    fn test_over_uppercase() {
        let p = sample_person();
        let lens = person_city_lens();
        let p2 = lens.over(|c| c.to_uppercase(), &p);
        assert_eq!(lens.get(&p2), "NYC"); // "NYC" is already uppercase
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
        assert_eq!(p2.name, "Bob");
    }

    // -- Immutability / original unchanged --

    #[test]
    fn test_immutability_preserved() {
        let p = sample_person();
        let lens = person_city_lens();
        let _p2 = lens.set("Chicago".to_string(), &p);
        // Original person is unchanged
        assert_eq!(lens.get(&p), "NYC");
    }

    // -- Lens laws --

    #[test]
    fn test_get_set_law() {
        // get-set: setting what you got changes nothing
        let p = sample_person();
        let lens = person_city_lens();
        let city = lens.get(&p);
        let p2 = lens.set(city, &p);
        assert_eq!(p, p2);
    }

    #[test]
    fn test_set_get_law() {
        // set-get: getting what you set yields the set value
        let p = sample_person();
        let lens = person_city_lens();
        let p2 = lens.set("Denver".to_string(), &p);
        assert_eq!(lens.get(&p2), "Denver");
    }

    #[test]
    fn test_set_set_law() {
        // set-set: setting twice is the same as setting once with the last value
        let p = sample_person();
        let lens = person_city_lens();
        let p2 = lens.set("A".to_string(), &p);
        let p3 = lens.set("B".to_string(), &p2);
        let p4 = lens.set("B".to_string(), &p);
        assert_eq!(p3, p4);
    }

    // -- Additional composed lenses --

    #[test]
    fn test_person_street_lens() {
        let p = sample_person();
        let lens = person_street_lens();
        assert_eq!(lens.get(&p), "Main St");
        let p2 = lens.set("Oak Ave".to_string(), &p);
        assert_eq!(lens.get(&p2), "Oak Ave");
        assert_eq!(p2.addr.city, "NYC"); // city preserved
    }

    #[test]
    fn test_name_lens() {
        let p = sample_person();
        let lens = name_lens();
        assert_eq!(lens.get(&p), "Alice");
        let p2 = lens.set("Bob".to_string(), &p);
        assert_eq!(lens.get(&p2), "Bob");
        assert_eq!(p2.addr.city, "NYC"); // addr preserved
    }
}
