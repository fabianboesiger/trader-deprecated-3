use crate::model::Candlestick;
use num_traits::cast::ToPrimitive;

#[derive(Clone)]
pub struct Obv {
    sum: f64,
    previous_value: Option<f64>, 
}

impl Obv {
    pub fn new() -> Self {
        Obv {
            sum: 0.0,
            previous_value: None,
        }
    }

    pub fn compute(&mut self, value: &Candlestick, recover: bool) -> f64 {
        let current_value = value.close.to_f64().unwrap();
        let volume = value.volume.to_f64().unwrap();
        let new_sum = self.sum + if let Some(previous_value) = self.previous_value {
            if current_value > previous_value {
                volume
            } else
            if current_value < previous_value {
                -volume
            } else {
                0.0
            }
        } else {
            0.0
        };
        self.previous_value = Some(current_value);

        if !recover {
            self.sum = new_sum;
        }

        new_sum
    }
}
