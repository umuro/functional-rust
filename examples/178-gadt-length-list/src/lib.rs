// Example 178: Length-Indexed Lists
// Rust uses const generics to track length at compile time

// === Approach 1: Const generic array wrapper ===

#[derive(Debug, Clone)]
struct Vec2<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Vec2<T, N> {
    fn replicate(val: T) -> Self {
        Vec2 { data: [val; N] }
    }
}

impl<T: Copy, const N: usize> Vec2<T, N> {
    fn head(&self) -> T where [(); N - 1]: Sized {
        self.data[0]
    }

    fn map<U: Default + Copy>(&self, f: impl Fn(T) -> U) -> Vec2<U, N> {
        let mut result = [U::default(); N];
        for i in 0..N {
            result[i] = f(self.data[i]);
        }
        Vec2 { data: result }
    }
}

fn zip_vec<T: Copy + Default, U: Copy + Default, const N: usize>(
    a: &Vec2<T, N>,
    b: &Vec2<U, N>,
) -> Vec2<(T, U), N> {
    let mut result = [(T::default(), U::default()); N];
    for i in 0..N {
        result[i] = (a.data[i], b.data[i]);
    }
    Vec2 { data: result }
}

// === Approach 2: Type-level Peano numbers with nested tuples ===

trait Nat {}
struct Zero;
struct Succ<N: Nat>(std::marker::PhantomData<N>);
impl Nat for Zero {}
impl<N: Nat> Nat for Succ<N> {}

trait TypeVec<T> {
    fn to_vec(&self) -> std::vec::Vec<T> where T: Clone;
}

#[derive(Debug)]
struct VNil;

#[derive(Debug)]
struct VCons<T, N: Nat, Rest: TypeVec<T>>(T, Rest, std::marker::PhantomData<N>);

impl<T: Clone> TypeVec<T> for VNil {
    fn to_vec(&self) -> std::vec::Vec<T> { vec![] }
}

impl<T: Clone, N: Nat, Rest: TypeVec<T>> TypeVec<T> for VCons<T, N, Rest> {
    fn to_vec(&self) -> std::vec::Vec<T> {
        let mut v = vec![self.0.clone()];
        v.extend(self.1.to_vec());
        v
    }
}

impl<T, N: Nat, Rest: TypeVec<T>> VCons<T, N, Rest> {
    fn head(&self) -> &T { &self.0 }
    fn tail(&self) -> &Rest { &self.1 }
}

fn vnil() -> VNil { VNil }

fn vcons<T, N: Nat, R: TypeVec<T>>(x: T, rest: R) -> VCons<T, Succ<N>, R>
where R: TypeVec<T>
{
    VCons(x, rest, std::marker::PhantomData)
}

// === Approach 3: Recursive type with compile-time length encoding ===

trait SizedList {
    type Elem;
    const LEN: usize;
    fn to_vec(&self) -> std::vec::Vec<Self::Elem> where Self::Elem: Clone;
}

struct LNil<T>(std::marker::PhantomData<T>);
struct LCons<T, Tail: SizedList<Elem = T>>(T, Tail);

impl<T> SizedList for LNil<T> {
    type Elem = T;
    const LEN: usize = 0;
    fn to_vec(&self) -> std::vec::Vec<T> where T: Clone { vec![] }
}

impl<T, Tail: SizedList<Elem = T>> SizedList for LCons<T, Tail> {
    type Elem = T;
    const LEN: usize = 1 + Tail::LEN;
    fn to_vec(&self) -> std::vec::Vec<T> where T: Clone {
        let mut v = vec![self.0.clone()];
        v.extend(self.1.to_vec());
        v
    }
}

impl<T, Tail: SizedList<Elem = T>> LCons<T, Tail> {
    fn head(&self) -> &T { &self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_generic_head() {
        let v = Vec2 { data: [1, 2, 3] };
        assert_eq!(v.head(), 1);
    }

    #[test]
    fn test_const_generic_map() {
        let v = Vec2 { data: [1, 2, 3] };
        let d = v.map(|x| x * 2);
        assert_eq!(d.data, [2, 4, 6]);
    }

    #[test]
    fn test_const_generic_zip() {
        let a = Vec2 { data: [1, 2] };
        let b = Vec2 { data: [10, 20] };
        let z = zip_vec(&a, &b);
        assert_eq!(z.data, [(1, 10), (2, 20)]);
    }

    #[test]
    fn test_const_generic_replicate() {
        let v: Vec2<i32, 3> = Vec2::replicate(42);
        assert_eq!(v.data, [42, 42, 42]);
    }

    #[test]
    fn test_peano_vec() {
        let pv = vcons(1, vcons(2, vcons::<_, Zero, _>(3, vnil())));
        assert_eq!(*pv.head(), 1);
        assert_eq!(pv.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_recursive_list() {
        let l = LCons(10, LCons(20, LNil(std::marker::PhantomData)));
        assert_eq!(*l.head(), 10);
        assert_eq!(l.to_vec(), vec![10, 20]);
        assert_eq!(<LCons<i32, LCons<i32, LNil<i32>>> as SizedList>::LEN, 2);
    }
}
