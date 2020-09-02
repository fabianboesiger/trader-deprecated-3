use crate::model::Candlestick;
use super::{Macd, Obv, MeanVariance};
use num_traits::cast::ToPrimitive;

#[derive(Clone)]
pub struct ObvMacd {
    stdev_period: usize,
    stdevs: f64,
    obv: Obv,
    obv_macd: Macd,
    stdev_mma: MeanVariance,
}

impl ObvMacd {
    pub fn new(fast: usize, slow: usize, signal: usize, stdev_period: usize, stdevs: f64) -> Self {
        ObvMacd {
            obv_macd: Macd::new(fast, slow, signal),
            stdev_period,
            stdevs,
            obv: Obv::new(),
            stdev_mma: MeanVariance::new(stdev_period),
        }
    }

    pub fn compute(&mut self, value: &Candlestick, recover: bool) -> Option<(f64, f64, f64)> {
        let obv = self.obv.compute(value, recover);
        if let Some((_macd, signal, _histogram)) = self.obv_macd.compute(obv, recover) {
            if let Some((_mean, variance)) = self.stdev_mma.compute(signal, recover) {
                let stdev = variance.sqrt();
                return Some((self.stdevs * stdev, signal, -self.stdevs * stdev));
            }
        }

        None
    }
}
