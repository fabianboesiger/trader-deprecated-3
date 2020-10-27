use super::Series;

#[derive(Clone)]
pub struct Ema<const PERIOD: f64> {
    ema: Option<f64>,
}

impl<const PERIOD: f64> Series for Ema<PERIOD> {
    type Analysis = f64;

    fn new() -> Self {
        Ema { ema: None }
    }

    fn compute(&mut self, value: f64, recover: bool) -> Option<f64> {
        if let Some(mut ema) = self.ema.clone() {
            ema += (2.0 / (PERIOD + 1.0)) * (value - ema);
            if !recover {
                self.ema = Some(ema);
            }
            Some(ema)
        } else {
            if !recover {
                self.ema = Some(value);
            }
            None
        }
    }
}
