use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Money(Decimal);

impl Money {
    pub fn new(amount: Decimal) -> Self {
        Self(amount)
    }

    pub fn amount(&self) -> Decimal {
        self.0
    }

    pub fn zero() -> Self {
        Self(Decimal::ZERO)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add for Money {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Money {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Neg for Money {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AllocationRatio(Decimal);

impl AllocationRatio {
    pub fn new(ratio: Decimal) -> Result<Self, String> {
        if ratio < Decimal::ZERO || ratio > Decimal::ONE {
            return Err("Ratio must be between 0 and 1".to_string());
        }
        Ok(Self(ratio))
    }

    pub fn value(&self) -> Decimal {
        self.0
    }
}
