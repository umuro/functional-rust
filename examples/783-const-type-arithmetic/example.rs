// 783. Type-Level Arithmetic with const Generics
// Concat, split, unit-typed quantities — arithmetic in the type system

// ── Array concatenation with type-level sum ────────────────────────────────────

/// Concatenate two fixed-size arrays — output length is A+B in the type
pub fn concat<T: Copy + Default, const A: usize, const B: usize>(
    a: [T; A],
    b: [T; B],
) -> [T; { A + B }] {
    let mut out = [T::default(); { A + B }];
    out[..A].copy_from_slice(&a);
    out[A..].copy_from_slice(&b);
    out
}

/// Split a fixed-size array into two parts — sizes must sum to N
pub fn split<T: Copy + Default, const N: usize, const A: usize>(
    arr: [T; N],
) -> ([T; A], [T; { N - A }])
where
    [(); N - A]: Sized,
{
    let mut left  = [T::default(); A];
    let mut right = [T::default(); { N - A }];
    left.copy_from_slice(&arr[..A]);
    right.copy_from_slice(&arr[A..]);
    (left, right)
}

/// Interleave two equal-length arrays → double length
pub fn interleave<T: Copy + Default, const N: usize>(
    a: [T; N],
    b: [T; N],
) -> [T; { N * 2 }] {
    let mut out = [T::default(); { N * 2 }];
    for i in 0..N {
        out[2 * i]     = a[i];
        out[2 * i + 1] = b[i];
    }
    out
}

// ── Unit-typed quantities ─────────────────────────────────────────────────────

/// Meters<N> — carry the number of meters in the type (const unit tracking)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters<const N: usize>;

impl<const N: usize> Meters<N> {
    pub fn value() -> usize { N }
}

/// Type-level addition of Meters
pub fn add_meters<const A: usize, const B: usize>(_a: Meters<A>, _b: Meters<B>) -> Meters<{ A + B }> {
    Meters
}

// ── Repeated application: [T; N*K] from [T; N] ────────────────────────────────

pub fn repeat_array<T: Copy + Default, const N: usize, const K: usize>(
    arr: [T; N],
) -> [T; { N * K }] {
    let mut out = [T::default(); { N * K }];
    for k in 0..K {
        out[k*N..(k+1)*N].copy_from_slice(&arr);
    }
    out
}

// ── Length-indexed list operations ────────────────────────────────────────────

/// zip two arrays of the same length → array of pairs
pub fn zip<A: Copy + Default, B: Copy + Default, const N: usize>(
    a: [A; N],
    b: [B; N],
) -> [(A, B); N]
where
    (A, B): Default,
{
    std::array::from_fn(|i| (a[i], b[i]))
}

fn main() {
    // Concat: type knows the length
    let a = [1, 2, 3];
    let b = [4, 5];
    let c = concat(a, b);
    println!("concat([1,2,3],[4,5]) = {c:?}");
    // c has type [i32; 5] — enforced by compiler

    // Split
    let (left, right): ([i32; 2], [i32; 3]) = split(c);
    println!("split at 2: left={left:?}, right={right:?}");

    // Interleave
    let odds  = [1, 3, 5];
    let evens = [2, 4, 6];
    let merged = interleave(odds, evens);
    println!("interleave = {merged:?}"); // [1,2,3,4,5,6]

    // Unit-typed meters
    let m5  = Meters::<5>;
    let m3  = Meters::<3>;
    let m8  = add_meters(m5, m3);
    println!("5m + 3m = {}m", Meters::value::<{ 5 + 3 }>());
    // m8 has type Meters<8> — wrong dimensions would not compile

    // Repeat
    let base = [0u8, 1, 2];
    let repeated = repeat_array::<_, 3, 3>(base);
    println!("repeat 3×3 = {repeated:?}");

    // Zip
    let zipped = zip([1, 2, 3], ['a', 'b', 'c']);
    println!("zip = {zipped:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_length() {
        let c = concat([1, 2, 3], [4, 5]);
        assert_eq!(c.len(), 5);
        assert_eq!(c, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn split_correct() {
        let (l, r): ([i32; 2], [i32; 3]) = split([1, 2, 3, 4, 5]);
        assert_eq!(l, [1, 2]);
        assert_eq!(r, [3, 4, 5]);
    }

    #[test]
    fn interleave_correct() {
        assert_eq!(interleave([1, 3], [2, 4]), [1, 2, 3, 4]);
    }

    #[test]
    fn repeat_correct() {
        let r = repeat_array::<i32, 2, 3>([1, 2]);
        assert_eq!(r, [1, 2, 1, 2, 1, 2]);
    }

    #[test]
    fn zip_correct() {
        let z = zip([1, 2], ['a', 'b']);
        assert_eq!(z, [(1, 'a'), (2, 'b')]);
    }
}
