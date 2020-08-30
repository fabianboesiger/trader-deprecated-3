mod bollinger;

pub use bollinger::*;

use crate::model::{Market, Order};
use async_trait::async_trait;
use openlimits::binance::Binance;
use tokio::sync::mpsc::UnboundedSender;

#[async_trait]
pub trait Trader: Clone + Send + 'static {
    async fn run(self, exchange: &Binance, market: Market, sender: UnboundedSender<Order>);
}
