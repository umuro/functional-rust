/// Space Age — Float Computation with Variants
///
/// Ownership: Planet is Copy (enum with no data). All computations use f64 (Copy).

#[derive(Debug, Clone, Copy)]
pub enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

impl Planet {
    pub fn orbital_period(self) -> f64 {
        match self {
            Planet::Mercury => 0.2408467,
            Planet::Venus => 0.61519726,
            Planet::Earth => 1.0,
            Planet::Mars => 1.8808158,
            Planet::Jupiter => 11.862615,
            Planet::Saturn => 29.447498,
            Planet::Uranus => 84.016846,
            Planet::Neptune => 164.79132,
        }
    }

    pub const ALL: [Planet; 8] = [
        Planet::Mercury,
        Planet::Venus,
        Planet::Earth,
        Planet::Mars,
        Planet::Jupiter,
        Planet::Saturn,
        Planet::Uranus,
        Planet::Neptune,
    ];
}

const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

pub fn age_on(planet: Planet, seconds: f64) -> f64 {
    seconds / (EARTH_YEAR_SECONDS * planet.orbital_period())
}

/// Version 2: Using a lookup table instead of match
pub fn age_on_table(planet_index: usize, seconds: f64) -> f64 {
    const PERIODS: [f64; 8] = [
        0.2408467, 0.61519726, 1.0, 1.8808158, 11.862615, 29.447498, 84.016846, 164.79132,
    ];
    seconds / (EARTH_YEAR_SECONDS * PERIODS[planet_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn test_earth() {
        assert!(approx(age_on(Planet::Earth, 1_000_000_000.0), 31.69));
    }

    #[test]
    fn test_mercury() {
        assert!(approx(age_on(Planet::Mercury, 1_000_000_000.0), 131.56));
    }

    #[test]
    fn test_neptune() {
        assert!(approx(age_on(Planet::Neptune, 1_000_000_000.0), 0.19));
    }

    #[test]
    fn test_all_planets() {
        for &p in &Planet::ALL {
            let age = age_on(p, EARTH_YEAR_SECONDS);
            assert!(approx(age, 1.0 / p.orbital_period()));
        }
    }

    #[test]
    fn test_table_version() {
        assert!(approx(age_on_table(0, 1_000_000_000.0), 131.56));
    }
}
