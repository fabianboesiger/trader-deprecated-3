use super::Series;
use crate::model::Value;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct MeanVariance<const PERIOD: usize> {
    buffer: VecDeque<f64>,
}

impl<const PERIOD: usize> Series for MeanVariance<PERIOD> {
    type Analysis = (f64, f64);

    fn new() -> Self {
        MeanVariance {
            buffer: VecDeque::with_capacity(PERIOD),
        }
    }

    fn compute(&mut self, value: f64, recover: bool) -> Option<(f64, f64)> {
        if self.buffer.len() >= PERIOD {
            let removed = self.buffer.pop_front();
            self.buffer.push_back(value);

            let mut sum = 0.0;
            let mut sum_squared = 0.0;
            for value in &self.buffer {
                sum += value;
                sum_squared += value * value;
            }

            let mean = sum / PERIOD as f64;
            let variance = sum_squared / PERIOD as f64 - mean.powi(2);

            if recover {
                self.buffer.pop_back();
                if let Some(removed) = removed {
                    self.buffer.push_front(removed);
                }
            }

            Some((mean, variance))
        } else {
            if !recover {
                self.buffer.push_back(value);
            }
            None
        }
    }
}
