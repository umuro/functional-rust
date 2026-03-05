//! # Profunctor Optics
//! Optics as profunctor transformations.

pub trait Profunctor<A, B, C, D> {
    fn dimap(self, f: impl Fn(C) -> A, g: impl Fn(B) -> D) -> Self where Self: Sized;
}

impl<A, B, C, D, F: Fn(A) -> B> Profunctor<A, B, C, D> for F {
    fn dimap(self, _f: impl Fn(C) -> A, _g: impl Fn(B) -> D) -> Self { self }
}

#[cfg(test)]
mod tests {
    #[test] fn placeholder() { assert!(true); }
}
