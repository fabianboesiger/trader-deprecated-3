use crate::model::{Quantity, Value};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Log {
    Order { buy: Quantity, sell: Quantity },
    Value(Value),
}
