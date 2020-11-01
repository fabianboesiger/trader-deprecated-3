use crate::{
    indicators::{Indicator, BollingerBands, Current, series::{Value, Macd}},
    model::Side,
    trader::Position,
};
use num_traits::cast::ToPrimitive;

pub trait Strategy<I: Indicator>: Clone + Send + 'static {
    fn run(&mut self, analysis: Option<I::Analysis>, position: Position) -> Option<Side>;
}

#[derive(Clone)]
pub struct Custom;

type Indicators = (
    Current<Value>,
    Current<Macd<40.0, 50.0, 30.0>>,
    BollingerBands<20, 2.0>,
);

impl Strategy<Indicators> for Custom {
    fn run(&mut self, analysis: Option<<Indicators as Indicator>::Analysis>, position: Position) -> Option<Side> {
        if let Some((
            value,
            (_macd, _signal, histogram),
            (upper, lower),
        )) = analysis {
            match position {
                Position::Long(buy_value) => {
                    // Sell.
                    if value > upper {

                        //self.position = Position::Short;
                        Some(Side::Sell)
                    } else {
                        if value < buy_value.to_f64().unwrap() * 0.98 {
                            return Some(Side::Sell);
                        }

                        if value > buy_value.to_f64().unwrap() * 1.02 {
                            return Some(Side::Sell);
                        }

                        None
                    }
                }
                Position::Short => {
                    // Buy.
                    if value < lower && histogram > 0.0 {
                        //self.position = Position::Long(candlestick.close);
                        Some(Side::Buy)
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}