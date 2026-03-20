#![allow(clippy::all)]
//! Strategy Pattern via Closures
//!
//! Interchangeable algorithms as closure parameters and struct fields.

use std::cmp::Ordering;

/// Sorter with configurable comparison strategy.
pub struct Sorter<T> {
    compare: Box<dyn Fn(&T, &T) -> Ordering>,
}

impl<T: Clone> Sorter<T> {
    pub fn new(compare: impl Fn(&T, &T) -> Ordering + 'static) -> Self {
        Sorter {
            compare: Box::new(compare),
        }
    }

    pub fn sort(&self, mut data: Vec<T>) -> Vec<T> {
        data.sort_by(|a, b| (self.compare)(a, b));
        data
    }
}

/// Pricing with configurable discount strategy.
pub struct PriceCalculator {
    discount: Box<dyn Fn(f64) -> f64>,
}

impl PriceCalculator {
    pub fn new(discount: impl Fn(f64) -> f64 + 'static) -> Self {
        PriceCalculator {
            discount: Box::new(discount),
        }
    }

    pub fn calculate(&self, base_price: f64) -> f64 {
        (self.discount)(base_price)
    }
}

/// Common discount strategies.
pub fn no_discount() -> impl Fn(f64) -> f64 {
    |price| price
}

pub fn percentage_discount(pct: f64) -> impl Fn(f64) -> f64 {
    move |price| price * (1.0 - pct / 100.0)
}

pub fn fixed_discount(amount: f64) -> impl Fn(f64) -> f64 {
    move |price| (price - amount).max(0.0)
}

/// Validator with configurable validation strategy.
pub struct Validator<T> {
    rules: Vec<Box<dyn Fn(&T) -> Result<(), String>>>,
}

impl<T> Validator<T> {
    pub fn new() -> Self {
        Validator { rules: Vec::new() }
    }

    pub fn add_rule(mut self, rule: impl Fn(&T) -> Result<(), String> + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn validate(&self, value: &T) -> Result<(), Vec<String>> {
        let errors: Vec<String> = self
            .rules
            .iter()
            .filter_map(|rule| rule(value).err())
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<T> Default for Validator<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorter_ascending() {
        let sorter = Sorter::new(|a: &i32, b: &i32| a.cmp(b));
        assert_eq!(sorter.sort(vec![3, 1, 4, 1, 5]), vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_sorter_descending() {
        let sorter = Sorter::new(|a: &i32, b: &i32| b.cmp(a));
        assert_eq!(sorter.sort(vec![3, 1, 4, 1, 5]), vec![5, 4, 3, 1, 1]);
    }

    #[test]
    fn test_sorter_by_length() {
        let sorter = Sorter::new(|a: &String, b: &String| a.len().cmp(&b.len()));
        let result = sorter.sort(vec!["aaa".into(), "b".into(), "cc".into()]);
        assert_eq!(result, vec!["b", "cc", "aaa"]);
    }

    #[test]
    fn test_price_no_discount() {
        let calc = PriceCalculator::new(no_discount());
        assert!((calc.calculate(100.0) - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_price_percentage_discount() {
        let calc = PriceCalculator::new(percentage_discount(20.0));
        assert!((calc.calculate(100.0) - 80.0).abs() < 0.001);
    }

    #[test]
    fn test_price_fixed_discount() {
        let calc = PriceCalculator::new(fixed_discount(15.0));
        assert!((calc.calculate(100.0) - 85.0).abs() < 0.001);
    }

    #[test]
    fn test_validator_passes() {
        let validator = Validator::new()
            .add_rule(|s: &String| {
                if s.len() >= 3 {
                    Ok(())
                } else {
                    Err("too short".into())
                }
            })
            .add_rule(|s: &String| {
                if s.chars().all(|c| c.is_alphanumeric()) {
                    Ok(())
                } else {
                    Err("invalid chars".into())
                }
            });

        assert!(validator.validate(&"hello".to_string()).is_ok());
    }

    #[test]
    fn test_validator_fails() {
        let validator = Validator::new().add_rule(|n: &i32| {
            if *n > 0 {
                Ok(())
            } else {
                Err("must be positive".into())
            }
        });

        assert!(validator.validate(&-5).is_err());
    }
}
