/// 742: Type Witnesses — evidence passing for invariants

// ── Sorted witness ─────────────────────────────────────────────────────────────

/// A `Vec<T>` that is guaranteed to be sorted.
/// The private inner field prevents construction without going through `sort()`.
#[derive(Debug, Clone)]
pub struct Sorted<T>(Vec<T>);

impl<T: Ord + Clone> Sorted<T> {
    /// Only entry point — the act of sorting produces the witness.
    pub fn sort(mut v: Vec<T>) -> Self {
        v.sort();
        Sorted(v)
    }

    /// Merge two sorted vecs — result is still sorted (merge sort merge step).
    pub fn merge(self, other: Sorted<T>) -> Sorted<T> {
        let mut a = self.0.into_iter().peekable();
        let mut b = other.0.into_iter().peekable();
        let mut result = Vec::with_capacity(a.size_hint().0 + b.size_hint().0);
        loop {
            match (a.peek(), b.peek()) {
                (Some(av), Some(bv)) => {
                    if av <= bv { result.push(a.next().unwrap()); }
                    else        { result.push(b.next().unwrap()); }
                }
                (Some(_), None) => { result.extend(a); break; }
                (None, Some(_)) => { result.extend(b); break; }
                (None, None)    => break,
            }
        }
        Sorted(result)   // safe: merge of two sorted = sorted
    }

    /// Binary search — safe to use because we KNOW it's sorted.
    pub fn binary_search(&self, target: &T) -> bool {
        self.0.binary_search(target).is_ok()
    }

    pub fn as_slice(&self) -> &[T] { &self.0 }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

// ── NonZero witness ────────────────────────────────────────────────────────────

/// A `u64` guaranteed to be non-zero.
#[derive(Debug, Clone, Copy)]
pub struct NonZeroU64(u64);

impl NonZeroU64 {
    pub fn new(n: u64) -> Option<Self> {
        if n == 0 { None } else { Some(NonZeroU64(n)) }
    }

    pub fn get(self) -> u64 { self.0 }

    /// Division by non-zero — never panics (no division-by-zero possible).
    pub fn divide(self, dividend: u64) -> u64 {
        dividend / self.0   // safe: self.0 guaranteed ≠ 0
    }
}

// ── Authenticated witness ──────────────────────────────────────────────────────

pub struct Authenticated;
pub struct Unauthenticated;

use std::marker::PhantomData;

pub struct Session<Auth> {
    user_id: u64,
    _auth: PhantomData<Auth>,
}

impl Session<Unauthenticated> {
    pub fn new() -> Self { Session { user_id: 0, _auth: PhantomData } }

    pub fn authenticate(self, user_id: u64, _password: &str) -> Session<Authenticated> {
        // In reality: verify password hash
        Session { user_id, _auth: PhantomData }
    }
}

impl Session<Authenticated> {
    pub fn user_id(&self) -> u64 { self.user_id }
    // Only authenticated sessions can access protected resources
    pub fn access_profile(&self) -> String {
        format!("Profile for user {}", self.user_id)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_witness_is_sorted() {
        let s = Sorted::sort(vec![3, 1, 4, 1, 5, 9]);
        let v = s.as_slice();
        for w in v.windows(2) {
            assert!(w[0] <= w[1], "not sorted: {:?}", v);
        }
    }

    #[test]
    fn binary_search_found() {
        let s = Sorted::sort(vec![1, 2, 3, 4, 5]);
        assert!(s.binary_search(&3));
    }

    #[test]
    fn binary_search_not_found() {
        let s = Sorted::sort(vec![1, 2, 4, 5]);
        assert!(!s.binary_search(&3));
    }

    #[test]
    fn merge_preserves_sorted_invariant() {
        let a = Sorted::sort(vec![1, 3, 5]);
        let b = Sorted::sort(vec![2, 4, 6]);
        let m = a.merge(b);
        let v = m.as_slice();
        for w in v.windows(2) {
            assert!(w[0] <= w[1]);
        }
        assert_eq!(v, &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn non_zero_rejects_zero() {
        assert!(NonZeroU64::new(0).is_none());
        assert!(NonZeroU64::new(1).is_some());
    }

    #[test]
    fn non_zero_divide_no_panic() {
        let d = NonZeroU64::new(7).unwrap();
        assert_eq!(d.divide(49), 7);
    }
}
