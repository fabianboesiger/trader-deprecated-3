mod value;
mod mean_variance;
mod ema;
//mod mma;
mod macd;

pub use value::*;
pub use mean_variance::*;
pub use ema::*;
//pub use mma::*;
pub use macd::*;

use std::fmt::Debug;

pub trait Series: Clone + Send + 'static {
    type Analysis: Clone + Debug + Send + 'static;

    fn new() -> Self;
    fn compute(&mut self, value: f64, recover: bool) -> Option<Self::Analysis>;
}