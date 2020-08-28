use openlimits::binance::model::{Kline, KlineSummary};
use rust_decimal::prelude::*;

#[derive(Debug)]
pub struct Candlestick {
    pub open: Decimal,
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
}

impl From<KlineSummary> for Candlestick {
    fn from(kline: KlineSummary) -> Candlestick {
        Candlestick {
            open: kline.open,
            close: kline.close,
            high: kline.high,
            low: kline.low,
        }
    }
}

impl From<Kline> for Candlestick {
    fn from(kline: Kline) -> Candlestick {
        Candlestick {
            open: Decimal::from_str(&kline.open).unwrap(),
            close: Decimal::from_str(&kline.close).unwrap(),
            high: Decimal::from_str(&kline.high).unwrap(),
            low: Decimal::from_str(&kline.low).unwrap(),
        }
    }
}
