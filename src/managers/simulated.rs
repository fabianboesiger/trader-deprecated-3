use super::Manager;
use crate::model::{Asset, Market, Order, Quantity, Side, Value, ValuedQuantity, MAIN_ASSET};
use async_trait::async_trait;
use openlimits::binance::Binance;
use rust_decimal::prelude::*;
use std::{cmp::min, collections::HashMap, fmt};
use tokio::sync::mpsc::Receiver;

pub struct Simulated {
    assets: HashMap<Asset, ValuedQuantity>,
    fee: Decimal,
    investment_fraction: Decimal,
}

impl Simulated {
    pub fn new(investment_fraction: f64, fee: f64) -> Self {
        let mut assets = HashMap::new();
        for asset in Asset::all() {
            assets.insert(
                asset,
                ValuedQuantity {
                    quantity: Quantity {
                        quantity: if asset == MAIN_ASSET {
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
    async fn run(&mut self, _exchange: &Binance, mut reciever: Receiver<Order>) {
        while let Some(Order { side, value }) = reciever.recv().await {
            assert_eq!(value.market.quote, MAIN_ASSET);

            let b = value.market.base;
            let q = value.market.quote;

            self.assets.get_mut(&b).unwrap().value = value;

            let sum = self.total();

            if let Some(side) = side {
                match side {
                    Side::Buy => {
                        let quantity = min(
                            self.assets.get(&q).unwrap().quantity,
                            sum / self.investment_fraction,
                        ) / value;

                        self.assets.get_mut(&b).unwrap().quantity +=
                            quantity * (Decimal::one() - self.fee);
                        self.assets.get_mut(&q).unwrap().quantity -= quantity * value;
                    }
                    Side::Sell => {
                        let quantity = self.assets.get(&b).unwrap().quantity;

                        self.assets.get_mut(&b).unwrap().quantity -= quantity;
                        self.assets.get_mut(&q).unwrap().quantity +=
                            quantity * value * (Decimal::one() - self.fee);
                    }
                }
                println!("{}", self);
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
