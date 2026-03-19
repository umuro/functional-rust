#![allow(clippy::all)]
/// # Church Numerals — Functions as Numbers
///
/// Lambda calculus encoding of natural numbers using higher-order functions.
/// A Church numeral N is a function that applies `f` N times to `x`.
///
/// In Rust, we use `Box<dyn Fn>` for heap-allocated closures since
/// Rust closures have unique unnameable types (unlike OCaml's uniform representation).

/// Type alias for Church numerals: takes a function and a value, applies f n times.
type Church = Box<dyn Fn(Box<dyn Fn(i64) -> i64>) -> Box<dyn Fn(i64) -> i64>>;

/// Zero: apply f zero times (return x unchanged)
pub fn zero() -> Church {
    Box::new(|_f| Box::new(|x| x))
}

/// One: apply f once
pub fn one() -> Church {
    Box::new(|f| Box::new(move |x| f(x)))
}

/// Successor: given n, apply f one more time
pub fn succ(n: Church) -> Church {
    Box::new(move |f: Box<dyn Fn(i64) -> i64>| {
        // We need to share f between n and the extra application
        // This is tricky in Rust due to ownership — use Rc
        use std::rc::Rc;
        let f = Rc::new(f);
        let f2 = f.clone();
        let inner = n(Box::new(move |x| f2(x)));
        let f3 = f.clone();
        Box::new(move |x| f3(inner(x)))
    })
}

/// Add: m + n = apply f m times, then n times
pub fn add(m: Church, n: Church) -> Church {
    Box::new(move |f: Box<dyn Fn(i64) -> i64>| {
        use std::rc::Rc;
        let f = Rc::new(f);
        let f2 = f.clone();
        let inner_m = m(Box::new(move |x| f(x)));
        let inner_n = n(Box::new(move |x| f2(x)));
        Box::new(move |x| inner_m(inner_n(x)))
    })
}

/// Convert Church numeral to integer
pub fn to_int(n: Church) -> i64 {
    let f: Box<dyn Fn(i64) -> i64> = Box::new(|x| x + 1);
    n(f)(0)
}

/// A simpler approach using a concrete recursive type instead of closures
#[derive(Clone, Debug)]
pub enum ChurchNum {
    Zero,
    Succ(Box<ChurchNum>),
}

impl ChurchNum {
    pub fn to_int(&self) -> i64 {
        match self {
            ChurchNum::Zero => 0,
            ChurchNum::Succ(n) => 1 + n.to_int(),
        }
    }

    pub fn from_int(n: i64) -> Self {
        if n <= 0 {
            ChurchNum::Zero
        } else {
            ChurchNum::Succ(Box::new(ChurchNum::from_int(n - 1)))
        }
    }

    pub fn add(self, other: Self) -> Self {
        match self {
            ChurchNum::Zero => other,
            ChurchNum::Succ(n) => ChurchNum::Succ(Box::new(n.add(other))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(to_int(zero()), 0);
    }

    #[test]
    fn test_one() {
        assert_eq!(to_int(one()), 1);
    }

    #[test]
    fn test_succ() {
        assert_eq!(to_int(succ(zero())), 1);
        assert_eq!(to_int(succ(one())), 2);
    }

    #[test]
    fn test_add() {
        let two = succ(one());
        let three = add(one(), two);
        assert_eq!(to_int(three), 3);
    }

    #[test]
    fn test_church_num_adt() {
        let three = ChurchNum::from_int(3);
        let four = ChurchNum::from_int(4);
        assert_eq!(three.add(four).to_int(), 7);
    }

    #[test]
    fn test_church_num_zero() {
        assert_eq!(ChurchNum::Zero.to_int(), 0);
    }
}
