use super::MeanVariance;
use crate::model::Candlestick;
use num_traits::cast::ToPrimitive;

#[derive(Clone)]
pub struct BollingerBands {
    standard_derivations: f64,
    mean_variance: MeanVariance,
}

impl BollingerBands {
    pub fn new(period: usize, standard_derivations: f64) -> Self {
        BollingerBands {
            standard_derivations,
            mean_variance: MeanVariance::new(period),
        }
    }

    pub fn update(&mut self, value: &Candlestick) -> Option<(f64, f64)> {
        let typical_price = (value.high.to_f64().unwrap()
            + value.low.to_f64().unwrap()
            + value.close.to_f64().unwrap())
            / 3.0;
        if let Some((mean, variance)) = self.mean_variance.update(typical_price) {
            let standard_derivation = variance.sqrt();
            let upper = mean + self.standard_derivations * standard_derivation;
            let lower = mean - self.standard_derivations * standard_derivation;
            Some((upper, lower))
        } else {
            None
        }
    }
}
