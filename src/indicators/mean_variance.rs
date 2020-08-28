use std::collections::VecDeque;

#[derive(Clone)]
pub struct MeanVariance {
    period: usize,
    buffer: VecDeque<f64>,
}

impl MeanVariance {
    pub fn new(period: usize) -> Self {
        MeanVariance {
            period,
            buffer: VecDeque::with_capacity(period),
        }
    }

    pub fn update(&mut self, value: f64) -> Option<(f64, f64)> {
        if self.buffer.len() >= self.period {
            self.buffer.pop_front();
            self.buffer.push_back(value);

            let mut sum = 0.0;
            let mut sum_squared = 0.0;
            for value in &self.buffer {
                sum += value;
                sum_squared += value * value;
            }

            let mean = sum / self.period as f64;
            let variance = sum_squared / self.period as f64 - mean.powi(2);

            Some((mean, variance))
        } else {
            self.buffer.push_back(value);
            None
        }
    }
}
