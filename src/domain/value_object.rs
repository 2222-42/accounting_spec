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

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn new(ratio: Decimal) -> Result<Self, &'static str> {
        if ratio < Decimal::ZERO || ratio > Decimal::ONE {
            return Err("Ratio must be between 0 and 1");
        }
        Ok(Self(ratio))
    }

    #[allow(dead_code)]
    pub fn value(&self) -> Decimal {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_money_operations() {
        let m1 = Money::new(Decimal::from_str("100.00").unwrap());
        let m2 = Money::new(Decimal::from_str("50.00").unwrap());

        assert_eq!(m1 + m2, Money::new(Decimal::from_str("150.00").unwrap()));
        assert_eq!(m1 - m2, Money::new(Decimal::from_str("50.00").unwrap()));
        assert_eq!(-m1, Money::new(Decimal::from_str("-100.00").unwrap()));
    }

    #[test]
    fn test_allocation_ratio() {
        assert!(AllocationRatio::new(Decimal::from_str("0.5").unwrap()).is_ok());
        assert!(AllocationRatio::new(Decimal::from_str("1.0").unwrap()).is_ok());
        assert!(AllocationRatio::new(Decimal::from_str("0.0").unwrap()).is_ok());
        assert!(AllocationRatio::new(Decimal::from_str("1.1").unwrap()).is_err());
        assert!(AllocationRatio::new(Decimal::from_str("-0.1").unwrap()).is_err());
    }
}
