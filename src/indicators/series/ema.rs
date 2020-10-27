use super::Series;

#[derive(Clone)]
pub struct Ema<const ALPHA: f64> {
    ema: Option<f64>,
}

impl<const ALPHA: f64> Series for Ema<ALPHA> {
    type Analysis = f64;

    fn new() -> Self {
        Ema { ema: None }
    }

    fn compute(&mut self, value: f64, recover: bool) -> Option<f64> {
        if let Some(mut ema) = self.ema.clone() {
            ema += ALPHA * (value - ema);
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
