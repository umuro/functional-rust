//! # Phantom Type Basics
//! PhantomData for type-level information

use std::marker::PhantomData;

/// Wrapper with phantom type parameter
pub struct Tagged<T, Tag> {
    pub value: T,
    _tag: PhantomData<Tag>,
}

impl<T, Tag> Tagged<T, Tag> {
    pub fn new(value: T) -> Self { Tagged { value, _tag: PhantomData } }
    pub fn into_inner(self) -> T { self.value }
}

/// Type-level markers
pub struct Validated;
pub struct Unvalidated;

/// ID that tracks validation status
pub struct UserId<State>(u64, PhantomData<State>);

impl UserId<Unvalidated> {
    pub fn new(id: u64) -> Self { UserId(id, PhantomData) }
    pub fn validate(self) -> Option<UserId<Validated>> {
        if self.0 > 0 { Some(UserId(self.0, PhantomData)) } else { None }
    }
}

impl UserId<Validated> {
    pub fn get(&self) -> u64 { self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tagged() {
        struct MyTag;
        let t: Tagged<i32, MyTag> = Tagged::new(42);
        assert_eq!(t.into_inner(), 42);
    }
    #[test]
    fn test_validated() {
        let id = UserId::new(123);
        let validated = id.validate().unwrap();
        assert_eq!(validated.get(), 123);
    }
}
