use super::Value;

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Order {
    pub side: Option<Side>,
    pub value: Value,
}
