mod ema;
mod macd;
mod mean_variance;
mod mma;
mod value;
//mod fft;

pub use ema::*;
pub use macd::*;
pub use mean_variance::*;
pub use mma::*;
pub use value::*;
//pub use fft::*;

use std::fmt::Debug;

pub trait Series: Send + 'static {
    type Analysis: Debug + Send + 'static;

    fn new() -> Self;
    fn compute(&mut self, value: f64, recover: bool) -> Option<Self::Analysis>;
}
