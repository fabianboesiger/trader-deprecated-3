pub mod series;

mod bollinger_bands;
//mod rsi;
mod obv;
//mod obv_macd;
mod current;

pub use bollinger_bands::*;
//pub use rsi::*;
pub use obv::*;
//pub use obv_macd::*;
pub use current::*;

use crate::model::Candlestick;
use std::fmt::Debug;

pub trait Indicator: Clone + Send + 'static {
    type Analysis: Clone + Debug + Send + 'static;

    fn new() -> Self;
    fn compute(&mut self, value: &Candlestick, recover: bool) -> Option<Self::Analysis>;
}

macro_rules! peel {
    ( $name:ident, $($other:ident,)* ) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => {
        impl<$($name: Indicator,)+> Indicator for ($($name,)+) {
            type Analysis = ($($name::Analysis,)+);

            fn new() -> Self {
                ($($name::new(),)+)
            }

            #[allow(non_snake_case)]
            fn compute(&mut self, candlestick: &Candlestick, recover: bool) -> Option<Self::Analysis> {
                let ($($name,)+) = self;
                
                if let ($(Some($name),)+) = ($($name.compute(candlestick, recover),)+) {
                    Some(($($name,)+))
                } else {
                    None
                }
            }
        }
        peel! { $($name,)+ }
    };
}

tuple! {T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,}