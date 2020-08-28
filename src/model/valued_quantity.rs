use super::{Quantity, Value};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct ValuedQuantity {
    pub quantity: Quantity,
    pub value: Value,
}

impl ValuedQuantity {
    pub fn get_quantity_value(&self) -> Quantity {
        self.quantity * self.value
    }
}

impl fmt::Display for ValuedQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.quantity, self.get_quantity_value())
    }
}
