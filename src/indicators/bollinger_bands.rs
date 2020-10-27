use super::{Indicator, series::{Series, MeanVariance}};
use crate::model::Candlestick;
use num_traits::cast::ToPrimitive;

#[derive(Clone)]
pub struct BollingerBands<const PERIOD: usize, const STDEV: f64> {
    mean_variance: MeanVariance<PERIOD>,
}

impl<const PERIOD: usize, const STDEV: f64> Indicator for BollingerBands<PERIOD, STDEV> {
    type Analysis = (f64, f64);

    fn new() -> Self {
        BollingerBands {
            mean_variance: MeanVariance::<PERIOD>::new(),
        }
    }

    fn compute(&mut self, candlestick: &Candlestick, recover: bool) -> Option<(f64, f64)> {
        let typical_price = (candlestick.high.to_f64().unwrap()
            + candlestick.low.to_f64().unwrap()
            + candlestick.close.to_f64().unwrap())
            / 3.0;
        if let Some((mean, variance)) = self.mean_variance.compute(typical_price, recover) {
            let standard_derivation = variance.sqrt();
            let upper = mean + STDEV * standard_derivation;
            let lower = mean - STDEV * standard_derivation;
            Some((upper, lower))
        } else {
            None
        }
    }
}
