use super::Indicator;
use crate::model::Candlestick;
use num_traits::cast::ToPrimitive;

#[derive(Clone)]
pub struct Tr {
    previous_close: Option<f64>,
}

impl Indicator for Tr {
    type Analysis = f64;

    fn new() -> Self {
        Tr {
            previous_close: None,
        }
    }

    fn compute(&mut self, candlestick: &Candlestick, recover: bool) -> Option<f64> {
        let high = candlestick.high.to_f64().unwrap();
        let low = candlestick.low.to_f64().unwrap();

        let result = if let Some(previous_close) = self.previous_close {
            let true_range = (high - low)
                .max((high - previous_close).abs())
                .max((high - previous_close).abs());
            Some(true_range)
        } else {
            None
        };

        if !recover {
            let close = candlestick.close.to_f64().unwrap();
            self.previous_close = Some(close);
        }

        result
    }
}
