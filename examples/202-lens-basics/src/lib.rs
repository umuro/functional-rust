// Example 202: Lens Basics — Lens as a Pair of Get and Set

// === Approach 1: Lens as struct with closures === //

struct Lens<S, A> {
    get: Box<dyn Fn(&S) -> A>,
    set: Box<dyn Fn(A, &S) -> S>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    fn new(
        get: impl Fn(&S) -> A + 'static,
        set: impl Fn(A, &S) -> S + 'static,
    ) -> Self {
        Lens { get: Box::new(get), set: Box::new(set) }
    }

    fn view(&self, s: &S) -> A {
        (self.get)(s)
    }

    fn set(&self, a: A, s: &S) -> S {
        (self.set)(a, s)
    }

    fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        let a = (self.get)(s);
        (self.set)(f(a), s)
    }
}

// === Approach 2: Lens via trait (zero-cost abstraction) === //

trait LensLike<S, A> {
    fn get(s: &S) -> A;
    fn set(a: A, s: &S) -> S;

    fn over(f: impl FnOnce(A) -> A, s: &S) -> S {
        let a = Self::get(s);
        Self::set(f(a), s)
    }
}

// === Approach 3: Macro-generated lenses === //

macro_rules! make_lens {
    ($lens_name:ident, $struct:ty, $field:ident, $field_ty:ty) => {
        struct $lens_name;
        impl LensLike<$struct, $field_ty> for $lens_name {
            fn get(s: &$struct) -> $field_ty { s.$field.clone() }
            fn set(a: $field_ty, s: &$struct) -> $struct {
                let mut new = s.clone();
                new.$field = a;
                new
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Employee {
    emp_name: String,
    address: Address,
}

// Generate lenses via macro
make_lens!(PersonName, Person, name, String);
make_lens!(PersonAge, Person, age, u32);
make_lens!(EmpAddress, Employee, address, Address);
make_lens!(AddrCity, Address, city, String);

fn name_lens() -> Lens<Person, String> {
    Lens::new(|p| p.name.clone(), |n, p| Person { name: n, ..p.clone() })
}

fn age_lens() -> Lens<Person, u32> {
    Lens::new(|p| p.age, |a, p| Person { age: a, ..p.clone() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_lens_get_set() {
        let p = Person { name: "Bob".into(), age: 25 };
        let nl = name_lens();
        assert_eq!(nl.view(&p), "Bob");
        let p2 = nl.set("Robert".into(), &p);
        assert_eq!(nl.view(&p2), "Robert");
        assert_eq!(p2.age, 25); // other fields unchanged
    }

    #[test]
    fn test_trait_lens() {
        let p = Person { name: "Eve".into(), age: 40 };
        assert_eq!(PersonAge::get(&p), 40);
        let p2 = PersonAge::set(41, &p);
        assert_eq!(PersonAge::get(&p2), 41);
    }

    #[test]
    fn test_over_modify() {
        let p = Person { name: "X".into(), age: 10 };
        let p2 = PersonAge::over(|a| a * 2, &p);
        assert_eq!(PersonAge::get(&p2), 20);
    }

    #[test]
    fn test_macro_lens_for_nested() {
        let emp = Employee {
            emp_name: "Charlie".into(),
            address: Address { street: "1st".into(), city: "NYC".into(), zip: "10001".into() },
        };
        let addr = EmpAddress::get(&emp);
        assert_eq!(AddrCity::get(&addr), "NYC");
    }
}
