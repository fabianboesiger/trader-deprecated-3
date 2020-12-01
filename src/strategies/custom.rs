use super::Strategy;
use crate::{
    indicators::{
        series::Value,
        Atr, BollingerBands, Current, Indicator, Rsi, Timestamp
    },
    model::Action,
};
use rust_decimal::prelude::*;
use chrono::{DateTime, Utc, TimeZone, Duration};

type Indicators = (
    Current<Value>,
    BollingerBands<20, 2.0>,
    Rsi<14>,
    Atr<14>,
    Timestamp
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
        
        if let Some((
            value,
            (upper, lower),
            rsi,
            atr,
            now
        )) = analysis {
           
            if rsi > 70.0 || value > upper {
                self.allowed_to_enter = true;
                return Action::Exit;
            }
            
            if rsi < 30.0 {
                self.rsi_breakthrough = now;
            }
            
            if 
                //rsi <= 50.0 &&
                //value <= lower &&
                //value < lower &&
                ((
                    rsi >= 30.0 &&
                    now - self.rsi_breakthrough <= Duration::hours(2)
                ) || (
                    rsi >= 40.0 &&
                    rsi <= 60.0 &&
                    value < lower
                )) &&
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
