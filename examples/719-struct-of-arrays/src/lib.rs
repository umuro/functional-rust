//! # 719: Struct of Arrays (SoA) vs Array of Structs (AoS)
//!
//! Demonstrates how data layout affects cache efficiency.
//! AoS: `Vec<Particle>` — fields interleaved, bad for single-field iteration.
//! SoA: `{xs, ys, zs, masses}` — each field is contiguous, cache-friendly.

// ── Array of Structures (AoS) ─────────────────────────────────────────────────

/// Classic OOP layout: one struct per element.
/// Memory layout: `[x0,y0,z0,mass0, x1,y1,z1,mass1, ...]`.
/// When iterating only `x`, 75 % of each cache line is wasted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParticleAoS {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub mass: f32,
}

/// Sum all x-coordinates using AoS.
/// Each element is 16 bytes; only 4 bytes (`x`) are used per iteration.
pub fn sum_x_aos(particles: &[ParticleAoS]) -> f32 {
    particles.iter().map(|p| p.x).sum()
}

/// Apply gravity to y-coordinates using AoS.
/// Even though only `y` changes, the entire struct is loaded per element.
pub fn apply_gravity_aos(particles: &mut [ParticleAoS], dt: f32) {
    particles.iter_mut().for_each(|p| p.y -= 9.81 * p.mass * dt);
}

// ── Structure of Arrays (SoA) ─────────────────────────────────────────────────

/// Cache-friendly layout: each field is a separate contiguous `Vec`.
/// Memory layout: `[x0,x1,...,xN]` then `[y0,y1,...,yN]` etc.
/// Iterating over `xs` accesses only the data you need.
#[derive(Debug, Default)]
pub struct ParticlesSoA {
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
    pub zs: Vec<f32>,
    pub masses: Vec<f32>,
}

impl ParticlesSoA {
    /// Build a SoA collection from an iterator of `(x, y, z, mass)` tuples.
    pub fn from_tuples(iter: impl IntoIterator<Item = (f32, f32, f32, f32)>) -> Self {
        let (xs, ys, zs, masses) = iter.into_iter().fold(
            (vec![], vec![], vec![], vec![]),
            |(mut xs, mut ys, mut zs, mut ms), (x, y, z, m)| {
                xs.push(x);
                ys.push(y);
                zs.push(z);
                ms.push(m);
                (xs, ys, zs, ms)
            },
        );
        Self { xs, ys, zs, masses }
    }

    /// Number of particles.
    pub fn len(&self) -> usize {
        self.xs.len()
    }

    /// Returns true when there are no particles.
    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }
}

/// Sum all x-coordinates using SoA.
/// Touches **only** `xs` — every byte in every cache line is useful.
pub fn sum_x_soa(soa: &ParticlesSoA) -> f32 {
    soa.xs.iter().sum()
}

/// Apply gravity to y-coordinates using SoA.
/// Only `ys` and `masses` are accessed — two contiguous passes, zero waste.
pub fn apply_gravity_soa(soa: &mut ParticlesSoA, dt: f32) {
    soa.ys
        .iter_mut()
        .zip(soa.masses.iter())
        .for_each(|(y, &m)| *y -= 9.81 * m * dt);
}

// ── Conversion helpers ────────────────────────────────────────────────────────

/// Convert AoS to SoA (useful when you collect from an API that returns AoS).
pub fn aos_to_soa(particles: &[ParticleAoS]) -> ParticlesSoA {
    ParticlesSoA {
        xs: particles.iter().map(|p| p.x).collect(),
        ys: particles.iter().map(|p| p.y).collect(),
        zs: particles.iter().map(|p| p.z).collect(),
        masses: particles.iter().map(|p| p.mass).collect(),
    }
}

/// Convert SoA back to AoS (e.g. to pass one particle to an external API).
pub fn soa_to_aos(soa: &ParticlesSoA) -> Vec<ParticleAoS> {
    soa.xs
        .iter()
        .zip(soa.ys.iter())
        .zip(soa.zs.iter())
        .zip(soa.masses.iter())
        .map(|(((x, y), z), m)| ParticleAoS {
            x: *x,
            y: *y,
            z: *z,
            mass: *m,
        })
        .collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_aos() -> Vec<ParticleAoS> {
        vec![
            ParticleAoS {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                mass: 0.5,
            },
            ParticleAoS {
                x: 4.0,
                y: 5.0,
                z: 6.0,
                mass: 1.0,
            },
            ParticleAoS {
                x: 7.0,
                y: 8.0,
                z: 9.0,
                mass: 2.0,
            },
        ]
    }

    fn sample_soa() -> ParticlesSoA {
        ParticlesSoA::from_tuples([
            (1.0, 2.0, 3.0, 0.5),
            (4.0, 5.0, 6.0, 1.0),
            (7.0, 8.0, 9.0, 2.0),
        ])
    }

    #[test]
    fn test_sum_x_aos_and_soa_agree() {
        let aos = sample_aos();
        let soa = sample_soa();
        let expected = 1.0_f32 + 4.0 + 7.0;
        assert!((sum_x_aos(&aos) - expected).abs() < f32::EPSILON);
        assert!((sum_x_soa(&soa) - expected).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sum_x_empty() {
        assert_eq!(sum_x_aos(&[]), 0.0_f32);
        assert_eq!(sum_x_soa(&ParticlesSoA::default()), 0.0_f32);
    }

    #[test]
    fn test_apply_gravity_aos() {
        let mut particles = sample_aos();
        // dt = 0 → y unchanged
        apply_gravity_aos(&mut particles, 0.0);
        assert!((particles[0].y - 2.0).abs() < f32::EPSILON);
        // dt = 1, mass = 0.5 → y -= 9.81 * 0.5 * 1 = 4.905
        apply_gravity_aos(&mut particles, 1.0);
        let expected_y0 = 2.0 - 9.81 * 0.5 * 1.0_f32;
        assert!((particles[0].y - expected_y0).abs() < 1e-5);
    }

    #[test]
    fn test_apply_gravity_soa() {
        let mut soa = sample_soa();
        apply_gravity_soa(&mut soa, 0.0);
        assert!((soa.ys[0] - 2.0).abs() < f32::EPSILON);
        apply_gravity_soa(&mut soa, 1.0);
        let expected_y0 = 2.0 - 9.81 * 0.5 * 1.0_f32;
        assert!((soa.ys[0] - expected_y0).abs() < 1e-5);
    }

    #[test]
    fn test_aos_to_soa_roundtrip() {
        let aos = sample_aos();
        let soa = aos_to_soa(&aos);
        let back = soa_to_aos(&soa);
        assert_eq!(aos, back);
    }

    #[test]
    fn test_soa_len_and_is_empty() {
        let soa = sample_soa();
        assert_eq!(soa.len(), 3);
        assert!(!soa.is_empty());
        assert!(ParticlesSoA::default().is_empty());
    }

    #[test]
    fn test_gravity_soa_and_aos_agree() {
        let mut aos = sample_aos();
        let mut soa = sample_soa();
        let dt = 0.016_f32;
        apply_gravity_aos(&mut aos, dt);
        apply_gravity_soa(&mut soa, dt);
        for (i, p) in aos.iter().enumerate() {
            assert!((p.y - soa.ys[i]).abs() < 1e-5, "y[{i}] mismatch");
        }
    }
}
