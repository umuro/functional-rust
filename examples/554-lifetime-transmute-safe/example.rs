//! # 554. Safe Transmute with Lifetimes
//! When and how to safely transmute lifetime parameters.

/// UNSAFE: extend a lifetime (dangerous — only safe if you guarantee validity)
/// This is the canonical example of what NOT to do:
unsafe fn extend_lifetime_unsafe<'short, 'long, T>(
    r: &'short T
) -> &'long T {
    std::mem::transmute(r)
}

/// SAFE PATTERN: lifetime extension with proven invariants
/// The `'static` item truly is static, but the type system needs help
fn get_static_str(s: &'static str) -> &'static str {
    s // no transmute needed — already static
}

/// Safe slice from raw parts — documented invariants
fn safe_split(data: &[i32], mid: usize) -> (&[i32], &[i32]) {
    assert!(mid <= data.len());
    // This is what split_at does internally via unsafe:
    unsafe {
        let ptr = data.as_ptr();
        let left  = std::slice::from_raw_parts(ptr, mid);
        let right = std::slice::from_raw_parts(ptr.add(mid), data.len() - mid);
        (left, right)
    }
}

/// Erase lifetime for a truly owned value (common pattern in custom allocators)
struct OwnedData {
    data: Vec<u8>,
}

impl OwnedData {
    fn new(data: Vec<u8>) -> Self { OwnedData { data } }

    /// SAFETY: returned slice is valid as long as OwnedData lives
    /// We use unsafe to express the lifetime relationship
    fn as_slice(&self) -> &[u8] {
        &self.data // safe — normal borrow
    }

    /// Imaginary "reset with new data" that reuses the buffer
    fn reset(&mut self, new_data: &[u8]) {
        self.data.clear();
        self.data.extend_from_slice(new_data);
    }
}

/// Demonstrate why lifetime transmute is dangerous
fn danger_demo() {
    // DO NOT DO THIS:
    // let dangling: &'static str = unsafe {
    //     let local = String::from("I will be dropped");
    //     extend_lifetime_unsafe(&local)
    // }; // local dropped here — dangling pointer!
    // println!("{}", dangling); // USE-AFTER-FREE!

    // DO THIS INSTEAD:
    let owned = String::from("safe owned data");
    let borrowed: &str = &owned;
    println!("Safe borrow: {}", borrowed);
    // borrowed only valid while owned lives — borrow checker enforces this
}

/// Safe conversion patterns (alternatives to transmute)
fn safe_conversions() {
    // From/Into for type-safe conversions
    let s: String = String::from("hello");
    let bytes: Vec<u8> = s.into_bytes(); // safe, no transmute
    let s2 = String::from_utf8(bytes).unwrap(); // safe conversion back
    println!("round-trip: {}", s2);

    // as for numeric casts (safe for widening, truncating for narrowing)
    let x: i32 = 1000;
    let y: u8 = x as u8; // truncates — documented behavior
    println!("i32 {} as u8 = {}", x, y);

    // bytemuck for safe type punning (crate, not std)
    // Without bytemuck: just use proper API
    let float: f32 = 1.5;
    let bits: u32 = float.to_bits(); // safe, explicit
    let back: f32 = f32::from_bits(bits);
    println!("f32 bits: {}, back: {}", bits, back);
}

fn main() {
    println!("=== Danger demo (conceptual) ===");
    danger_demo();

    println!("\n=== Safe split ===");
    let data = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = safe_split(&data, 3);
    println!("left: {:?}, right: {:?}", left, right);

    println!("\n=== OwnedData ===");
    let mut owned = OwnedData::new(b"hello world".to_vec());
    println!("slice: {:?}", std::str::from_utf8(owned.as_slice()).unwrap());
    owned.reset(b"new data");
    println!("after reset: {:?}", std::str::from_utf8(owned.as_slice()).unwrap());

    println!("\n=== Safe conversions ===");
    safe_conversions();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_split() {
        let v = vec![1, 2, 3, 4, 5];
        let (l, r) = safe_split(&v, 2);
        assert_eq!(l, &[1, 2]);
        assert_eq!(r, &[3, 4, 5]);
    }

    #[test]
    fn test_owned_data() {
        let mut d = OwnedData::new(b"hello".to_vec());
        assert_eq!(d.as_slice(), b"hello");
        d.reset(b"world");
        assert_eq!(d.as_slice(), b"world");
    }

    #[test]
    fn test_f32_bits_roundtrip() {
        let f = std::f32::consts::PI;
        let bits = f.to_bits();
        assert_eq!(f32::from_bits(bits), f);
    }
}
