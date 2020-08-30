use chrono::{DateTime, TimeZone, Utc};
use openlimits::binance::model::{Kline, KlineSummary};
use rust_decimal::prelude::*;

#[derive(Debug)]
pub struct Candlestick {
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub open: Decimal,
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub live: bool,
    pub last: bool,
}

impl From<KlineSummary> for Candlestick {
    fn from(kline: KlineSummary) -> Candlestick {
        Candlestick {
            open_time: Utc.timestamp_millis(kline.open_time),
            close_time: Utc.timestamp_millis(kline.close_time),
            open: kline.open,
            close: kline.close,
            high: kline.high,
            low: kline.low,
            live: false,
            last: true,
        }
    }
}

impl From<Kline> for Candlestick {
    fn from(kline: Kline) -> Candlestick {
        Candlestick {
            open_time: Utc.timestamp_millis(kline.start_time),
            close_time: Utc.timestamp_millis(kline.end_time),
            open: Decimal::from_str(&kline.open).unwrap(),
            close: Decimal::from_str(&kline.close).unwrap(),
            high: Decimal::from_str(&kline.high).unwrap(),
            low: Decimal::from_str(&kline.low).unwrap(),
            live: true,
            last: kline.is_final_bar,
        }
    }
}
