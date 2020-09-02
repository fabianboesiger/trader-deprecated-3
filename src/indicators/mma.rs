use super::Ema;

#[derive(Clone)]
pub struct Mma {
    ema: Ema
}

impl Mma {
    pub fn new(period: usize) -> Self {
        Mma {
            ema: Ema::new(1.0 / period as f64)
        }
    }

    pub fn compute(&mut self, value: f64, recover: bool) -> Option<f64> {
        self.ema.compute(value, recover)
    }
}
