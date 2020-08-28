mod bollinger;

pub use bollinger::*;

use crate::model::{Market, Order};
use async_trait::async_trait;
use openlimits::binance::Binance;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait Trader: Clone + Send + 'static {
    async fn run(&mut self, exchange: &Binance, market: Market, sender: Sender<Order>);
}
