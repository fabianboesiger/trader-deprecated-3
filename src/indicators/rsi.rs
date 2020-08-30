use crate::model::Candlestick;
use num_traits::cast::ToPrimitive;

use super::Mma;

#[derive(Clone)]
pub struct Rsi {
    previous_close: Option<f64>,
    up_mma: Mma,
    down_mma: Mma,
}

impl Rsi {
    pub fn new(period: usize) -> Self {
        Rsi {
            previous_close: None,
            up_mma: Mma::new(period),
            down_mma: Mma::new(period),
        }
    }

    pub fn compute(&mut self, value: &Candlestick, recover: bool) -> Option<f64> {
        let current_close = value.close.to_f64().unwrap();
        let output = if let Some(previous_close) = self.previous_close {
            let (up, down) = if current_close > previous_close {
                (current_close - previous_close, 0.0)
            } else {
                (previous_close - current_close, 0.0)
            };
            if let (Some(up), Some(down)) = (
                self.up_mma.compute(up, recover),
                self.down_mma.compute(down, recover),
            ) {
                let relative_strength = up / down;
                Some(100.0 - 100.0 / (1.0 + relative_strength))
            } else {
                None
            }
        } else {
            None
        };

        if !recover {
            self.previous_close = Some(current_close);
        }

        output
    }
}
