//! # 549. PhantomData for Lifetime Variance
//! Using PhantomData to control type variance and signal ownership.

use std::marker::PhantomData;

/// Covariant in 'a: &'long T can be used as &'short T
/// PhantomData<&'a T> makes the struct covariant in 'a
struct CovariantRef<'a, T> {
    ptr: *const T,
    _marker: PhantomData<&'a T>, // covariant in 'a and T
}

impl<'a, T> CovariantRef<'a, T> {
    unsafe fn new(r: &'a T) -> Self {
        CovariantRef { ptr: r as *const T, _marker: PhantomData }
    }

    unsafe fn get(&self) -> &T { &*self.ptr }
}

/// Invariant in T: neither T's subtypes nor supertypes substitute
/// PhantomData<*mut T> is invariant
struct InvariantBox<T> {
    ptr: *mut T,
    _marker: PhantomData<*mut T>, // invariant
}

/// Phantom type state machine — zero runtime cost
struct Locked;
struct Unlocked;

struct Door<State> {
    name: String,
    _state: PhantomData<State>,
}

impl Door<Locked> {
    fn new(name: &str) -> Self {
        Door { name: name.to_string(), _state: PhantomData }
    }

    fn unlock(self) -> Door<Unlocked> {
        println!("Unlocking '{}'", self.name);
        Door { name: self.name, _state: PhantomData }
    }
}

impl Door<Unlocked> {
    fn open(&self) {
        println!("Opening '{}'", self.name);
    }

    fn lock(self) -> Door<Locked> {
        println!("Locking '{}'", self.name);
        Door { name: self.name, _state: PhantomData }
    }
}

/// Validated type — PhantomData as proof of validation
struct Validated<T> {
    inner: T,
    _marker: PhantomData<()>, // could carry validation type
}

impl<T: Clone + std::fmt::Debug> Validated<T> {
    fn try_new(value: T, valid: bool) -> Option<Self> {
        if valid { Some(Validated { inner: value, _marker: PhantomData }) }
        else { None }
    }

    fn get(&self) -> &T { &self.inner }
}

/// Owned pointer with explicit ownership via PhantomData
struct OwnedPtr<T> {
    ptr: *mut T,
    _owned: PhantomData<T>, // signals we OWN this T (affects drop check)
}

impl<T> OwnedPtr<T> {
    fn new(value: T) -> Self {
        OwnedPtr {
            ptr: Box::into_raw(Box::new(value)),
            _owned: PhantomData,
        }
    }

    fn get(&self) -> &T { unsafe { &*self.ptr } }
}

impl<T> Drop for OwnedPtr<T> {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.ptr)); }
    }
}

fn main() {
    // Covariant ref
    let data = 42i32;
    let cref = unsafe { CovariantRef::new(&data) };
    println!("CovariantRef: {}", unsafe { cref.get() });

    // State machine — type safety without runtime cost
    let door = Door::<Locked>::new("front door");
    // door.open(); // ERROR: Door<Locked> has no open() method!
    let door = door.unlock();
    door.open();
    let door = door.lock();
    let door = door.unlock(); // can unlock again
    door.open();

    // Validated type
    let valid = Validated::try_new(42i32, true);
    let invalid = Validated::try_new(0i32, false);
    println!("valid: {:?}", valid.map(|v| *v.get()));
    println!("invalid: {:?}", invalid.map(|v| *v.get()));

    // OwnedPtr
    let p = OwnedPtr::new(String::from("owned data"));
    println!("OwnedPtr: {}", p.get());
    // p drops here — memory freed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covariant_ref() {
        let x = 100i32;
        let r = unsafe { CovariantRef::new(&x) };
        assert_eq!(unsafe { *r.get() }, 100);
    }

    #[test]
    fn test_state_machine() {
        let door = Door::<Locked>::new("test");
        let open_door = door.unlock();
        open_door.open(); // compiles — correct state
        // open_door.unlock(); // Would fail — Door<Unlocked> has no unlock()
    }

    #[test]
    fn test_validated() {
        let v = Validated::try_new("hello", true);
        assert!(v.is_some());
        assert_eq!(*v.unwrap().get(), "hello");
        let invalid = Validated::try_new("x", false);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_owned_ptr() {
        let p = OwnedPtr::new(42i32);
        assert_eq!(*p.get(), 42);
        // No memory leak — Drop impl handles cleanup
    }
}
