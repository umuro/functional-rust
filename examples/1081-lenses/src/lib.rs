/// A lens is a pair of getter/setter functions that focus on a part of a larger structure.
/// This is the functional approach to accessing and modifying nested data without mutation.
///
/// In OCaml, lenses are records of closures: `{ get: 's -> 'a; set: 'a -> 's -> 's }`.
/// In Rust, we use boxed closures since closures have unique, unsized types.
// Type aliases for lens closure types — avoids clippy::type_complexity
type Getter<S, A> = Box<dyn Fn(&S) -> &A>;
type Setter<S, A> = Box<dyn Fn(A, &S) -> S>;

pub struct Lens<S, A> {
    // Takes a reference to the whole and returns a reference to the part
    get_fn: Getter<S, A>,
    // Takes a new part value and the whole, returns a new whole
    set_fn: Setter<S, A>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    /// Create a new lens from getter and setter functions.
    pub fn new(get: impl Fn(&S) -> &A + 'static, set: impl Fn(A, &S) -> S + 'static) -> Self {
        Lens {
            get_fn: Box::new(get),
            set_fn: Box::new(set),
        }
    }

    /// Focus the lens: get the part from the whole.
    pub fn get<'s>(&self, whole: &'s S) -> &'s A {
        (self.get_fn)(whole)
    }

    /// Update the whole by replacing the focused part.
    pub fn set(&self, value: A, whole: &S) -> S {
        (self.set_fn)(value, whole)
    }

    /// Compose two lenses: `self` focuses on a mid-level part, `inner` focuses deeper.
    /// The result focuses from the outermost structure directly to the innermost part.
    ///
    /// OCaml: `compose outer inner = { get = fun s -> inner.get (outer.get s); ... }`
    /// Rust must clone the mid-level value for the set path because we need both
    /// the old mid-level (to pass to inner.set) and the whole (to pass to outer.set).
    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where
        S: Clone,
        A: Clone,
    {
        // Share the inner lens between get and set closures
        let inner = std::sync::Arc::new(inner);
        let outer_get = std::sync::Arc::new(self.get_fn);
        let outer_set = std::sync::Arc::new(self.set_fn);

        let get_outer = std::sync::Arc::clone(&outer_get);
        let get_inner = std::sync::Arc::clone(&inner);

        let set_outer_get = std::sync::Arc::clone(&outer_get);
        let set_outer_set = std::sync::Arc::clone(&outer_set);
        let set_inner = std::sync::Arc::clone(&inner);

        Lens::new(
            move |s: &S| {
                // SAFETY of lifetime: we return a reference into `s` via two dereferences.
                // The inner reference is valid as long as `s` is valid because both get_fns
                // return references into their argument.
                let mid: &A = (get_outer)(s);
                // We need to extend the lifetime — the borrow checker can't see through
                // the boxed closure that the returned &B borrows from s.
                // This is safe because both get functions return references into their input.
                let mid_ptr: *const A = mid;
                (get_inner.get_fn)(unsafe { &*mid_ptr })
            },
            move |b: B, s: &S| {
                // Get the current mid-level value, clone it so we can modify it
                let mid: &A = (set_outer_get)(s);
                let new_mid: A = (set_inner.set_fn)(b, mid);
                (set_outer_set)(new_mid, s)
            },
        )
    }
}

/// Apply a function to the focused part of a lens, returning an updated whole.
///
/// OCaml: `let over lens f s = lens.set (f (lens.get s)) s`
pub fn over<S: 'static, A: Clone + 'static>(
    lens: &Lens<S, A>,
    f: impl FnOnce(A) -> A,
    whole: &S,
) -> S {
    let current = lens.get(whole).clone(); // clone the part so we can transform it
    lens.set(f(current), whole)
}

// --- Domain types for demonstration ---

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
        |p: &Person| &p.addr,
        |a: Address, p: &Person| Person {
            name: p.name.clone(), // clone name — we're building a new Person
            addr: a,
        },
    )
}

/// Lens focusing on the `city` field of an `Address`.
pub fn city_lens() -> Lens<Address, String> {
    Lens::new(
        |a: &Address| &a.city,
        |c: String, a: &Address| Address {
            street: a.street.clone(), // clone street — we're building a new Address
            city: c,
        },
    )
}

/// Lens focusing on the `street` field of an `Address`.
pub fn street_lens() -> Lens<Address, String> {
    Lens::new(
        |a: &Address| &a.street,
        |s: String, a: &Address| Address {
            street: s,
            city: a.city.clone(), // clone city — we're building a new Address
        },
    )
}

/// Composed lens: Person -> city (via addr).
pub fn person_city_lens() -> Lens<Person, String> {
    addr_lens().compose(city_lens())
}

/// Composed lens: Person -> street (via addr).
pub fn person_street_lens() -> Lens<Person, String> {
    addr_lens().compose(street_lens())
}

pub fn sample_person() -> Person {
    Person {
        name: "Alice".to_string(),
        addr: Address {
            street: "Main St".to_string(),
            city: "NYC".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_get() {
        let p = sample_person();
        let lens = addr_lens();
        assert_eq!(lens.get(&p).city, "NYC");
    }

    #[test]
    fn test_simple_set() {
        let p = sample_person();
        let lens = city_lens();
        let new_addr = lens.set("LA".to_string(), &p.addr);
        assert_eq!(new_addr.city, "LA");
        assert_eq!(new_addr.street, "Main St");
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
        assert_eq!(p2.addr.city, "Boston");
        assert_eq!(p2.name, "Alice");
        assert_eq!(p2.addr.street, "Main St");
    }

    #[test]
    fn test_over_transforms_focused_value() {
        let p = sample_person();
        let lens = person_city_lens();
        let p2 = over(&lens, |c| c.to_lowercase(), &p);
        assert_eq!(p2.addr.city, "nyc");
        assert_eq!(p2.name, "Alice");
    }

    #[test]
    fn test_original_unchanged_after_set() {
        let p = sample_person();
        let lens = person_city_lens();
        let _p2 = lens.set("Boston".to_string(), &p);
        // Original is unchanged — functional update
        assert_eq!(p.addr.city, "NYC");
    }

    #[test]
    fn test_set_get_roundtrip() {
        let p = sample_person();
        let lens = person_city_lens();
        let p2 = lens.set("SF".to_string(), &p);
        assert_eq!(lens.get(&p2), "SF");
    }

    #[test]
    fn test_compose_street_lens() {
        let p = sample_person();
        let lens = person_street_lens();
        assert_eq!(lens.get(&p), "Main St");
        let p2 = lens.set("Oak Ave".to_string(), &p);
        assert_eq!(p2.addr.street, "Oak Ave");
        assert_eq!(p2.addr.city, "NYC");
    }

    #[test]
    fn test_over_with_street() {
        let p = sample_person();
        let lens = person_street_lens();
        let p2 = over(&lens, |s| format!("123 {s}"), &p);
        assert_eq!(lens.get(&p2), "123 Main St");
    }

    #[test]
    fn test_lens_laws_get_set() {
        // Law: set (get s) s == s
        let p = sample_person();
        let lens = person_city_lens();
        let city = lens.get(&p).clone();
        let p2 = lens.set(city, &p);
        assert_eq!(p, p2);
    }

    #[test]
    fn test_lens_laws_set_get() {
        // Law: get (set a s) == a
        let p = sample_person();
        let lens = person_city_lens();
        let p2 = lens.set("Denver".to_string(), &p);
        assert_eq!(lens.get(&p2), "Denver");
    }

    #[test]
    fn test_lens_laws_set_set() {
        // Law: set b (set a s) == set b s
        let p = sample_person();
        let lens = person_city_lens();
        let p_ab = lens.set("B".to_string(), &lens.set("A".to_string(), &p));
        let p_b = lens.set("B".to_string(), &p);
        assert_eq!(p_ab, p_b);
    }
}
