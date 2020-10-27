use super::{Series, Ema};

#[derive(Clone)]
pub struct Macd<const FAST: f64, const SLOW: f64, const SIGNAL: f64> {
    slow_mma: Ema::<SLOW>,
    fast_mma: Ema::<FAST>,
    signal_mma: Ema::<SIGNAL>,
}

impl<const FAST: f64, const SLOW: f64, const SIGNAL: f64> Series for Macd<FAST, SLOW, SIGNAL> {
    type Analysis = (f64, f64, f64);

    fn new() -> Self {
        Macd {
            slow_mma: Ema::new(),
            fast_mma: Ema::new(),
            signal_mma: Ema::new()
        }
    }

    fn compute(&mut self, value: f64, recover: bool) -> Option<(f64, f64, f64)> {
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
