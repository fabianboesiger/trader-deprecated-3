use super::{Indicator, series::{Series, Mma}};
use crate::model::Candlestick;
use num_traits::cast::ToPrimitive;


#[derive(Clone)]
pub struct Rsi<const PERIOD: usize> {
    previous_close: Option<f64>,
    up_mma: Mma::<PERIOD>,
    down_mma: Mma::<PERIOD>,
}

impl<const PERIOD: usize> Indicator for Rsi<PERIOD> {
    type Analysis = f64;

    fn new() -> Self {
        Rsi {
            previous_close: None,
            up_mma: Mma::new(),
            down_mma: Mma::new(),
        }
    }

    fn compute(&mut self, value: &Candlestick, recover: bool) -> Option<f64> {
        let current_close = value.close.to_f64().unwrap();
        let output = if let Some(previous_close) = self.previous_close {
            let (up, down) = if current_close > previous_close {
                (current_close - previous_close, 0.0)
            } else {
                (0.0, previous_close - current_close)
            };

            if let (Some(up), Some(down)) = (
                self.up_mma.compute(up, recover),
                self.down_mma.compute(down, recover),
            ) {
                if down != 0.0 {
                    let relative_strength = up / down;
                    Some(100.0 - 100.0 / (1.0 + relative_strength))
                } else {
                    Some(100.0)
                }
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
