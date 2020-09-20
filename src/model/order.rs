use super::Value;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Order {
    pub side: Option<Side>,
    pub value: Value,
    pub timestamp: DateTime<Utc>,
}