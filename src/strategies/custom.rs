use super::Strategy;
use crate::{
    indicators::{
        series::{Macd, Value},
        Atr, BollingerBands, Current, Indicator, Rsi, Timestamp
    },
    model::Action,
};
use chrono::{DateTime, Utc, NaiveDateTime, Duration};
use rust_decimal::prelude::*;

type Indicators = (
    Timestamp,
    Current<Value>,
    Current<Macd<4.0, 8.0, 3.0>>,
    BollingerBands<20, 2.0>,
    Rsi<14>,
    Atr<14>,
);

#[derive(Clone)]
pub struct Custom {
    allowed_to_enter: bool,
    rsi_breakthrough: DateTime<Utc>,
    bb_breakthrough: DateTime<Utc>,
}

impl Strategy<Indicators> for Custom {
    fn new() -> Self {
        Self {
            allowed_to_enter: false,
            rsi_breakthrough: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            bb_breakthrough: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
        }
    }

    fn run(&mut self, analysis: Option<<Indicators as Indicator>::Analysis>) -> Action {
        if let Some((
            now,
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

            if value < lower {
                self.bb_breakthrough = now;
            }

            if rsi < 30.0 {
                self.rsi_breakthrough = now;
            }

            if 
                now - self.bb_breakthrough <= Duration::hours(3) &&
                now - self.rsi_breakthrough <= Duration::hours(3) &&
                self.allowed_to_enter
            {
                self.allowed_to_enter = false;
                return Action::Enter {
                    take_profit: Some(Decimal::from_f64(value + 1.6 * atr).unwrap()),
                    stop_loss: Some(Decimal::from_f64(value - 1.6 * atr).unwrap()),
                };
            }
        }

        Action::Hold
    }
}
