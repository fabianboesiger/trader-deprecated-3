use super::Trader;
use crate::{
    indicators::{BollingerBands, Rsi, Macd, ObvMacd},
    model::{Candlestick, Interval, Market, Order, Side, Value},
};
use async_trait::async_trait;
use futures::{stream, Stream, StreamExt};
use num_traits::cast::ToPrimitive;
use openlimits::binance::{
    client::websocket::BinanceWebsocket,
    model::{
        websocket::{BinanceWebsocketMessage, Subscription},
        KlineParams, KlineSummaries, Paginator,
    },
    Binance,
};
use tokio::sync::{mpsc::Sender, Barrier};
use std::sync::Arc;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, Duration};

#[derive(Copy, Clone)]
enum Position {
    Long(Decimal),
    Short,
}

#[derive(Clone)]
pub struct Custom {
    interval: Interval,
    position: Position,
    bollinger_bands: BollingerBands,
    rsi: Rsi,
    macd: Macd,
    delta_obv: ObvMacd,
    last_stop_loss: Option<DateTime<Utc>>,
    last_sell: Option<DateTime<Utc>>,
}

impl Custom {
    pub fn new(interval: Interval) -> Self {
        Custom {
            interval,
            position: Position::Short,
            bollinger_bands: BollingerBands::new(20, 2.0),
            rsi: Rsi::new(14),
            macd: Macd::new(30, 40, 20),
            delta_obv: ObvMacd::new(2, 3, 3, 200, 2.0),
            last_stop_loss: None,
            last_sell: None,
        }
    }
}

impl Custom {
    async fn consume_candlesticks<S: Stream<Item = Candlestick>>(&mut self, stream: S, exchange: &Binance, market: Market, barrier: Arc<Barrier>, mut sender: &mut Sender<Order>) {
        let mut stream = Box::pin(stream);
        
        while let Some(candlestick) = stream.next().await {
            let analysis = (
                self.bollinger_bands.compute(&candlestick, !candlestick.last),
                self.rsi.compute(&candlestick, !candlestick.last),
                self.macd.compute(candlestick.close.to_f64().unwrap(), !candlestick.last),
                self.delta_obv.compute(&candlestick, !candlestick.last),
            );
    
            let side = if let (
                Some((upper, lower)),
                Some(rsi),
                Some((macd, signal, histogram)),
                Some((max_delta_obv, delta_obv, min_delta_obv))
            ) = analysis {
                match self.position {
                    Position::Long(buy_value) => {
                        // Sell.
                        if (candlestick.high.to_f64().unwrap() > upper && rsi > 70.0) || delta_obv < min_delta_obv {
    
                            self.position = Position::Short;
                            self.last_sell = Some(candlestick.current_time);
                            Some(Side::Sell)
                        } else {
                            // Check if stop loss kicks in.
                            /*
                            if candlestick.close < buy_value * Decimal::from_f64(0.9).unwrap() {
                                self.position = Position::Short;
                                self.last_stop_loss = Some(candlestick.current_time);
                                Some(Side::Sell)
                            } else {s
                                None
                            }*/
                            None
                        }
                    }
                    Position::Short => {
                        // Buy.
                        if candlestick.low.to_f64().unwrap() < lower && rsi < 30.0 && histogram > 0.0 && delta_obv > min_delta_obv {
                            let mut no_buy = false;
                            if let Some(last_stop_loss) = self.last_stop_loss {
                                if candlestick.current_time < last_stop_loss + Duration::hours(6) {
                                    no_buy = true;
                                }
                            }
                            if let Some(last_sell) = self.last_sell {
                                if candlestick.current_time < last_sell + Duration::hours(3) {
                                    no_buy = true;
                                }
                            }
    
                            if !no_buy {
                                self.position = Position::Long(candlestick.close);
                                Some(Side::Buy)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                }
            } else {
                None
            };
            
            if candlestick.live {
                sender
                    .send(Order {
                        value: Value {
                            value: candlestick.close,
                            market,
                        },
                        side,
                        timestamp: candlestick.current_time
                    })
                    .await
                    .unwrap();
            }
    
            
            if !candlestick.live {
                barrier.wait().await;
            }
        }
    }
    
}

#[async_trait]
impl Trader for Custom {
    async fn run(mut self, exchange: &Binance, market: Market, barrier: Arc<Barrier>, mut sender: Sender<Order>) {
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

        self.consume_candlesticks(historical_candlesticks, exchange, market, barrier.clone(), &mut sender).await;

        // Candlestick subscription.
        let sub = Subscription::Candlestick(
            format!("{}{}", market.base, market.quote).to_lowercase(),
            format!("{}", self.interval),
        );

        // If the stream returns none, assume connection loss and try to reconnect.
        loop {
            // Get live data using websocket API.
            let mut websocket = BinanceWebsocket::new();
            websocket.subscribe(sub.clone()).await.unwrap();
            let live_candlesticks = websocket.filter_map(|message| async move {
                // TODO: Host computer aborts connection.
                if let Ok(BinanceWebsocketMessage::Candlestick(candlestick)) = message {
                    Some(Candlestick::from(candlestick))
                } else {
                    None
                }
            });

            self.consume_candlesticks(live_candlesticks, exchange, market, barrier.clone(), &mut sender).await;
        }
    }
}
