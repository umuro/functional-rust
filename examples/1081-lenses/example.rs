/// A lens is a pair of getter/setter functions that focus on a part of a larger structure.
/// This is the functional approach to accessing and modifying nested data without mutation.

// Type aliases for lens closure types
type Getter<S, A> = Box<dyn Fn(&S) -> &A>;
type Setter<S, A> = Box<dyn Fn(A, &S) -> S>;

struct Lens<S, A> {
    get_fn: Getter<S, A>,
    set_fn: Setter<S, A>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    fn new(get: impl Fn(&S) -> &A + 'static, set: impl Fn(A, &S) -> S + 'static) -> Self {
        Lens {
            get_fn: Box::new(get),
            set_fn: Box::new(set),
        }
    }

    fn get<'s>(&self, whole: &'s S) -> &'s A {
        (self.get_fn)(whole)
    }

    fn set(&self, value: A, whole: &S) -> S {
        (self.set_fn)(value, whole)
    }

    fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where
        S: Clone,
        A: Clone,
    {
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
                let mid: &A = (get_outer)(s);
                let mid_ptr: *const A = mid;
                (get_inner.get_fn)(unsafe { &*mid_ptr })
            },
            move |b: B, s: &S| {
                let mid: &A = (set_outer_get)(s);
                let new_mid: A = (set_inner.set_fn)(b, mid);
                (set_outer_set)(new_mid, s)
            },
        )
    }
}

/// Apply a function to the focused part of a lens, returning an updated whole.
fn over<S: 'static, A: Clone + 'static>(
    lens: &Lens<S, A>,
    f: impl FnOnce(A) -> A,
    whole: &S,
) -> S {
    let current = lens.get(whole).clone();
    lens.set(f(current), whole)
}

// --- Domain types ---

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
        |p: &Person| &p.addr,
        |a: Address, p: &Person| Person {
            name: p.name.clone(),
            addr: a,
        },
    )
}

fn city_lens() -> Lens<Address, String> {
    Lens::new(
        |a: &Address| &a.city,
        |c: String, a: &Address| Address {
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

    let p2 = over(&lens, |c| c.to_uppercase(), &p);
    println!("City after over(uppercase): {}", lens.get(&p2));

    let p3 = lens.set("Boston".to_string(), &p);
    println!("City after set: {}", lens.get(&p3));
    println!("Original unchanged: {}", lens.get(&p));
}

/* Output:
   City: NYC
   City after over(uppercase): NYC
   City after set: Boston
   Original unchanged: NYC
*/
