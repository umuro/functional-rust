#![allow(clippy::all)]
/// An isomorphism: a lossless, bidirectional transformation between types S and A.
///
/// Laws:
///   get(reverse_get(a)) == a   (right identity)
///   reverse_get(get(s)) == s   (left identity)
pub struct Iso<S, A> {
    pub get: Box<dyn Fn(&S) -> A>,
    pub reverse_get: Box<dyn Fn(&A) -> S>,
}

impl<S: 'static, A: 'static> Iso<S, A> {
    pub fn new(get: impl Fn(&S) -> A + 'static, reverse_get: impl Fn(&A) -> S + 'static) -> Self {
        Iso {
            get: Box::new(get),
            reverse_get: Box::new(reverse_get),
        }
    }

    /// Swap directions: the inverse Iso<A, S>.
    pub fn reverse(self) -> Iso<A, S> {
        Iso {
            get: self.reverse_get,
            reverse_get: self.get,
        }
    }

    /// Compose two isos: Iso<S,A> then Iso<A,B> → Iso<S,B>.
    pub fn compose<B: 'static>(self, other: Iso<A, B>) -> Iso<S, B>
    where
        A: 'static,
    {
        let get_self = self.get;
        let rev_self = self.reverse_get;
        let get_other = other.get;
        let rev_other = other.reverse_get;

        Iso {
            get: Box::new(move |s| get_other(&get_self(s))),
            reverse_get: Box::new(move |b| rev_self(&rev_other(b))),
        }
    }

    /// Apply `get`.
    pub fn get(&self, s: &S) -> A {
        (self.get)(s)
    }

    /// Apply `reverse_get`.
    pub fn reverse_get(&self, a: &A) -> S {
        (self.reverse_get)(a)
    }
}

// ---------------------------------------------------------------------------
// Approach 1: numeric unit conversions
// ---------------------------------------------------------------------------

/// Celsius ↔ Fahrenheit
pub fn celsius_fahrenheit() -> Iso<f64, f64> {
    Iso::new(|c| c * 9.0 / 5.0 + 32.0, |f| (f - 32.0) * 5.0 / 9.0)
}

/// Meters ↔ Kilometers
pub fn meters_kilometers() -> Iso<f64, f64> {
    Iso::new(|m| m / 1000.0, |km| km * 1000.0)
}

// ---------------------------------------------------------------------------
// Approach 2: newtype wrappers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, PartialEq)]
pub struct Fahrenheit(pub f64);

/// Celsius newtype ↔ raw f64
pub fn celsius_raw() -> Iso<Celsius, f64> {
    Iso::new(|c: &Celsius| c.0, |f: &f64| Celsius(*f))
}

/// Fahrenheit newtype ↔ raw f64
pub fn fahrenheit_raw() -> Iso<Fahrenheit, f64> {
    Iso::new(|f: &Fahrenheit| f.0, |v: &f64| Fahrenheit(*v))
}

// ---------------------------------------------------------------------------
// Approach 3: String ↔ Vec<char> (structural Iso)
// ---------------------------------------------------------------------------

pub fn string_chars() -> Iso<String, Vec<char>> {
    Iso::new(
        |s: &String| s.chars().collect(),
        |cs: &Vec<char>| cs.iter().collect(),
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    // --- celsius_fahrenheit roundtrip ---

    #[test]
    fn test_celsius_to_fahrenheit() {
        let iso = celsius_fahrenheit();
        assert!(approx_eq(iso.get(&0.0), 32.0));
        assert!(approx_eq(iso.get(&100.0), 212.0));
        assert!(approx_eq(iso.get(&-40.0), -40.0));
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let iso = celsius_fahrenheit();
        assert!(approx_eq(iso.reverse_get(&32.0), 0.0));
        assert!(approx_eq(iso.reverse_get(&212.0), 100.0));
    }

    #[test]
    fn test_celsius_fahrenheit_roundtrip() {
        let iso = celsius_fahrenheit();
        let start = 37.0_f64;
        // forward then back
        assert!(approx_eq(iso.reverse_get(&iso.get(&start)), start));
        // back then forward
        let f = 98.6_f64;
        assert!(approx_eq(iso.get(&iso.reverse_get(&f)), f));
    }

    // --- reverse swaps directions ---

    #[test]
    fn test_reverse_swaps_directions() {
        let iso = celsius_fahrenheit().reverse(); // now Iso<f64, f64> but F→C
        assert!(approx_eq(iso.get(&32.0), 0.0));
        assert!(approx_eq(iso.reverse_get(&0.0), 32.0));
    }

    // --- compose chains two isos ---

    #[test]
    fn test_compose_meters_to_kilometers_then_to_string() {
        // Compose Iso<f64,f64> (m→km) with a string iso for demonstration.
        // We use a simple km→String / String→km numeric iso.
        let m_to_km = meters_kilometers();
        let km_to_string: Iso<f64, String> = Iso::new(
            |km: &f64| format!("{km:.3}"),
            |s: &String| s.parse::<f64>().unwrap_or(0.0),
        );
        let composed = m_to_km.compose(km_to_string);
        // 1500 m → 1.5 km → "1.500"
        assert_eq!(composed.get(&1500.0), "1.500");
        // "1.500" → 1.5 km → 1500 m
        assert!(approx_eq(
            composed.reverse_get(&"1.500".to_string()),
            1500.0
        ));
    }

    // --- string_chars roundtrip ---

    #[test]
    fn test_string_chars_roundtrip() {
        let iso = string_chars();
        let s = "hello".to_string();
        assert_eq!(iso.reverse_get(&iso.get(&s)), s);

        let chars = vec!['r', 'u', 's', 't'];
        assert_eq!(iso.get(&iso.reverse_get(&chars)), chars);
    }

    #[test]
    fn test_string_chars_empty() {
        let iso = string_chars();
        let empty = String::new();
        assert_eq!(iso.get(&empty), vec![]);
        assert_eq!(iso.reverse_get(&vec![]), empty);
    }

    // --- newtype wrappers ---

    #[test]
    fn test_celsius_newtype_roundtrip() {
        let iso = celsius_raw();
        let c = Celsius(37.5);
        assert!(approx_eq(iso.get(&c), 37.5));
        assert_eq!(iso.reverse_get(&37.5), Celsius(37.5));
        // full roundtrip
        assert_eq!(iso.reverse_get(&iso.get(&c)), c);
    }

    // --- Iso laws: get ∘ reverse_get = id and reverse_get ∘ get = id ---

    #[test]
    fn test_iso_laws_meters_kilometers() {
        let iso = meters_kilometers();
        let m = 42_000.0_f64;
        assert!(approx_eq(iso.reverse_get(&iso.get(&m)), m));
        let km = 42.0_f64;
        assert!(approx_eq(iso.get(&iso.reverse_get(&km)), km));
    }
}
