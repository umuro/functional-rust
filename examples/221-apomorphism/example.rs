// example.rs content
use example_221_apomorphism::{cons, from_vec, insert, nil, replace_first, take, to_vec};

fn main() {
    // ── insert into a sorted list ────────────────────────────────────────────
    let sorted = cons(1, cons(3, cons(5, nil())));

    println!("sorted          : {:?}", to_vec(&sorted));
    println!("insert 2        : {:?}", to_vec(&insert(2, sorted.clone())));
    println!("insert 0 (front): {:?}", to_vec(&insert(0, sorted.clone())));
    println!("insert 6 (back) : {:?}", to_vec(&insert(6, sorted.clone())));

    // ── take ─────────────────────────────────────────────────────────────────
    let xs = from_vec(&[1, 2, 3, 4, 5]);

    println!("\nxs              : {:?}", to_vec(&xs));
    println!("take 3          : {:?}", to_vec(&take(3, xs.clone())));
    println!("take 0          : {:?}", to_vec(&take(0, xs.clone())));
    println!("take 10         : {:?}", to_vec(&take(10, xs.clone())));

    // ── replace first occurrence ─────────────────────────────────────────────
    let ys = from_vec(&[1, 2, 3, 2]);

    println!("\nys              : {:?}", to_vec(&ys));
    println!(
        "replace_first 2→99: {:?}",
        to_vec(&replace_first(2, 99, ys.clone()))
    );
    println!(
        "replace_first 9→0 : {:?}",
        to_vec(&replace_first(9, 0, ys))
    );
}