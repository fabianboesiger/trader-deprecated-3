use super::{series::Series, Indicator};
use crate::model::Candlestick;
use num_traits::cast::ToPrimitive;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Timestamp;

impl Indicator for Timestamp {
    type Analysis = DateTime<Utc>;

    fn new() -> Self {
        Timestamp
    }

    fn compute(&mut self, candlestick: &Candlestick, recover: bool) -> Option<Self::Analysis> {
        Some(candlestick.close_time)
    }
}
