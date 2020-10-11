use crate::model::{Asset, Market, MAIN_ASSET};
use crate::{loggers::Logger, managers::Manager, traders::Trader};
use openlimits::binance::Binance;
use tokio::sync::{mpsc::channel, Barrier};
use tokio::task;
use std::sync::Arc;

pub struct Environment<T: Trader, M: Manager, L: Logger> {
    pub trader: T,
    pub manager: M,
    pub logger: L,
}

impl<T: Trader, M: Manager, L: Logger> Environment<T, M, L> {
    pub async fn trade(self) {
        let exchange: &'static Binance = Box::leak(Box::new(Binance::new(false).await));

        let (order_sender, order_reciever) = channel(1);
        let (message_sender, message_reciever) = channel(16);

        let tradable = Asset::all()
            .into_iter()
            .filter(|asset| *asset != MAIN_ASSET)
            .collect::<Vec<Asset>>();

        let barrier = Arc::new(Barrier::new(tradable.len()));

        for asset in tradable
        {
            let trader = self.trader.clone();
            let barrier = barrier.clone();
            let sender = order_sender.clone();
            let market = Market {
                base: asset,
                quote: MAIN_ASSET,
            };

            task::spawn(async move {
                trader.run(exchange, market, barrier, sender).await;
            });
        }

        tokio::join! {
            self.manager.run(exchange, order_reciever, message_sender),
            self.logger.run(message_reciever)
        };
    }
}
