use super::Strategy;
use crate::{
    indicators::{
        series::{Macd, Value},
        Atr, BollingerBands, Current, Indicator, Rsi,
    },
    model::Action,
    trader::Position,
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
        if let Some((value, (_macd, _signal, _histogram), (upper, lower), rsi, atr)) = analysis {
            let bb_stdev = (upper - lower) / 2.0;
            let bb_signal = (value - (lower + bb_stdev)) / bb_stdev;

            let rsi_signal = (rsi - 50.0) / 20.0;

            let signal = (bb_signal + rsi_signal) / 2.0;

            if signal <= -1.0 && self.allowed_to_enter {
                self.allowed_to_enter = false;
                return Action::Enter {
                    take_profit: Some(value + 2.0 * atr),
                    stop_loss: Some(value - 1.9 * atr),
                };
            } else if signal >= 0.8 {
                self.allowed_to_enter = true;
                if signal >= 1.0 {
                    return Action::Exit;
                }
            }

            /*
            if value > upper || rsi > 70.0 {
                return Action::Exit;
            }

            if value < lower && rsi < 30.0 && histogram > 0.0 {
                return Action::Enter {
                    take_profit: Some(value + 2.0 * atr),
                    stop_loss: Some(value - 2.0 * atr),
                };
            }
            */
        }

        Action::Hold
    }
}
