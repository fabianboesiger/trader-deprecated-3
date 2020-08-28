use super::Trader;
use crate::{
    indicators::BollingerBands,
    model::{Candlestick, Interval, Market, Order, Side, Value},
};
use async_trait::async_trait;
use futures::{stream, StreamExt};
use num_traits::cast::ToPrimitive;
use openlimits::binance::{
    client::websocket::BinanceWebsocket,
    model::{
        websocket::{BinanceWebsocketMessage, Subscription},
        KlineParams, KlineSummaries, Paginator,
    },
    Binance,
};
use tokio::sync::mpsc::Sender;

#[derive(Copy, Clone)]
enum Position {
    Long,
    Short,
}

#[derive(Clone)]
pub struct Bollinger {
    interval: Interval,
    position: Position,
    bollinger_bands: BollingerBands,
}

impl Bollinger {
    pub fn new(interval: Interval, period: usize, standard_derivations: f64) -> Self {
        Bollinger {
            interval,
            position: Position::Short,
            bollinger_bands: BollingerBands::new(period, standard_derivations),
        }
    }
}
#[async_trait]
impl Trader for Bollinger {
    async fn run(&mut self, exchange: &Binance, market: Market, mut sender: Sender<Order>) {
        // Get historical data using REST API.
        let params = KlineParams {
            symbol: format!("{}{}", market.base, market.quote),
            interval: format!("{}", self.interval),
            paginator: Some(Paginator {
                start_time: None,
                end_time: None,
                limit: Some(1000),
                from_id: None,
                order_id: None,
            }),
        };
        let response = exchange.get_klines(&params).await.unwrap();
        let KlineSummaries::AllKlineSummaries(summaries) = response;
        let historical_candlesticks = stream::iter(
            summaries
                .into_iter()
                .map(Candlestick::from)
                .collect::<Vec<Candlestick>>(),
        );

        // Get live data using websocket API.
        let mut websocket = BinanceWebsocket::new();
        let sub = Subscription::Candlestick(
            format!("{}{}", market.base, market.quote).to_lowercase(),
            format!("{}", self.interval),
        );
        websocket.subscribe(sub).await.unwrap();
        let live_candlesticks = websocket.filter_map(|message| async move {
            if let BinanceWebsocketMessage::Candlestick(candlestick) = message.unwrap() {
                if candlestick.kline.is_final_bar {
                    return Some(Candlestick::from(candlestick.kline));
                }
            }
            None
        });

        // Chain and handle the candlesticks.
        let mut stream = historical_candlesticks.chain(live_candlesticks).boxed();
        while let Some(candlestick) = stream.next().await {
            let side = if let Some((upper, lower)) = self.bollinger_bands.update(&candlestick) {
                match self.position {
                    Position::Long => {
                        if candlestick.high.to_f64().unwrap() > upper {
                            self.position = Position::Short;
                            Some(Side::Sell)
                        } else {
                            None
                        }
                    }
                    Position::Short => {
                        if candlestick.low.to_f64().unwrap() < lower {
                            self.position = Position::Long;
                            Some(Side::Buy)
                        } else {
                            None
                        }
                    }
                }
            } else {
                None
            };

            sender
                .send(Order {
                    value: Value {
                        value: candlestick.close,
                        market,
                    },
                    side,
                })
                .await
                .unwrap();
        }
    }
}
