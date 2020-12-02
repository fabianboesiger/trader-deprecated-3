use crate::backends::Exchange;
use crate::model::{Action, Asset, Market, Order, Quantity, Value, ValuedQuantity, MAIN_ASSET};
use crate::wallet::{Position, State, States};
use chrono::{DateTime, Utc};
use rust_decimal::prelude::*;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;

const FEE: f64 = 0.001;

pub struct Trades {
    pool: PgPool,
    pairs: Vec<(Trade, Option<Trade>)>,
    states: HashMap<Asset, State>,
}

impl Trades {
    pub async fn new() -> Trades {
        dotenv::dotenv().ok();

        let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let mut states = HashMap::new();
        for asset in Asset::all() {
            states.insert(
                asset,
                State {
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
                            value: if asset == MAIN_ASSET {
                                Decimal::one()
                            } else {
                                Decimal::zero()
                            },
                            market: Market {
                                base: asset,
                                quote: MAIN_ASSET,
                            },
                        },
                    },
                },
            );
        }

        Trades {
            pool,
            pairs: Vec::new(),
            states,
        }
    }

    pub async fn fetch_all(&mut self) {
        let trades: Vec<Trade> = sqlx::query_as!(
            DbTrade,
            r#"
                SELECT *
                FROM trades
                ORDER BY date_time ASC"#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
        .into_iter()
        .map(Trade::from)
        .collect();

        for trade in trades {
            self.apply_trade(trade);
        }
    }

    pub async fn insert(&mut self, trade: Trade) {
        trade.clone().insert(&self.pool).await;
        self.apply_trade(trade);
    }

    fn apply_trade(&mut self, trade: Trade) {
        // Update states.
        if let Position::Short = trade.position {
            self.states
                .get_mut(&trade.base.asset)
                .unwrap()
                .valued_quantity
                .quantity -= trade.base;
            self.states
                .get_mut(&trade.quote.asset)
                .unwrap()
                .valued_quantity
                .quantity += trade.quote;
        } else {
            self.states
                .get_mut(&trade.base.asset)
                .unwrap()
                .valued_quantity
                .quantity += trade.base;
            self.states
                .get_mut(&trade.quote.asset)
                .unwrap()
                .valued_quantity
                .quantity -= trade.quote;
        }
        self.states.get_mut(&trade.base.asset).unwrap().position = trade.position;

        let i = self
            .pairs
            .iter()
            .enumerate()
            .filter(|(_, (long, short))| (*long).base.asset == trade.base.asset && short.is_none())
            .map(|(i, _)| i)
            .next();

        if let Some(i) = i {
            self.pairs[i].1 = Some(trade);
        } else {
            self.pairs.push((trade, None));
        }
    }

    fn compute_mean_stdev(values: Vec<f32>) -> (f32, f32) {
        let n = values.len() as f32;
        let mut sum = 0.0;
        let mut sum_squared = 0.0;
        for value in values {
            sum += value;
            sum_squared += value.powi(2);
        }
        let mean = sum / n;
        let variance = sum_squared / n - mean.powi(2);
        let stdev = variance.sqrt();
        (mean, stdev)
    }

    pub fn update_value(&mut self, value: Value) {
        // Update value of this asset.
        self.states
            .get_mut(&value.market.base)
            .unwrap()
            .valued_quantity
            .value = value;
        //self.sender.send(Log::Value(value)).unwrap();
    }

    pub fn get_quantity(&self, asset: &Asset) -> Quantity {
        self.states.get(&asset).unwrap().valued_quantity.quantity
    }

    pub fn get_position(&self, asset: &Asset) -> Position {
        self.states.get(&asset).unwrap().position
    }

    pub fn total(&self) -> Quantity {
        let mut sum = Quantity {
            quantity: Decimal::zero(),
            asset: MAIN_ASSET,
        };

        for state in self.states.values() {
            sum += state.valued_quantity.get_value_quantity();
        }

        sum
    }

    pub fn render(&self) -> String {
        const P: f32 = 0.99;
        const Z: f32 = z_table::reverse_lookup(P);

        let r = |x: f32| (x * 100.0).round() / 100.0;
        let rd = |x: Decimal| r(x.to_f32().unwrap());

        let total = rd(self.total().quantity);

        // Average amount of trades per day.
        let start = self
            .pairs
            .iter()
            .map(|(long, short)| {
                let mut vec = vec![long];
                if let Some(short) = short {
                    vec.push(short);
                }
                vec
            })
            .flatten()
            .map(|trade| trade.timestamp)
            .min();

        let (trades_per_day, days) = if let Some(start) = start {
            let duration = Utc::now() - start;
            let days = duration.num_minutes() as f32 / 60.0 / 24.0;

            let completed = self
                .pairs
                .iter()
                .filter(|(_, short)| short.is_some())
                .count();

            (completed as f32 / days, days)
        } else {
            (0.0, 0.0)
        };

        // Profit stats.
        let (wins, losses): (Vec<f32>, Vec<f32>) = self
            .pairs
            .iter()
            .filter(|(_, short)| short.is_some())
            .map(|(long, short)| {
                (short.as_ref().unwrap().quote.quantity - long.quote.quantity) / long.quote.quantity
            })
            .map(|diff| diff.to_f32().unwrap())
            .partition(|diff| *diff >= 0.0);

        let w = wins.len();
        let l = losses.len();

        let (win_mean, win_stdev) = if wins.len() > 0 {
            Self::compute_mean_stdev(wins)
        } else {
            (0.0, 0.0)
        };

        let (loss_mean, loss_stdev) = if losses.len() > 0 {
            Self::compute_mean_stdev(losses)
        } else {
            (0.0, 0.0)
        };

        let (win_ratio, mean, interval) = if w + l > 0 {
            let win_ratio = w as f32 / (w + l) as f32;
            let mean = win_ratio * win_mean + (1.0 - win_ratio) * loss_mean;

            if w > 0 && l == 0 {
                let win_cap = win_mean + Z * win_stdev;
                (win_ratio, mean, win_cap - mean)
            } else if l > 0 && w == 0 {
                let loss_cap = loss_mean - Z * loss_stdev;
                (win_ratio, mean, mean - loss_cap)
            } else {
                let win_cap = win_mean + Z * win_stdev;
                let loss_cap = loss_mean - Z * loss_stdev;
                (win_ratio, mean, (win_cap - mean).max(mean - loss_cap))
            }
        } else {
            (0.5, 0.0, 0.0)
        };

        let (mean, interval) = (mean * trades_per_day, interval);

        //let exp = total * (1.0 + mean / total).powf(365.25) - total;
        //let exp_max = total * (1.0 + (mean + interval) / total).powf(365.25) - total;
        //let exp_dev = exp_max - exp;

        let mut string = format!(
            r#"
            <section>
                <h2>Overview</h2>
                <table>
                    <tr><th>Equity</th><td>{} USDT</td></tr>
                    <tr><th>Estimated Daily Profit</th><td>{} ± {} USDT/day <sup>1</sup></td></tr>
                    <tr><th>Daily Profit Percentage</th><td>{} ± {} %/day <sup>1</sup></td></tr>
                    <tr><th>Online Since</th><td>{} days</td></tr>
                    <tr><th>Average Trades per Day</th><td>{}</td></tr>
                    <tr><th>Win Ratio</th><td>{} %</td></tr>
                    <tr><th>Average Profit per Trade</th><td>{} ± {} % <sup>1</sup></td></tr>
                    <tr><th>Average Loss per Trade</th><td>{} ± {} % <sup>1</sup></td></tr>
                </table>
                <p>
                    <sup>1</sup> Assuming normal distribution of profits, 99% percentile.<br/>
                </p>
            </section>
            <section>
                <h2>Positions</h2>
                <table>
                    <thead>
                        <tr>
                            <th>Market</th>
                            <th>Stop Loss</th>
                            <th>Take Profit</th>
                            <th>Value</th>
                            <th>USDT</th>
                        </tr>
                    </thead>
                    <tbody>"#,
            r(total),
            r(mean * 0.3 * total),
            r(interval * 0.3 * total),
            r(mean * 0.3 * 100.0),
            r(interval * 0.3 * 100.0),
            r(days),
            r(trades_per_day),
            r(win_ratio * 100.0),
            r(win_mean * 100.0),
            r(win_stdev * Z * 100.0),
            r(loss_mean * 100.0),
            r(loss_stdev * Z * 100.0),
        );

        let mut states = self.states.iter().collect::<Vec<(&Asset, &State)>>();

        states.sort_by(|(_, s1), (_, s2)| {
            s2.valued_quantity
                .get_value_quantity()
                .quantity
                .cmp(&s1.valued_quantity.get_value_quantity().quantity)
        });

        for (asset, state) in states.iter().filter(|(_, state)| {
            if let Position::Short = state.position {
                false
            } else {
                true
            }
        }) {
            let (position, stop_loss, take_profit) = if let Position::Long {
                stop_loss,
                take_profit,
                ..
            } = state.position
            {
                (
                    String::from("LONG"),
                    rd(stop_loss.unwrap()).to_string(),
                    rd(take_profit.unwrap()).to_string(),
                )
            } else {
                (String::from("SHORT"), String::new(), String::new())
            };

            string += format!(
                r#"    
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                </tr>"#,
                asset,
                stop_loss,
                take_profit,
                rd(state.valued_quantity.value.value),
                rd(state.valued_quantity.get_value_quantity().quantity),
            )
            .as_str();
        }

        string += format!(
            r#"
                    </tbody>
                </table>
            </section>
        "#
        )
        .as_str();

        string
    }
}

struct DbTrade {
    base_asset: String,
    base_quantity: Decimal,
    quote_asset: String,
    quote_quantity: Decimal,
    date_time: DateTime<Utc>,
    market_value: Decimal,
    is_long: bool,
    take_profit: Option<Decimal>,
    stop_loss: Option<Decimal>,
}

impl DbTrade {
    async fn insert(self, pool: &PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO trades (
                base_asset,
                base_quantity,
                quote_asset,
                quote_quantity,
                date_time,
                market_value,
                is_long,
                take_profit,
                stop_loss
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            self.base_asset,
            self.base_quantity,
            self.quote_asset,
            self.quote_quantity,
            self.date_time,
            self.market_value,
            self.is_long,
            self.take_profit,
            self.stop_loss,
        )
        .execute(pool)
        .await
        .unwrap();
    }
}

impl From<Trade> for DbTrade {
    fn from(trade: Trade) -> DbTrade {
        let (is_long, take_profit, stop_loss) = if let Position::Long {
            take_profit,
            stop_loss,
            ..
        } = trade.position
        {
            (true, take_profit, stop_loss)
        } else {
            (false, None, None)
        };
        DbTrade {
            base_asset: trade.base.asset.to_string(),
            base_quantity: trade.base.quantity,
            quote_asset: trade.quote.asset.to_string(),
            quote_quantity: trade.quote.quantity,
            date_time: trade.timestamp,
            market_value: trade.value.value,
            is_long,
            take_profit,
            stop_loss,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Trade {
    pub base: Quantity,
    pub quote: Quantity,
    pub value: Value,
    pub timestamp: DateTime<Utc>,
    pub position: Position,
}

impl Trade {
    pub async fn insert(self, pool: &PgPool) {
        DbTrade::from(self).insert(pool).await
    }
}

impl From<DbTrade> for Trade {
    fn from(db_trade: DbTrade) -> Trade {
        Trade {
            base: Quantity {
                asset: db_trade.base_asset.clone().into(),
                quantity: db_trade.base_quantity,
            },
            quote: Quantity {
                asset: db_trade.quote_asset.clone().into(),
                quantity: db_trade.quote_quantity,
            },
            value: Value {
                market: Market {
                    base: db_trade.base_asset.into(),
                    quote: db_trade.quote_asset.into(),
                },
                value: db_trade.market_value,
            },
            timestamp: db_trade.date_time,
            position: if db_trade.is_long {
                Position::Long {
                    take_profit: db_trade.take_profit,
                    stop_loss: db_trade.stop_loss,
                }
            } else {
                Position::Short
            },
        }
    }
}
