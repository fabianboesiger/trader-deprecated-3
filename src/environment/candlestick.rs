use super::{Asset, Value};
use openlimits::binance::model::{Kline, KlineSummary};
use rust_decimal::prelude::*;

#[derive(Debug)]
pub struct Candlestick<const BASE: Asset, const QUOTE: Asset> {
    pub open: Value<BASE, QUOTE>,
    pub close: Value<BASE, QUOTE>,
    pub high: Value<BASE, QUOTE>,
    pub low: Value<BASE, QUOTE>,
}

impl<const BASE: Asset, const QUOTE: Asset> From<KlineSummary> for Candlestick<BASE, QUOTE> {
    fn from(kline: KlineSummary) -> Self {
        Candlestick {
            open: Value(kline.open),
            close: Value(kline.close),
            high: Value(kline.high),
            low: Value(kline.low),
        }
    }
}

impl<const BASE: Asset, const QUOTE: Asset> From<Kline> for Candlestick<BASE, QUOTE> {
    fn from(kline: Kline) -> Self {
        Candlestick {
            open: Value(Decimal::from_str(&kline.open).unwrap()),
            close: Value(Decimal::from_str(&kline.close).unwrap()),
            high: Value(Decimal::from_str(&kline.high).unwrap()),
            low: Value(Decimal::from_str(&kline.low).unwrap()),
        }
    }
}
