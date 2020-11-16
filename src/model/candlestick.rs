use chrono::{DateTime, TimeZone, Utc};
use openlimits::binance::model::{websocket::CandlestickMessage, KlineSummary};
use rust_decimal::prelude::*;

#[derive(Debug)]
pub struct Candlestick {
    pub open_time: DateTime<Utc>,
    pub close_time: DateTime<Utc>,
    pub current_time: DateTime<Utc>,
    pub open: Decimal,
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub volume: Decimal,
    pub live: bool,
    pub last: bool,
}

impl From<KlineSummary> for Candlestick {
    fn from(kline: KlineSummary) -> Candlestick {
        Candlestick {
            open_time: Utc.timestamp_millis(kline.open_time),
            close_time: Utc.timestamp_millis(kline.close_time),
            current_time: Utc.timestamp_millis(kline.close_time),
            open: kline.open,
            close: kline.close,
            high: kline.high,
            low: kline.low,
            volume: kline.volume,
            live: false,
            last: true,
        }
    }
}

impl From<CandlestickMessage> for Candlestick {
    fn from(candlestick: CandlestickMessage) -> Candlestick {
        Candlestick {
            open_time: Utc.timestamp_millis(candlestick.kline.start_time),
            close_time: Utc.timestamp_millis(candlestick.kline.end_time),
            current_time: Utc.timestamp_millis(candlestick.event_time as i64),
            open: Decimal::from_str(&candlestick.kline.open).unwrap(),
            close: Decimal::from_str(&candlestick.kline.close).unwrap(),
            high: Decimal::from_str(&candlestick.kline.high).unwrap(),
            low: Decimal::from_str(&candlestick.kline.low).unwrap(),
            volume: Decimal::from_str(&candlestick.kline.volume).unwrap(),
            live: true,
            last: candlestick.kline.is_final_bar,
        }
    }
}
