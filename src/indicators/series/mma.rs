use super::{Series, Ema};

#[derive(Clone)]
pub struct Mma<const PERIOD: f64> {
    ema: Ema::<{1.0 / PERIOD as f64}>
}

impl<const PERIOD: usize> Series for Mma<PERIOD> {
    type Analysis = f64;

    fn new() -> Self {
        Mma {
            ema: Ema::new()
        }
    }

    fn compute(&mut self, value: f64, recover: bool) -> Option<f64> {
        self.ema.compute(value, recover)
    }
}
