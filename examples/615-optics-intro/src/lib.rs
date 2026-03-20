#![allow(clippy::all)]
//! # Optics Introduction
//! Composable accessors for nested data.

pub struct Lens<S, A> {
    pub get: Box<dyn Fn(&S) -> A>,
    pub set: Box<dyn Fn(&S, A) -> S>,
}

impl<S: Clone + 'static, A: Clone + 'static> Lens<S, A> {
    pub fn new(get: impl Fn(&S) -> A + 'static, set: impl Fn(&S, A) -> S + 'static) -> Self {
        Lens {
            get: Box::new(get),
            set: Box::new(set),
        }
    }
    pub fn view(&self, s: &S) -> A {
        (self.get)(s)
    }
    pub fn over(&self, s: &S, f: impl Fn(A) -> A) -> S {
        let a = (self.get)(s);
        (self.set)(s, f(a))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn name_lens() -> Lens<Person, String> {
    Lens::new(
        |p: &Person| p.name.clone(),
        |p: &Person, n| Person {
            name: n,
            ..p.clone()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lens() {
        let p = Person {
            name: "Alice".into(),
            age: 30,
        };
        let lens = name_lens();
        assert_eq!(lens.view(&p), "Alice");
        let p2 = lens.over(&p, |n| n.to_uppercase());
        assert_eq!(p2.name, "ALICE");
    }
}
