use super::Strategy;
use crate::{
    indicators::{
        series::Value,
        Atr, BollingerBands, Current, Indicator, Rsi
    },
    model::Action,
};
use chrono::{DateTime, Utc, NaiveDateTime};
use rust_decimal::prelude::*;

type Indicators = (
    Current<Value>,
    BollingerBands<20, 2.0>,
    Rsi<14>,
    Atr<14>,
);

#[derive(Clone)]
pub struct Custom {
    allowed_to_enter: bool,
}

impl Strategy<Indicators> for Custom {
    fn new() -> Self {
        Self {
            allowed_to_enter: false,
        }
    }

    fn run(&mut self, analysis: Option<<Indicators as Indicator>::Analysis>) -> Action {
        if let Some((
            value,
            (upper, lower),
            rsi,
            atr
        )) = analysis {
           
            if value > upper || rsi > 70.0 {
                self.allowed_to_enter = true;
                return Action::Exit;
            }
           
            if 
                value < lower &&
                rsi < 30.0 &&
                //histogram >= 0.0 &&
                1.6 * atr / value >= 0.005 && // Is it actually worth the trade?
                self.allowed_to_enter
            {
                self.allowed_to_enter = false;
                return Action::Enter {
                    take_profit: Some(Decimal::from_f64(value + 1.6 * atr).unwrap()),
                    stop_loss: Some(Decimal::from_f64(value - 1.6 * atr).unwrap()),
                    stake: Decimal::new(2, 1),
                };
            }
        }

        Action::Hold
    }
}
