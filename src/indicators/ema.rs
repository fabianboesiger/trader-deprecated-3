#[derive(Clone)]
pub struct Ema {
    alpha: f64,
    ema: Option<f64>,
}

impl Ema {
    pub fn new(alpha: f64) -> Self {
        Ema { alpha, ema: None }
    }

    pub fn compute(&mut self, value: f64, recover: bool) -> Option<f64> {
        if let Some(mut ema) = self.ema {
            ema += self.alpha * (value - ema);
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
