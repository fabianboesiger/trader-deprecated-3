use super::Strategy;
use crate::{
    indicators::{series::Value, Atr, BollingerBands, Current, Indicator, Rsi, Timestamp},
    model::Action,
};
use chrono::{DateTime, Duration, TimeZone, Utc};
use rust_decimal::prelude::*;

type Indicators = (
    Current<Value>,
    BollingerBands<20, 2.0>,
    Rsi<14>,
    Atr<14>,
    Timestamp,
);

#[derive(Clone)]
pub struct Custom {
    allowed_to_enter: bool,
    rsi_breakthrough: DateTime<Utc>,
}

impl Strategy<Indicators> for Custom {
    fn new() -> Self {
        Self {
            allowed_to_enter: false,
            rsi_breakthrough: Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 0, 0),
        }
    }

    fn run(&mut self, analysis: Option<<Indicators as Indicator>::Analysis>) -> Action {
        println!("{:?}", analysis);

        if let Some((value, (upper, lower), rsi, atr, now)) = analysis {
            if rsi > 70.0 {
                self.allowed_to_enter = true;
                return Action::Exit;
            }

            if rsi < 30.0 {
                self.rsi_breakthrough = now;
            }

            if
                1.8 * atr / value >= 0.005 && // Is it actually worth the trade?
                self.allowed_to_enter &&
                rsi >= 30.0 &&
                now - self.rsi_breakthrough <= Duration::hours(2) &&
                now - self.rsi_breakthrough >= Duration::minutes(2)
            {
                self.allowed_to_enter = false;
                return Action::Enter {
                    take_profit: Some(Decimal::from_f64(value + atr * 1.8).unwrap()),
                    stop_loss: Some(Decimal::from_f64(value - atr * 1.8).unwrap()),
                    stake: Decimal::new(3, 1),
                };
            }
        }

        Action::Hold
    }
}
