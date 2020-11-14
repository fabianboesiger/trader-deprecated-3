use super::Strategy;
use crate::{
    indicators::{Indicator, BollingerBands, Current, Rsi, Atr, series::{Value, Macd}},
    model::{Side, Action},
    trader::Position,
};
use num_traits::cast::ToPrimitive;

type Indicators = (
    Current<Value>,
    Current<Macd<150.0, 200.0, 50.0>>,
    BollingerBands<20, 2.0>,
    Rsi<14>,
    Atr<14>,
);

#[derive(Clone)]
pub struct Custom;

impl Strategy<Indicators> for Custom {
    fn run(&mut self, analysis: Option<<Indicators as Indicator>::Analysis>, position: Position) -> Action {
        if let Some((
            value,
            (_macd, _signal, histogram),
            (upper, lower),
            rsi,
            atr,
        )) = analysis {
            if value > upper || rsi > 70.0 {
                return Action::Exit;
            }

            if value < lower && rsi < 30.0 && histogram > 0.0 {
                return Action::Enter {
                    take_profit: Some(value + 1.5 * atr),
                    stop_loss: Some(value - 1.5 * atr),
                };
            }
        }

        Action::Hold
    }
}