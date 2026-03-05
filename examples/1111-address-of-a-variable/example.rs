/// Returns the memory address of any value as a `usize`.
///
/// Uses `std::ptr::addr_of!` to avoid creating an intermediate reference,
/// which is safe even for packed/unaligned fields.
pub fn address_of<T>(x: &T) -> usize {
    std::ptr::addr_of!(*x) as usize
}

/// Returns true when two references point to distinct memory locations.
pub fn addresses_differ<T>(a: &T, b: &T) -> bool {
    address_of(a) != address_of(b)
}

fn main() {
    let a: f64 = 3.14;
    println!("address of a (f64 on stack) : 0x{:x}", address_of(&a));

    let b: i32 = 42;
    println!("address of b (i32 on stack) : 0x{:x}", address_of(&b));

    let c: Box<i32> = Box::new(99);
    println!("address of *c (i32 on heap) : 0x{:x}", address_of(&*c));

    let x: i32 = 1;
    let y: i32 = 2;
    println!(
        "x and y have different addresses: {}",
        addresses_differ(&x, &y)
    );

    // In OCaml, unboxed integers (plain `int`) have no heap address —
    // Obj.repr returns an immediate value, not a pointer.
    // In Rust, every value — including plain i32 — has a real memory address.
    let unboxed: i32 = 17;
    println!(
        "address of unboxed i32 (17)    : 0x{:x}  ← works in Rust, would error in OCaml",
        address_of(&unboxed)
    );
}

/* Output (addresses are illustrative; actual values vary per run):
   address of a (f64 on stack) : 0x7ffd3a1b2c10
   address of b (i32 on stack) : 0x7ffd3a1b2c0c
   address of *c (i32 on heap) : 0x556b8e1d7b60
   x and y have different addresses: true
   address of unboxed i32 (17) : 0x7ffd3a1b2c08  ← works in Rust, would error in OCaml
*/
