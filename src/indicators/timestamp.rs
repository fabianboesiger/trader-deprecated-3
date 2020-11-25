use super::Indicator;
use crate::model::Candlestick;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Timestamp;

impl Indicator for Timestamp {
    type Analysis = DateTime<Utc>;

    fn new() -> Self {
        Timestamp
    }

    fn compute(&mut self, candlestick: &Candlestick, _recover: bool) -> Option<Self::Analysis> {
        Some(candlestick.close_time)
    }
}
