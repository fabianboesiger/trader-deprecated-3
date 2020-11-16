use crate::model::{Quantity, Value};
use chrono::{DateTime, Utc};
use futures::{Stream, StreamExt};
use rust_decimal::prelude::*;
use serde::Serialize;
use sqlx::{PgPool, Row};

#[derive(Debug, Serialize)]
pub enum Log {
    Trade {
        buy: Quantity,
        sell: Quantity,
        timestamp: DateTime<Utc>,
    },
    Value(Value),
}

impl Log {
    /*
    pub async fn select_assets_at(pool: &PgPool, timestamp: DateTime<Utc>) -> Vec<Message> {
        sqlx::query_as!(
            Message::Asset,
            r#"
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
                )
                SELECT asset, SUM(quantity) AS quantity
                FROM moves
                WHERE date_time <= $1
                GROUP BY asset
            "#,
            timestamp
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }
    */
    pub async fn insert(&self, pool: &PgPool) {
        match self {
            Log::Trade {
                buy,
                sell,
                timestamp,
            } => {
                sqlx::query!(
                    r#"
                        INSERT INTO trades (
                            buy_asset,
                            buy_quantity,
                            sell_asset,
                            sell_quantity,
                            date_time
                        )
                        VALUES ($1, $2, $3, $4, $5)
                    "#,
                    String::from(buy.asset),
                    buy.quantity,
                    String::from(sell.asset),
                    sell.quantity,
                    timestamp
                )
                .execute(pool)
                .await
                .unwrap();
            }
            _ => {}
        }
    }

    pub fn select_all_trades(pool: &PgPool) -> impl Stream<Item = Result<Log, sqlx::Error>> {
        // TODO: Fix casting as soon as https://github.com/launchbadge/sqlx/issues/666 is fixed.
        sqlx::query(
            r#"
                SELECT
                    buy_asset,
                    CAST(buy_quantity AS DOUBLE PRECISION),
                    sell_asset,
                    CAST(sell_quantity AS DOUBLE PRECISION),
                    date_time
                FROM trades
                ORDER BY date_time ASC
            "#,
        )
        .fetch(pool)
        .map(|row| {
            row.map(|row| {
                Log::Trade {
                    buy: Quantity {
                        asset: row.try_get::<String, _>("buy_asset").unwrap().into(),
                        quantity: Decimal::from_f64(row.try_get("buy_quantity").unwrap()).unwrap(),
                    },
                    sell: Quantity {
                        asset: row.try_get::<String, _>("sell_asset").unwrap().into(),
                        quantity: Decimal::from_f64(row.try_get("sell_quantity").unwrap()).unwrap(),
                    },
                    timestamp: row.try_get("date_time").unwrap(),
                }
                /*
                println!("{} {}", row.try_get::<f64, _>("casted_buy_quantity").unwrap(), row.try_get::<f64, _>("casted_sell_quantity").unwrap());
                Log::Trade {
                    buy: Quantity {
                        asset: row.try_get::<String, _>("buy_asset").unwrap().into(),
                        quantity: row.try_get("buy_quantity").unwrap(),
                    },
                    sell: Quantity {
                        asset: row.try_get::<String, _>("sell_asset").unwrap().into(),
                        quantity: row.try_get("sell_quantity").unwrap(),
                    },
                    timestamp: row.try_get("date_time").unwrap()
                }
                */
            })
        })
    }
}
