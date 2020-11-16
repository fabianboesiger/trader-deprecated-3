use super::Series;

#[derive(Clone)]
pub struct Mma<const PERIOD: usize> {
    mma: Option<f64>,
}

impl<const PERIOD: usize> Series for Mma<PERIOD> {
    type Analysis = f64;

    fn new() -> Self {
        Mma { mma: None }
    }

    fn compute(&mut self, value: f64, recover: bool) -> Option<f64> {
        if let Some(mut mma) = self.mma.clone() {
            mma = (((PERIOD - 1) as f64) * mma + value) / (PERIOD as f64);
            if !recover {
                self.mma = Some(mma);
            }
            Some(mma)
        } else {
            if !recover {
                self.mma = Some(value);
            }
            None
        }
    }
}
