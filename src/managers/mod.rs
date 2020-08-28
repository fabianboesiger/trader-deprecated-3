mod simulated;

pub use simulated::Simulated;

use crate::model::Order;
use async_trait::async_trait;
use openlimits::binance::Binance;
use tokio::sync::mpsc::Receiver;

#[async_trait]
pub trait Manager {
    async fn run(&mut self, exchange: &Binance, reciever: Receiver<Order>);
}
