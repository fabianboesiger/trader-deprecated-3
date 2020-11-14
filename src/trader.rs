use openlimits::binance::Binance;
use tokio::sync::{mpsc::Sender, Barrier};
use std::sync::Arc;
use crate::{
    indicators::Indicator,
    model::{Candlestick, Interval, Market, Order, Side, Value, Action},
    strategies::Strategy,
};
use futures::{stream, Stream, StreamExt};
use openlimits::binance::{
    client::websocket::BinanceWebsocket,
    model::{
        websocket::{BinanceWebsocketMessage, Subscription},
        KlineParams, KlineSummaries, Paginator,
    },
};
use rust_decimal::Decimal;
use tokio::time::{Instant, timeout_at};
use num_traits::cast::ToPrimitive;

#[derive(Copy, Clone)]
pub enum Position {
    Long {
        buy_value: f64,
        stop_loss: Option<f64>,
        take_profit: Option<f64>,
    },
    Short,
}

#[derive(Clone)]
pub struct Trader<S, I>
where
    I: Indicator,
    S: Strategy<I>
{
    interval: Interval,
    position: Position,
    indicator: I,
    strategy: S,
}

impl<S, I> Trader<S, I>
where
    I: Indicator,
    S: Strategy<I>
{
    pub fn new(strategy: S, interval: Interval) -> Self {
        Trader {
            interval,
            position: Position::Short,
            indicator: I::new(),
            strategy,
        }
    }

    async fn consume_candlesticks<T: Stream<Item = Candlestick>>(&mut self, stream: T, exchange: &Binance, market: Market, barrier: Arc<Barrier>, mut sender: &mut Sender<Order>) {
        let mut stream = Box::pin(stream);
        
        while let Ok(Some(candlestick)) = timeout_at(
            Instant::now() + std::time::Duration::from_secs(180),
            stream.next()
        ).await {
            println!("consume candlestick {}", market);

            let recover = !candlestick.last;
            let analysis = self.indicator.compute(&candlestick, recover);
            let current_value = candlestick.close.to_f64().unwrap();

            println!("analysis {:?}", analysis);
    
            let mut action = self.strategy.run(analysis, self.position);

            let do_exit = if let Position::Long {
                stop_loss,
                take_profit,
                ..
            } = self.position {
                let do_stop_loss = if let Some(stop_loss) = stop_loss {
                    current_value <= stop_loss
                } else {
                    false
                };
                let do_take_profit = if let Some(take_profit) = take_profit {
                    current_value >= take_profit
                } else {
                    false
                };
                do_stop_loss || do_take_profit
            } else {
                false
            };

            if do_exit {
                action = Action::Exit;
            }

            let side = match action {
                Action::Enter {
                    stop_loss,
                    take_profit,
                } => {
                    if let Position::Short = self.position {
                        self.position = Position::Long {
                            buy_value: current_value,
                            stop_loss,
                            take_profit,
                        };
                        Some(Side::Buy)
                    } else {
                        None
                    }
                },
                Action::Exit => {
                    if let Position::Long {..} = self.position {
                        self.position = Position::Short;
                        Some(Side::Sell)
                    } else {
                        None
                    }
                },
                Action::Hold => {
                    None
                }
            };
            
            if candlestick.live {
                println!("send order {}", market);

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
    
            /*
            if !candlestick.live {
                barrier.wait().await;
            }
            */
        }
    }

    pub async fn run(mut self, exchange: &Binance, market: Market, barrier: Arc<Barrier>, mut sender: Sender<Order>) {
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

        // If the stream returns none or timed out, assume connection loss and try to reconnect.
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
            println!("attempting to reconnect {}", market);
        }
    }
    
}