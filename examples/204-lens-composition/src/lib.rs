/// Type alias for a boxed getter function.
type GetFn<S, A> = Box<dyn Fn(&S) -> A>;
/// Type alias for a boxed setter function.
type SetFn<S, A> = Box<dyn Fn(A, &S) -> S>;

/// A Lens<S, A> focuses on a field of type A inside a structure S.
/// `get` extracts the field; `set` returns a new S with the field replaced.
pub struct Lens<S, A> {
    pub get: GetFn<S, A>,
    pub set: SetFn<S, A>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    pub fn new(get: impl Fn(&S) -> A + 'static, set: impl Fn(A, &S) -> S + 'static) -> Self {
        Lens {
            get: Box::new(get),
            set: Box::new(set),
        }
    }

    /// Compose two lenses: `self` focuses S→A, `inner` focuses A→B.
    /// Result is a single Lens<S, B> that traverses both levels at once.
    ///
    /// get:  s  ->  inner.get(self.get(s))
    /// set:  (b, s) ->  self.set(inner.set(b, self.get(s)), s)
    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where
        A: Clone,
    {
        // Pull the boxed closures out so they can be moved into new closures.
        let outer_get = self.get;
        let outer_set = self.set;
        let inner_get = inner.get;
        let inner_set = inner.set;

        // outer_get is needed by both the new `get` and `set` closures.
        // Wrap in Rc so both closures can share the same allocation without copying.
        use std::rc::Rc;
        let outer_get = Rc::new(outer_get);
        let outer_get2 = Rc::clone(&outer_get);

        Lens {
            get: Box::new(move |s| inner_get(&outer_get(s))),
            set: Box::new(move |b, s| {
                let a: A = outer_get2(s);
                let a2 = inner_set(b, &a);
                outer_set(a2, s)
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Domain types for the running example
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Street {
    pub number: u32,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Address {
    pub street: Street,
    pub city: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    pub pname: String,
    pub address: Address,
}

// ---------------------------------------------------------------------------
// Individual lenses — explicit closure types so the compiler can infer S/A
// ---------------------------------------------------------------------------

/// Lens: Person → Address
pub fn person_address_lens() -> Lens<Person, Address> {
    Lens::new(
        |p: &Person| p.address.clone(),
        |a: Address, p: &Person| Person {
            address: a,
            ..p.clone()
        },
    )
}

/// Lens: Address → Street
pub fn address_street_lens() -> Lens<Address, Street> {
    Lens::new(
        |a: &Address| a.street.clone(),
        |s: Street, a: &Address| Address {
            street: s,
            ..a.clone()
        },
    )
}

/// Lens: Street → number
pub fn street_number_lens() -> Lens<Street, u32> {
    Lens::new(
        |s: &Street| s.number,
        |n: u32, s: &Street| Street {
            number: n,
            ..s.clone()
        },
    )
}

/// Lens: Street → name
pub fn street_name_lens() -> Lens<Street, String> {
    Lens::new(
        |s: &Street| s.name.clone(),
        |name: String, s: &Street| Street { name, ..s.clone() },
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_person() -> Person {
        Person {
            pname: "Alice".into(),
            address: Address {
                city: "Wonderland".into(),
                street: Street {
                    number: 42,
                    name: "Main St".into(),
                },
            },
        }
    }

    #[test]
    fn test_compose_two_lenses_get() {
        // person_address |> address_street gives Person → Street
        let person_street = person_address_lens().compose(address_street_lens());
        let alice = sample_person();
        let street = (person_street.get)(&alice);
        assert_eq!(street.number, 42);
        assert_eq!(street.name, "Main St");
    }

    #[test]
    fn test_compose_two_lenses_set() {
        let person_street = person_address_lens().compose(address_street_lens());
        let alice = sample_person();
        let new_street = Street {
            number: 99,
            name: "Oak Ave".into(),
        };
        let updated = (person_street.set)(new_street, &alice);
        assert_eq!(updated.address.street.number, 99);
        assert_eq!(updated.address.street.name, "Oak Ave");
        // Other fields unchanged
        assert_eq!(updated.pname, "Alice");
        assert_eq!(updated.address.city, "Wonderland");
    }

    #[test]
    fn test_compose_three_lenses_get() {
        // person → address → street → number (three levels deep)
        let person_number = person_address_lens()
            .compose(address_street_lens())
            .compose(street_number_lens());
        let alice = sample_person();
        assert_eq!((person_number.get)(&alice), 42);
    }

    #[test]
    fn test_compose_three_lenses_set() {
        let person_number = person_address_lens()
            .compose(address_street_lens())
            .compose(street_number_lens());
        let alice = sample_person();
        let updated = (person_number.set)(7, &alice);
        assert_eq!(updated.address.street.number, 7);
        // Untouched fields survive
        assert_eq!(updated.address.street.name, "Main St");
        assert_eq!(updated.address.city, "Wonderland");
        assert_eq!(updated.pname, "Alice");
    }

    #[test]
    fn test_individual_lens_person_address() {
        let lens = person_address_lens();
        let alice = sample_person();
        let addr = (lens.get)(&alice);
        assert_eq!(addr.city, "Wonderland");

        let new_addr = Address {
            city: "Oz".into(),
            street: addr.street.clone(),
        };
        let updated = (lens.set)(new_addr, &alice);
        assert_eq!(updated.address.city, "Oz");
        assert_eq!(updated.pname, "Alice");
    }

    #[test]
    fn test_individual_lens_street_number() {
        let lens = street_number_lens();
        let s = Street {
            number: 10,
            name: "Elm".into(),
        };
        assert_eq!((lens.get)(&s), 10);
        let s2 = (lens.set)(20, &s);
        assert_eq!(s2.number, 20);
        assert_eq!(s2.name, "Elm");
    }

    #[test]
    fn test_composition_is_associative() {
        // (person_address |> address_street) |> street_number
        //   should equal
        // person_address |> (address_street |> street_number)
        // We verify both give the same get/set results on the same data.
        let alice = sample_person();

        let left = person_address_lens()
            .compose(address_street_lens())
            .compose(street_number_lens());
        let right =
            person_address_lens().compose(address_street_lens().compose(street_number_lens()));

        assert_eq!((left.get)(&alice), (right.get)(&alice));

        let l_updated = (left.set)(100, &alice);
        let r_updated = (right.set)(100, &alice);
        assert_eq!(l_updated, r_updated);
    }
}
