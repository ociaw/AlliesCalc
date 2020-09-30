use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Sub, SubAssign},
};

use crate::*;

/// A statistic measuring the mean and variance of some value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Stat {
    /// The mean of the value.
    pub mean: f64,
    /// The variance of the value.
    pub variance: f64,
}

impl Stat {
    /// Adds the value to the statistic, updating the mean and the variance.
    pub fn add_value<T: Into<f64>>(&mut self, value: T, p: Probability, total_p: Probability) {
        let value: f64 = value.into();
        let p: f64 = p.into();
        let total_p: f64 = total_p.into();

        let old_mean = self.mean;
        self.mean += (p / total_p) * (value - self.mean);
        self.variance += p * (value - old_mean) * (value - self.mean);
    }

    /// Gets the standard deviation of the statistic.
    pub fn std_dev(&self) -> f64 {
        self.variance.sqrt()
    }
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "μ: {:>7.2}, σ: {:>6.2}", self.mean, self.std_dev())
    }
}

impl Add for Stat {
    type Output = Stat;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            mean: self.mean + rhs.mean,
            variance: self.variance + rhs.variance,
        }
    }
}

impl AddAssign for Stat {
    fn add_assign(&mut self, rhs: Self) {
        self.mean += rhs.mean;
        self.variance += rhs.variance;
    }
}

impl Sub for Stat {
    type Output = Stat;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            mean: self.mean - rhs.mean,
            variance: self.variance + rhs.variance,
        }
    }
}

impl SubAssign for Stat {
    fn sub_assign(&mut self, rhs: Self) {
        self.mean -= rhs.mean;
        self.variance += rhs.variance;
    }
}

impl Div<f64> for Stat {
    type Output = Stat;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            mean: self.mean / rhs,
            variance: self.variance / rhs,
        }
    }
}

impl DivAssign<f64> for Stat {
    fn div_assign(&mut self, rhs: f64) {
        self.mean /= rhs;
        self.variance /= rhs;
    }
}

impl Default for Stat {
    fn default() -> Self {
        Self {
            mean: 0.0,
            variance: 0.0,
        }
    }
}
