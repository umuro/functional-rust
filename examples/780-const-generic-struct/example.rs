// 780. Generic Structs Parameterised by const
// Matrix<R, C>: dimensions checked at compile time

// ── Matrix ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn zero() -> Self { Self { data: [[0.0; C]; R] } }

    pub fn get(&self, r: usize, c: usize) -> f64 { self.data[r][c] }
    pub fn set(&mut self, r: usize, c: usize, v: f64) { self.data[r][c] = v; }
    pub fn rows() -> usize { R }
    pub fn cols() -> usize { C }
}

/// Identity matrix — only valid when R == C
impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut m = Self::zero();
        for i in 0..N { m.data[i][i] = 1.0; }
        m
    }

    /// Determinant of 2×2 matrix
    pub fn trace(&self) -> f64 {
        (0..N).map(|i| self.data[i][i]).sum()
    }
}

impl<const N: usize> Default for Matrix<N, N> {
    fn default() -> Self { Self::identity() }
}

/// Matrix multiplication: Matrix<R,N> × Matrix<N,C> → Matrix<R,C>
/// The inner dimension N must match — enforced at compile time!
pub fn mat_mul<const R: usize, const N: usize, const C: usize>(
    a: &Matrix<R, N>,
    b: &Matrix<N, C>,
) -> Matrix<R, C> {
    let mut out = Matrix::<R, C>::zero();
    for r in 0..R {
        for c in 0..C {
            let mut sum = 0.0;
            for k in 0..N { sum += a.data[r][k] * b.data[k][c]; }
            out.data[r][c] = sum;
        }
    }
    out
}

impl<const R: usize, const C: usize> std::fmt::Display for Matrix<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            write!(f, "[")?;
            for (j, v) in row.iter().enumerate() {
                if j > 0 { write!(f, ", ")?; }
                write!(f, "{v:7.2}")?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

// ── Statically-typed state machine ────────────────────────────────────────────

/// Transition table stored as a const-generic array
pub struct StateMachine<const N: usize, const ALPHA: usize> {
    table: [[usize; ALPHA]; N],
    accepting: [bool; N],
    current: usize,
}

impl<const N: usize, const ALPHA: usize> StateMachine<N, ALPHA> {
    pub fn new(table: [[usize; ALPHA]; N], accepting: [bool; N]) -> Self {
        Self { table, accepting, current: 0 }
    }
    pub fn step(&mut self, symbol: usize) {
        self.current = self.table[self.current][symbol];
    }
    pub fn is_accepting(&self) -> bool { self.accepting[self.current] }
    pub fn reset(&mut self) { self.current = 0; }
}

fn main() {
    // Matrix arithmetic
    let mut a: Matrix<2, 3> = Matrix::zero();
    a.data = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];

    let mut b: Matrix<3, 2> = Matrix::zero();
    b.data = [[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]];

    println!("A (2×3):\n{a}");
    println!("B (3×2):\n{b}");

    let c: Matrix<2, 2> = mat_mul(&a, &b);
    println!("A × B (2×2):\n{c}");
    // [[58, 64], [139, 154]]

    let id: Matrix<3, 3> = Matrix::identity();
    println!("I₃ trace = {}", id.trace()); // 3

    // State machine: DFA for strings ending in "ab" (3 states, 2 symbols)
    // States: 0=start, 1=saw 'a', 2=saw 'ab' (accepting)
    // Alphabet: 0='a', 1='b'
    let mut sm: StateMachine<3, 2> = StateMachine::new(
        [[1, 0], [1, 2], [1, 0]],  // transitions
        [false, false, true],       // accepting states
    );
    for symbol in [0, 1] { sm.step(symbol); } // "ab"
    println!("\nDFA after 'ab': accepting={}", sm.is_accepting());
    sm.reset();
    for symbol in [1, 0] { sm.step(symbol); } // "ba"
    println!("DFA after 'ba': accepting={}", sm.is_accepting());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_trace() {
        let id: Matrix<3, 3> = Matrix::identity();
        assert_eq!(id.trace(), 3.0);
    }

    #[test]
    fn mat_mul_2x2() {
        let mut a: Matrix<2, 2> = Matrix::zero();
        a.data = [[1.0, 0.0], [0.0, 1.0]];
        let mut b: Matrix<2, 2> = Matrix::zero();
        b.data = [[2.0, 3.0], [4.0, 5.0]];
        let c = mat_mul(&a, &b);
        assert_eq!(c.data, b.data); // I × B = B
    }

    #[test]
    fn mat_mul_dimensions() {
        let a: Matrix<2, 3> = Matrix::zero();
        let b: Matrix<3, 4> = Matrix::zero();
        let _c: Matrix<2, 4> = mat_mul(&a, &b); // compiles = dimensions match
    }
}
