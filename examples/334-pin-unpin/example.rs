use std::marker::PhantomPinned;
use std::pin::Pin;

struct SelfRef {
    data: String,
    ptr: *const u8,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn new(s: &str) -> Pin<Box<Self>> {
        let mut b = Box::new(Self { data: s.to_string(), ptr: std::ptr::null(), _pin: PhantomPinned });
        b.ptr = b.data.as_ptr();
        unsafe { Pin::new_unchecked(b) }
    }
    fn get_data(self: Pin<&Self>) -> &str { &self.data }
    fn ptr_valid(self: Pin<&Self>) -> bool { self.ptr == self.data.as_ptr() }
}

#[derive(Debug)]
struct Normal { x: i32 }

fn main() {
    let sr = SelfRef::new("hello");
    println!("Data: {}", sr.as_ref().get_data());
    println!("Ptr valid: {}", sr.as_ref().ptr_valid());

    let mut n = Normal { x: 42 };
    let p = Pin::new(&mut n);
    let inner = Pin::into_inner(p); // fine: Normal: Unpin
    println!("Normal: {inner:?}");

    let mut v = 99i32;
    let pv = Pin::new(&mut v);
    println!("Pinned: {}", *pv);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn self_ref_ptr_valid() { let s = SelfRef::new("test"); assert!(s.as_ref().ptr_valid()); }
    #[test] fn self_ref_data() { let s = SelfRef::new("hello"); assert_eq!(s.as_ref().get_data(), "hello"); }
    #[test] fn normal_is_unpin() { fn chk<T:Unpin>(){} chk::<Normal>(); chk::<i32>(); chk::<String>(); }
}
