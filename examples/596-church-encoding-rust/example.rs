// Church Booleans via closures
// In Rust we need concrete types; we use i32 as the "result type"
fn church_true(t: i32, _f: i32) -> i32  { t }
fn church_false(_t: i32, f: i32) -> i32 { f }

fn church_if(c: fn(i32,i32)->i32, t: i32, f: i32) -> i32 { c(t, f) }
fn church_not(b: fn(i32,i32)->i32) -> fn(i32,i32)->i32 {
    if b(1, 0) == 1 { church_false } else { church_true }
}
fn church_and(p: fn(i32,i32)->i32, q: fn(i32,i32)->i32) -> bool {
    p(1,0) == 1 && q(1,0) == 1
}

// Church Numerals (represented as usize via application)
fn to_int(church_n: impl Fn(fn(usize)->usize, usize) -> usize) -> usize {
    church_n(|x| x+1, 0)
}

fn zero(f: fn(usize)->usize, x: usize) -> usize { let _ = f; x }
fn one (f: fn(usize)->usize, x: usize) -> usize { f(x) }
fn two (f: fn(usize)->usize, x: usize) -> usize { f(f(x)) }
fn three(f: fn(usize)->usize, x: usize) -> usize { f(f(f(x))) }

fn church_add(m: fn(fn(usize)->usize,usize)->usize,
              n: fn(fn(usize)->usize,usize)->usize)
    -> impl Fn(fn(usize)->usize, usize) -> usize
{
    move |f, x| m(f, n(f, x))
}

fn church_mul(m: fn(fn(usize)->usize,usize)->usize,
              n: fn(fn(usize)->usize,usize)->usize)
    -> impl Fn(fn(usize)->usize, usize) -> usize
{
    move |f, x| m(|y| n(f, y), x)
}

// Church Pairs
fn church_pair<A: Copy, B: Copy>(a: A, b: B) -> impl Fn(fn(A,B)->A)->A + Copy {
    move |f| f(a, b)
}

fn main() {
    println!("2+3 = {}", to_int(church_add(two, three)));
    println!("2*3 = {}", to_int(church_mul(two, three)));
    println!("if True then 1 else 0 = {}", church_if(church_true, 1, 0));
    println!("if False then 1 else 0 = {}", church_if(church_false, 1, 0));
    println!("not True = {}", church_if(church_not(church_true), 1, 0));
    println!("True AND False = {}", church_and(church_true, church_false));
    println!("True AND True  = {}", church_and(church_true, church_true));
    println!("zero={} one={} two={} three={}",
        to_int(zero), to_int(one), to_int(two), to_int(three));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn add_church() { assert_eq!(to_int(church_add(two,three)), 5); }
    #[test] fn mul_church() { assert_eq!(to_int(church_mul(two,three)), 6); }
    #[test] fn bool_if()    { assert_eq!(church_if(church_true,42,0), 42); }
}
