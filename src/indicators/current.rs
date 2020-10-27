use super::{Indicator, series::Series};
use crate::model::Candlestick;
use num_traits::cast::ToPrimitive;

#[derive(Clone)]
pub struct Current<S: Series> {
    series: S
}

impl<S: Series> Indicator for Current<S> {
    type Analysis = S::Analysis;

    fn new() -> Self {
        Current {
            series: S::new()
        }
    }

    fn compute(&mut self, candlestick: &Candlestick, recover: bool) -> Option<Self::Analysis> {
        self.series.compute(candlestick.close.to_f64().unwrap(), recover)
    }
}