mod simulated;

pub use simulated::Simulated;

use crate::{model::Order, model::Quantity};
use async_trait::async_trait;
use openlimits::binance::Binance;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Exchange {
    pub buy: Quantity,
    pub sell: Quantity,
}

#[async_trait]
pub trait Backend: Send + 'static {
    fn new() -> Self;
    async fn buy(self, exchange: &Binance);
    async fn sell(self, exchange: &Binance);
}
