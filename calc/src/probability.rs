use core::convert::TryFrom;
use core::fmt::Display;
use core::iter::{Iterator, Sum};
use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Probability {
    value: f64,
}

const TOLERANCE: f64 = 0.000000000001;

impl Probability {
    pub fn new(value: f64) -> Self {
        use core::convert::TryInto;
        match value.try_into() {
            Err(_) => panic!("Probability value must be between 0 and 1."),
            Ok(res) => res,
        }
    }

    pub fn from_ratio(numer: u32, denom: u32) -> Self {
        if numer > denom {
            panic!(
                "Numerator must be less than or equal to denominator. {} / {}",
                numer, denom
            );
        }
        if denom == 0 {
            panic!("Denominator must be non-zero.");
        }
        Self {
            value: numer as f64 / denom as f64,
        }
    }

    pub const fn zero() -> Self {
        Self { value: 0.0 }
    }

    pub const fn one() -> Self {
        Self { value: 1.0 }
    }
}

impl Sum for Probability {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl Default for Probability {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl TryFrom<f64> for Probability {
    type Error = &'static str;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0.0 || value > 1.0 {
            Err("Probability value must be between 0 and 1.")
        } else {
            Ok(Self { value })
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
            value: if value > 1.0 { 1.0 } else { value },
        }
    }
}

impl AddAssign for Probability {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub<Probability> for Probability {
    type Output = Self;

    fn sub(self, rhs: Probability) -> Self::Output {
        let value = self.value - rhs.value;
        if value < 0.0 - TOLERANCE {
            panic!(
                "Sub results in a probability less than 0: {} - {}",
                self.value, rhs.value
            );
        }
        Self {
            value: if value < 0.0 { 0.0 } else { value },
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
        Self {
            value: self.value * rhs,
        }
    }
}

impl MulAssign for Probability {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.value.fmt(f)
    }
}

impl From<Probability> for f64 {
    fn from(value: Probability) -> Self {
        value.value
    }
}
