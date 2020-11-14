mod value;
mod mean_variance;
mod ema;
mod mma;
mod macd;
mod fft;

pub use value::*;
pub use mean_variance::*;
pub use ema::*;
pub use mma::*;
pub use macd::*;
pub use fft::*;

use std::fmt::Debug;

pub trait Series: Send + 'static {
    type Analysis: Debug + Send + 'static;

    fn new() -> Self;
    fn compute(&mut self, value: f64, recover: bool) -> Option<Self::Analysis>;
}