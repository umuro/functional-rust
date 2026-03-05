// Example 204: Lens Composition — Zoom Into Nested Structs

type GetFn<S, A> = Box<dyn Fn(&S) -> A>;
type SetFn<S, A> = Box<dyn Fn(A, &S) -> S>;

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

    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where
        A: Clone,
    {
        use std::rc::Rc;
        let outer_get = Rc::new(self.get);
        let outer_get2 = Rc::clone(&outer_get);
        let outer_set = self.set;
        let inner_get = inner.get;
        let inner_set = inner.set;
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

fn person_address_lens() -> Lens<Person, Address> {
    Lens::new(
        |p: &Person| p.address.clone(),
        |a: Address, p: &Person| Person { address: a, ..p.clone() },
    )
}

fn address_street_lens() -> Lens<Address, Street> {
    Lens::new(
        |a: &Address| a.street.clone(),
        |s: Street, a: &Address| Address { street: s, ..a.clone() },
    )
}

fn street_number_lens() -> Lens<Street, u32> {
    Lens::new(
        |s: &Street| s.number,
        |n: u32, s: &Street| Street { number: n, ..s.clone() },
    )
}

fn main() {
    let alice = Person {
        pname: "Alice".into(),
        address: Address {
            city: "Wonderland".into(),
            street: Street { number: 42, name: "Main St".into() },
        },
    };

    // Two-level composition: Person → Street
    let person_street = person_address_lens().compose(address_street_lens());
    let street = (person_street.get)(&alice);
    println!("Street: {:?}", street);

    // Three-level composition: Person → street number
    let person_number = person_address_lens()
        .compose(address_street_lens())
        .compose(street_number_lens());

    println!("Street number: {}", (person_number.get)(&alice));

    let updated = (person_number.set)(99, &alice);
    println!("After set(99): street number = {}", updated.address.street.number);
    println!("Name unchanged: {}", updated.pname);
}

/* Output:
   Street: Street { number: 42, name: "Main St" }
   Street number: 42
   After set(99): street number = 99
   Name unchanged: Alice
*/
