use super::{Asset, Value};

use rust_decimal::prelude::*;
use serde::Serialize;
use std::{
    cmp::Ordering,
    fmt,
    ops::{AddAssign, Div, Mul, Neg, SubAssign},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct Quantity {
    pub quantity: Decimal,
    pub asset: Asset,
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.quantity, self.asset)
    }
}

impl Ord for Quantity {
    fn cmp(&self, other: &Self) -> Ordering {
        assert_eq!(self.asset, other.asset);
        self.quantity.cmp(&other.quantity)
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AddAssign for Quantity {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.asset, other.asset);
        self.quantity += other.quantity;
    }
}

impl SubAssign for Quantity {
    fn sub_assign(&mut self, other: Self) {
        assert_eq!(self.asset, other.asset);
        self.quantity -= other.quantity;
    }
}

impl Neg for Quantity {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            quantity: -self.quantity,
            asset: self.asset,
        }
    }
}

impl Mul<Value> for Quantity {
    type Output = Self;

    fn mul(self, rhs: Value) -> Self::Output {
        assert_eq!(self.asset, rhs.market.base);
        Self::Output {
            quantity: self.quantity * rhs.value,
            asset: rhs.market.quote,
        }
    }
}

impl Div<Value> for Quantity {
    type Output = Self;

    fn div(self, rhs: Value) -> Self::Output {
        assert_eq!(self.asset, rhs.market.quote);
        Quantity {
            quantity: self.quantity / rhs.value,
            asset: rhs.market.base,
        }
    }
}

impl Mul<Decimal> for Quantity {
    type Output = Self;

    fn mul(self, rhs: Decimal) -> Self::Output {
        Self::Output {
            quantity: self.quantity * rhs,
            asset: self.asset,
        }
    }
}

impl Div<Decimal> for Quantity {
    type Output = Self;

    fn div(self, rhs: Decimal) -> Self::Output {
        Self::Output {
            quantity: self.quantity / rhs,
            asset: self.asset,
        }
    }
}
