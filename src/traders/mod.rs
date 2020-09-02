mod custom;

pub use custom::*;

use crate::model::{Market, Order};
use async_trait::async_trait;
use openlimits::binance::Binance;
use tokio::sync::{mpsc::Sender, Barrier};
use std::sync::Arc;

#[async_trait]
pub trait Trader: Clone + Send + 'static {
    async fn run(self, exchange: &Binance, market: Market, barrier: Arc<Barrier>, sender: Sender<Order>);
}
