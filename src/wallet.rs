use crate::model::{Trades, Trade, Asset, Market, Order, Quantity, Value, ValuedQuantity, MAIN_ASSET, Action};
use crate::backends::Backend;
use std::collections::HashMap;
use crate::{
    loggers::Log,
    backends::Exchange
};
use async_trait::async_trait;
use openlimits::binance::Binance;
use rust_decimal::prelude::*;
use tokio::sync::mpsc::{Receiver, UnboundedSender};
use sqlx::PgPool;
use tokio::sync::mpsc::channel;
use tokio::stream::StreamExt;
use num_traits::cast::ToPrimitive;
use chrono::{DateTime, Utc};
use serde::Serialize;

const FEE: f64 = 0.001;

pub enum Message {
    Order(Order),
    Exchange(Exchange)
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct State {
    pub position: Position,
    pub valued_quantity: ValuedQuantity,
}

pub type States = HashMap<Asset, State>;

#[derive(Copy, Clone, Debug, Serialize)]
pub enum Position {
    Long {
        stop_loss: Option<Decimal>,
        take_profit: Option<Decimal>,
    },
    Short,
}

pub struct Wallet<B: Backend> {
    trades: Trades,
    backend: std::marker::PhantomData<B>,
    sender: UnboundedSender<String>,
}

impl<B: Backend> Wallet<B> {

    pub async fn new(sender: UnboundedSender<String>) -> Self {

        
        /*
        let assets: HashMap<Asset, State> = sqlx::query!("
                WITH
                all_moves AS (
                    SELECT
                        base_asset AS asset,
                        base_quantity AS quantity,
                        date_time
                    FROM trades
                    WHERE is_long
                    UNION ALL
                    SELECT
                        base_asset AS asset,
                        -base_quantity AS quantity,
                        date_time
                        NULL,
                    FROM trades
                    WHERE NOT is_long
                    UNION ALL
                    SELECT
                        'USDT',
                        1000.0,
                        NULL
                ),
                last_moves AS (
                    SELECT
                        base_asset AS asset,
                        stop_loss,
                        take_profit,
                        ROW_NUMBER() OVER (
                            PARTITION BY base_asset
                            ORDER BY date_time DESC
                        ) AS row_number
                    FROM trades
                    WHERE row_number = 1
                ),
                summed_moves AS (
                    SELECT asset, SUM(quantity) AS quantity
                    FROM moves
                    WHERE date_time <= NOW()
                    OR date_time IS NULL
                    GROUP BY asset
                )
                SELECT
                    asset,
                    quantity,
                    date_time,
                    stop_loss,
                    take_profit
                FROM summed_moves
                LEFT JOIN last_moves
                USING (asset)
            ")
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| {
                let asset = row.asset.unwrap().into();
                (
                    asset,
                    State {
                        position: if let (
                            Some(stop_loss),
                            Some(take_profit)
                        ) = (
                            row.stop_loss,
                            row.take_profit
                        ) {
                            Position::Long {
                                stop_loss: Some(stop_loss.to_f64().unwrap()),
                                take_profit: Some(take_profit.to_f64().unwrap()),
                            }
                        } else {
                            Position::Short
                        },
                        valued_quantity: ValuedQuantity {
                            quantity: Quantity {
                                quantity: if let Some(quantity) = row.quantity {
                                    //Decimal::from_f64(row.quantity.unwrap()).unwrap()
                                    *quantity
                                } else 
                                if asset == MAIN_ASSET {
                                    Decimal::from_f64(1000.0).unwrap()
                                } else {
                                    Decimal::zero()
                                },
                                asset,
                            },
                            value: Value {
                                value: Decimal::one(),
                                market: Market {
                                    base: asset,
                                    quote: MAIN_ASSET,
                                },
                            },
                        },
                    }
                )
            })
            .collect();
        
        let mut default_assets = HashMap::new();
        for asset in Asset::all() {
            assets.insert(asset, State {
                position: Position::Short,
                valued_quantity: ValuedQuantity {
                    quantity: Quantity {
                        quantity: if asset == MAIN_ASSET {
                            Decimal::from_f64(1000.0).unwrap()
                        } else {
                            Decimal::zero()
                        },
                        asset,
                    },
                    value: Value {
                        value: Decimal::one(),
                        market: Market {
                            base: asset,
                            quote: MAIN_ASSET,
                        },
                    },
                },
            });
        }
        
        for (k, v) in default_assets.drain() {
            if !assets.contains_key(&k) {
                assets.insert(k, v);
            }
        }
        */

        let mut trades = Trades::new().await;
        trades.fetch_all().await;

        Self {
            trades: Trades::new().await,
            backend: std::marker::PhantomData,
            sender,
        }
    }
    /*
    async fn insert_trade(&mut self, trade: Trade) {
        // TODO: Get rid of unnecessary clone
        self.trades.insert(trade.clone(), &mut self.assets).await;
        self.sender.send(Log::State(*self.assets.get_mut(&trade.base.asset).unwrap())).unwrap();
    }
    */
    

    pub async fn run(
        mut self,
        exchange: &'static Binance,
        mut receiver: Receiver<Order>,
    ) {
        while let Some(Order {
            mut action,
            value,
            timestamp,
        }) = receiver.next().await {
            assert_eq!(value.market.quote, MAIN_ASSET);

            let base = value.market.base;
            let quote = value.market.quote;

            self.trades.update_value(value);
            self.sender.send(self.trades.render()).unwrap();

            let sum = self.trades.total();

            // Check if stop loss or take profit kicks in.
            if if let Position::Long {
                stop_loss,
                take_profit,
            } = self.trades.get_position(&base)
            {
                let do_stop_loss = if let Some(stop_loss) = stop_loss {
                    value.value <= stop_loss
                } else {
                    false
                };
                let do_take_profit = if let Some(take_profit) = take_profit {
                    value.value >= take_profit
                } else {
                    false
                };
                do_stop_loss || do_take_profit
            } else {
                false
            } {
                action = Action::Exit;
            }

            // Execute the action.
            match action {
                Action::Enter {
                    stop_loss,
                    take_profit,
                } => {
                    if let Position::Short = self.trades.get_position(&base) {
                        let quantity = sum * Decimal::new(2, 1);
                        if quantity <= self.trades.get_quantity(&quote) {
                            let buy = quantity / value * (Decimal::one() - Decimal::from_f64(FEE).unwrap());
                            let sell = quantity;
                            /*
                            sender
                                .send(Log::Trade {
                                    buy,
                                    sell,
                                    timestamp,
                                })
                                .await
                                .unwrap();
                            */
                            
                            let position = Position::Long {
                                stop_loss,
                                take_profit,
                            };

                            self.trades.insert(Trade {
                                base: buy,
                                quote: sell,
                                value,
                                timestamp,
                                position,
                            }).await;
                            self.sender.send(self.trades.render()).unwrap();

                        }
                        /*
                        let sender_clone = exchange_sender.clone();
                        tokio::task::spawn(async move {
                            B::new().buy(&exchange, sender_clone).await;
                        });
                        */
                    }
                    
                },
                Action::Exit => {
                    if let Position::Long {
                        stop_loss,
                        take_profit,
                    } = self.trades.get_position(&base) {
                        let quantity = self.trades.get_quantity(&base);
                        if !quantity.is_zero() {
                            let buy = quantity * value * (Decimal::one() - Decimal::from_f64(FEE).unwrap());
                            let sell = quantity;
                            /*
                            sender
                                .send(Log::Trade {
                                    buy,
                                    sell,
                                    timestamp,
                                })
                                .await
                                .unwrap();
                            */

                            let position = Position::Short;

                            self.trades.insert(Trade {
                                base: sell,
                                quote: buy,
                                value,
                                timestamp,
                                position,
                            }).await;
                            self.sender.send(self.trades.render()).unwrap();

                        }
                        /*
                        let sender_clone = exchange_sender.clone();
                        tokio::task::spawn(async move {
                            B::new().sell(&exchange, sender_clone).await;
                        });
                        */
                        
                    }
                },
                Action::Hold => {}
            }
        }
    }
}