mod simulated;

pub use simulated::Simulated;

use crate::{loggers::Log, model::Order};
use async_trait::async_trait;
use openlimits::binance::Binance;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[async_trait]
pub trait Manager {
    async fn run(
        self,
        exchange: &Binance,
        receiver: UnboundedReceiver<Order>,
        sender: UnboundedSender<Log>,
    );
}
