use super::Market;
use rust_decimal::Decimal;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Value {
    pub value: Decimal,
    pub market: Market,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = {} {}",
            self.market.base, self.value, self.market.quote
        )
    }
}
