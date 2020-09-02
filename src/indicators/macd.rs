use crate::model::Candlestick;
use super::Mma;
use num_traits::cast::ToPrimitive;

#[derive(Clone)]
pub struct Macd {
    slow_mma: Mma,
    fast_mma: Mma,
    signal_mma: Mma,
}

impl Macd {
    pub fn new(fast: usize, slow: usize, signal: usize) -> Self {
        Macd {
            slow_mma: Mma::new(slow),
            fast_mma: Mma::new(fast),
            signal_mma: Mma::new(signal)
        }
    }

    pub fn compute(&mut self, value: f64, recover: bool) -> Option<(f64, f64, f64)> {
        if let (Some(fast), Some(slow)) = (
            self.fast_mma.compute(value, recover),
            self.slow_mma.compute(value, recover)
        ) {
            let macd = fast - slow;
            if let Some(signal) = self.signal_mma.compute(macd, recover) {
                Some((macd, signal, macd - signal))
            } else {
                None
            }
        } else {
            None
        }
    }
}
