use super::Strategy;
use crate::{
    indicators::{
        series::{Macd, Value},
        Atr, BollingerBands, Current, Indicator, Rsi,
    },
    model::Action,
};

type Indicators = (
    Current<Value>,
    Current<Macd<150.0, 200.0, 50.0>>,
    BollingerBands<20, 2.0>,
    Rsi<14>,
    Atr<14>,
);

#[derive(Clone)]
pub struct Custom {
    allowed_to_enter: bool,
}

impl Custom {
    pub fn new() -> Self {
        Self {
            allowed_to_enter: false,
        }
    }
}

impl Strategy<Indicators> for Custom {
    fn run(&mut self, analysis: Option<<Indicators as Indicator>::Analysis>) -> Action {
        if let Some((
            value,
            (_macd, _signal, _histogram),
            (upper, lower),
            rsi,
            atr
        )) = analysis {
           
            if value > upper || rsi > 70.0 {
                self.allowed_to_enter = true;
                return Action::Exit;
            }

            if value < lower && rsi < 30.0 && self.allowed_to_enter {
                self.allowed_to_enter = false;
                return Action::Enter {
                    take_profit: Some(value + 1.5 * atr),
                    stop_loss: Some(value - 1.5 * atr),
                };
            }
        }

        Action::Hold
    }
}
