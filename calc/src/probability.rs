use core::convert::TryFrom;
use core::fmt::Display;
use core::iter::{Iterator, Sum};
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use fixed::prelude::*;
use fixed::types::U1F63;

/// The probability of an event occurring. Guarnteed to lie within [0, 1].
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Probability {
    value: U1F63,
}

/// The tolerance for floating point errors when adding or subtracting.
const TOLERANCE: f64 = 0.000000000001;

impl Probability {
    /// Construct a new `Probability` with a probability of `value`.
    ///
    /// Panics if `value` is less than 0.0 or greater than 1.0.
    pub fn new(value: f64) -> Self {
        use core::convert::TryInto;
        match value.try_into() {
            Err(_) => panic!("Probability value must be between 0 and 1."),
            Ok(res) => res,
        }
    }

    /// Construct a new `Probability` with a probability of `numer` / `denom`.
    ///
    /// Panics if `numer` is greater than `denom` or if `denom` is equal to 0.
    pub fn from_ratio(numer: u32, denom: u32) -> Self {
        use fixed::types::{U32F96};

        if numer > denom {
            panic!(
                "Numerator must be less than or equal to denominator. {} / {}",
                numer, denom
            );
        }
        if denom == 0 {
            panic!("Denominator must be non-zero.");
        }

        let numer = U32F96::from_num(numer);
        let denom = U32F96::from_num(denom);
        let quotient: U1F63 = U1F63::from_num(numer / denom);
        Self {
            value: quotient,
        }
    }

    /// Returns a `Probability` with a value of `0`.
    pub fn zero() -> Self {
        Self { value: U1F63::from_num(0) }
    }

    /// Returns a `Probability` with a value of `1`.
    pub fn one() -> Self {
        Self { value: U1F63::from_num(1) }
    }
}

impl Sum for Probability {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl Default for Probability {
    fn default() -> Self {
        Self::zero()
    }
}

impl TryFrom<f64> for Probability {
    type Error = &'static str;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0.0 || value > 1.0 {
            Err("Probability value must be between 0 and 1.")
        } else {
            Ok(Self { value: value.to_fixed() })
        }
    }
}

impl Add<Probability> for Probability {
    type Output = Self;

    fn add(self, rhs: Probability) -> Self::Output {
        let value = self.value + rhs.value;
        if value > 1.0 + TOLERANCE {
            panic!(
                "Add results in a probability greater than 1: {} + {}",
                self.value, rhs.value
            );
        }
        Self {
            value: if value > 1 { U1F63::from_num(1) } else { value },
        }
    }
}

impl AddAssign for Probability {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub<Probability> for Probability {
    type Output = Self;

    fn sub(self, rhs: Probability) -> Self::Output {
        let value = self.value.saturating_sub(rhs.value);
        Self {
            value
        }
    }
}

impl SubAssign for Probability {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<Probability> for Probability {
    type Output = Self;
    fn mul(self, rhs: Probability) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl Mul<f64> for Probability {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let fixed: U1F63 = rhs.to_fixed();
        Self {
            value: self.value * fixed,
        }
    }
}

impl MulAssign for Probability {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.value.fmt(f)
    }
}

impl From<Probability> for f64 {
    fn from(value: Probability) -> Self {
        value.value.lossy_into()
    }
}

impl From<Probability> for U1F63 {
    fn from(value: Probability) -> Self {
        value.value
    }
}
