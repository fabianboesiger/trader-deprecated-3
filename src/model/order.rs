use super::{Action, Value};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Order {
    pub action: Action,
    pub value: Value,
    pub timestamp: DateTime<Utc>,
}
