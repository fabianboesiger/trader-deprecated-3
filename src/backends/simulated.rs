use super::{Exchange, Backend};
use crate::{
    model::{Asset, Market, Order, Quantity, Value, ValuedQuantity, MAIN_ASSET},
};
use async_trait::async_trait;
use openlimits::binance::Binance;
use rust_decimal::prelude::*;
use std::{collections::HashMap, fmt};
use tokio::sync::mpsc::{Receiver, Sender};
use sqlx::PgPool;

/*
pub struct Simulated {
    assets: HashMap<Asset, ValuedQuantity>,
    fee: Decimal,
    investment_fraction: Decimal,
}

impl Simulated {
    pub async fn new(investment_fraction: f64, fee: f64) -> Self {

        dotenv::dotenv().ok();

        let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let db_assets: HashMap<Asset, Decimal> = sqlx::query!("
                WITH
                moves AS (
                    SELECT
                        buy_asset AS asset,
                        buy_quantity AS quantity,
                        date_time
                    FROM trades
                    UNION ALL
                    SELECT
                        sell_asset AS asset,
                        -sell_quantity AS quantity,
                        date_time
                    FROM trades
                    UNION ALL
                    SELECT 'USDT', 1000.0, NULL
                )
                SELECT asset, CAST(SUM(quantity) AS DOUBLE PRECISION) AS quantity
                FROM moves
                WHERE date_time <= NOW()
                OR date_time IS NULL
                GROUP BY asset
            ")
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| (row.asset.unwrap().into(), Decimal::from_f64(row.quantity.unwrap()).unwrap()))
            .collect();


        let mut assets = HashMap::new();
        for asset in Asset::all() {
            assets.insert(
                asset,
                ValuedQuantity {
                    quantity: Quantity {
                        quantity: if let Some(quantity) = db_assets.get(&asset) {
                            *quantity
                        } else 
                        if asset == MAIN_ASSET {
                            Decimal::from_f32(1000.0).unwrap()
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
            );
        }

        Self {
            assets,
            investment_fraction: Decimal::from_f64(investment_fraction).unwrap(),
            fee: Decimal::from_f64(fee).unwrap(),
        }
    }

    fn total(&self) -> Quantity {
        let mut sum = Quantity {
            quantity: Decimal::zero(),
            asset: MAIN_ASSET,
        };

        for valued_quantity in self.assets.values() {
            sum += valued_quantity.get_quantity_value();
        }

        sum
    }
}

#[async_trait]
impl Manager for Simulated {
    async fn run(
        mut self,
        _exchange: &Binance,
        mut receiver: Receiver<Order>,
        mut sender: Sender<Log>,
    ) {
        let mut _trade_count: u64 = 0;
        while let Some(Order {
            side,
            value,
            timestamp,
        }) = receiver.recv().await
        {
            assert_eq!(value.market.quote, MAIN_ASSET);

            let b = value.market.base;
            let q = value.market.quote;

            self.assets.get_mut(&b).unwrap().value = value;

            sender.send(Log::Value(value)).await.unwrap();

            let sum = self.total();

            if let Some(side) = side {
                match side {
                    Side::Buy => {
                        /*
                        let quantity = min(
                            self.assets.get(&q).unwrap().quantity,
                            sum / self.investment_fraction,
                        ) / value;
                        */
                        let quantity = sum / self.investment_fraction;
                        if quantity <= self.assets.get(&q).unwrap().quantity {
                            let buy = quantity / value * (Decimal::one() - self.fee);
                            let sell = quantity;

                            _trade_count += 1;
                            sender
                                .send(Log::Trade {
                                    buy,
                                    sell,
                                    timestamp,
                                })
                                .await
                                .unwrap();

                            self.assets.get_mut(&b).unwrap().quantity += buy;
                            self.assets.get_mut(&q).unwrap().quantity -= sell;
                        }
                    }
                    Side::Sell => {
                        let quantity = self.assets.get(&b).unwrap().quantity;
                        if !quantity.is_zero() {
                            let buy = quantity * value * (Decimal::one() - self.fee);
                            let sell = quantity;

                            _trade_count += 1;
                            sender
                                .send(Log::Trade {
                                    buy,
                                    sell,
                                    timestamp,
                                })
                                .await
                                .unwrap();

                            self.assets.get_mut(&q).unwrap().quantity += buy;
                            self.assets.get_mut(&b).unwrap().quantity -= sell;
                        }
                    }
                }
                //println!("{}TRADE COUNT: {}", self, trade_count);
            }
        }
    }
}

impl fmt::Display for Simulated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for valued_quantity in self.assets.values() {
            if !valued_quantity.get_quantity_value().quantity.is_zero() {
                writeln!(f, "{}", valued_quantity)?;
            }
        }

        writeln!(f, "TOTAL: {}", self.total())
    }
}
*/

pub struct Simulated {

}

#[async_trait]
impl Backend for Simulated {
    fn new() -> Self {
        Simulated {}
    }

    async fn buy(self, exchange: &Binance) {

    }

    async fn sell(self, exchange: &Binance) {

    }
}