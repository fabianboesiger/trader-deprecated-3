use super::{
    series::{MeanVariance, Series},
    Indicator, Tr,
};
use crate::model::Candlestick;

#[derive(Clone)]
pub struct Atr<const PERIOD: usize> {
    tr: Tr,
    mean_variance: MeanVariance<PERIOD>,
}

impl<const PERIOD: usize> Indicator for Atr<PERIOD> {
    type Analysis = f64;

    fn new() -> Self {
        Atr {
            tr: Tr::new(),
            mean_variance: MeanVariance::new(),
        }
    }

    fn compute(&mut self, candlestick: &Candlestick, recover: bool) -> Option<f64> {
        let true_range = self.tr.compute(candlestick, recover);
        if let Some(true_range) = true_range {
            let mean_variance = self.mean_variance.compute(true_range, recover);
            if let Some((mean, _variance)) = mean_variance {
                Some(mean)
            } else {
                None
            }
        } else {
            None
        }
    }
}
